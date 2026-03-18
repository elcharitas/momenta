use crate::nodes::Node;
use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::{
    any::Any,
    cmp::Ordering,
    marker::PhantomData,
    ops::{AddAssign, DivAssign, MulAssign, Not, SubAssign},
    sync::atomic::{AtomicBool, Ordering as AtomicOrdering},
};
use spin::Mutex;

type ScopeCallback = Arc<dyn Fn(&Node) + Send + Sync>;
type ScopeEffect = Box<dyn Fn() + Send>;
type ScopeEffectCleanup = Box<dyn FnOnce() + Send>;

/// Per-scope data stored in a Vec for O(1) access.
struct ScopeData {
    signal_count: usize,
    effect_count: usize,
    signals: Vec<StoredValue>,
    effects: Vec<Option<ScopeEffect>>,
    effect_cleanups: Vec<Option<ScopeEffectCleanup>>,
    effect_executing: Vec<bool>,
    function: Option<Box<dyn FnMut() -> Node + Send>>,
    callback: Option<ScopeCallback>,
}

impl ScopeData {
    fn new() -> Self {
        Self {
            signal_count: 0,
            effect_count: 0,
            signals: Vec::new(),
            effects: Vec::new(),
            effect_cleanups: Vec::new(),
            effect_executing: Vec::new(),
            function: None,
            callback: None,
        }
    }

    /// Reset counters and clear Vecs, keeping heap capacity for reuse.
    fn clear(&mut self) {
        self.signal_count = 0;
        self.effect_count = 0;
        self.signals.clear();
        self.effects.clear();
        self.effect_cleanups.clear();
        self.effect_executing.clear();
        self.function = None;
        self.callback = None;
    }
}

/// All reactive runtime state consolidated behind a single mutex.
/// Per-scope data is stored in Vec<Option<ScopeData>> for O(1) indexed access,
/// replacing BTreeMap<(scope, idx), T> lookups.
pub(crate) struct RuntimeState {
    current_scope: Option<usize>,
    rendering_scope: usize,
    next_scope_id: usize,
    batch_updates: bool,
    /// True when inside run_scope_transient — skips cross-scope dep tracking.
    transient_scope: bool,
    /// Per-scope storage: indexed by scope_id - 1. O(1) access.
    scopes: Vec<Option<ScopeData>>,
    /// Signals that have changed (for pending render processing).
    scope_signal_changes: Vec<(usize, usize)>,
    /// Count of effects currently executing (for re-entrancy guard in set).
    executing_effects_count: usize,
    /// Cross-scope: which signals does a scope depend on.
    scope_dependencies: BTreeMap<usize, Vec<(usize, usize)>>,
    /// Cross-scope: which scopes depend on a signal.
    signal_dependencies: BTreeMap<(usize, usize), Vec<usize>>,
    /// Scopes that need re-rendering.
    pending_scope_renders: Vec<usize>,
    #[allow(dead_code)]
    memo_cache: BTreeMap<String, Box<dyn SignalValue>>,
}

impl RuntimeState {
    const fn new() -> Self {
        Self {
            current_scope: None,
            rendering_scope: 0,
            next_scope_id: 1,
            batch_updates: false,
            transient_scope: false,
            scopes: Vec::new(),
            scope_signal_changes: Vec::new(),
            executing_effects_count: 0,
            scope_dependencies: BTreeMap::new(),
            signal_dependencies: BTreeMap::new(),
            pending_scope_renders: Vec::new(),
            memo_cache: BTreeMap::new(),
        }
    }

    #[inline]
    fn scope(&self, id: usize) -> Option<&ScopeData> {
        self.scopes.get(id - 1).and_then(|s| s.as_ref())
    }

    #[inline]
    fn scope_mut(&mut self, id: usize) -> Option<&mut ScopeData> {
        self.scopes.get_mut(id - 1).and_then(|s| s.as_mut())
    }

    #[inline]
    fn ensure_scope(&mut self, id: usize) -> &mut ScopeData {
        if id > self.scopes.len() {
            self.scopes.resize_with(id, || None);
        }
        self.scopes[id - 1].get_or_insert_with(ScopeData::new)
    }

    #[inline]
    fn get_signal(&self, id: (usize, usize)) -> Option<&StoredValue> {
        self.scope(id.0).and_then(|s| s.signals.get(id.1 - 1))
    }

    #[inline]
    fn get_signal_mut(&mut self, id: (usize, usize)) -> Option<&mut StoredValue> {
        self.scope_mut(id.0)
            .and_then(|s| s.signals.get_mut(id.1 - 1))
    }

    #[inline]
    fn has_executing_effects(&self) -> bool {
        self.executing_effects_count > 0
    }

    #[inline]
    fn push_pending_render(&mut self, scope_id: usize) {
        if !self.pending_scope_renders.contains(&scope_id) {
            self.pending_scope_renders.push(scope_id);
        }
    }
}

pub(crate) static RUNTIME: Mutex<RuntimeState> = Mutex::new(RuntimeState::new());
/// Serializes SSR renders so the global runtime can be safely reset per request.
static ISOLATED_RUNTIME_LOCK: Mutex<()> = Mutex::new(());
/// Fast atomic flag for has_current_scope() — avoids locking RUNTIME just to check.
static IN_SCOPE: AtomicBool = AtomicBool::new(false);

pub trait SignalValue: Send {
    fn as_any(&self) -> Option<&dyn Any>;
    fn as_any_mut(&mut self) -> Option<&mut dyn Any>;
}

macro_rules! impl_signal_value {
    ($($t:ty),*) => {
        $(
            impl SignalValue for $t {
                #[inline]
                fn as_any(&self) -> Option<&dyn Any> {
                    Some(self)
                }
                #[inline]
                fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
                    Some(self)
                }
            }
        )*
    };
}

impl_signal_value!(
    String,
    &'static str,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    bool,
    char,
    ()
);

impl<T: SignalValue + 'static> SignalValue for alloc::vec::Vec<T> {
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }
}

impl<T: SignalValue + 'static> SignalValue for Option<T> {
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }
}

#[derive(Debug)]
pub struct Signal<T> {
    id: (usize, usize),
    _marker: PhantomData<T>,
}

impl<T> Copy for Signal<T> {}

impl<T: SignalValue + Not<Output = bool> + Clone + 'static> Not for Signal<T> {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.get()
    }
}

impl<T: SignalValue + Clone + 'static> Signal<T> {
    pub fn then<R, F: FnOnce() -> R>(self, f: F) -> Option<R>
    where
        Signal<T>: Not<Output = bool>,
    {
        if !!self { Some(f()) } else { None }
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

macro_rules! impl_assign_ops {
    ($($trait:ident, $op:ident, $method:ident, $assign_method:ident),*) => {
        $(
            impl<T: SignalValue + PartialEq + Clone + core::ops::$trait<Output = T> + 'static> $method<T> for Signal<T> {
                fn $assign_method(&mut self, rhs: T) {
                    self.set(self.get().$op(rhs));
                }
            }
        )*
    };
}

impl_assign_ops!(
    Add, add, AddAssign, add_assign, Sub, sub, SubAssign, sub_assign, Mul, mul, MulAssign,
    mul_assign, Div, div, DivAssign, div_assign
);

impl<T: SignalValue + PartialEq + 'static> PartialEq<T> for Signal<T> {
    fn eq(&self, other: &T) -> bool {
        self.with(|val| val == other).unwrap_or(false)
    }
}

impl<T: SignalValue + PartialOrd + Clone + 'static> PartialOrd<T> for Signal<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.with(|val| val.partial_cmp(other)).unwrap_or(None)
    }
}

impl<T: SignalValue + PartialOrd + Clone + 'static> PartialOrd<Signal<T>> for Signal<T> {
    fn partial_cmp(&self, other: &Signal<T>) -> Option<Ordering> {
        other
            .with(|other_val| {
                self.with(|self_val| self_val.partial_cmp(other_val))
                    .unwrap_or(None)
            })
            .unwrap_or(None)
    }
}

impl<T: SignalValue + PartialEq + Clone + 'static> PartialEq<Signal<T>> for Signal<T> {
    fn eq(&self, other: &Signal<T>) -> bool {
        other
            .with(|other_val| self.with(|self_val| self_val == other_val).unwrap_or(false))
            .unwrap_or(false)
    }
}

// Boolean convenience methods
impl Signal<bool> {
    pub fn toggle(&self) {
        self.set(!self.get());
    }

    pub fn turn_on(&self) {
        self.set(true);
    }

    pub fn turn_off(&self) {
        self.set(false);
    }
}

impl<T: SignalValue + PartialEq + Clone + 'static> Signal<Vec<T>> {
    pub fn push(&self, item: T) {
        let mut vec = self.get();
        vec.push(item);
        self.set(vec);
    }

    pub fn pop(&self) -> Option<T> {
        let mut vec = self.get();
        let result = vec.pop();
        self.set(vec);
        result
    }

    pub fn insert(&self, index: usize, item: T) {
        let mut vec = self.get();
        vec.insert(index, item);
        self.set(vec);
    }

    pub fn remove(&self, index: usize) -> T {
        let mut vec = self.get();
        let result = vec.remove(index);
        self.set(vec);
        result
    }

    pub fn retain<F>(&self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut vec = self.get();
        vec.retain(f);
        self.set(vec);
    }

    pub fn len(&self) -> usize {
        self.with(|v| v.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.with(|v| v.is_empty()).unwrap_or(true)
    }

    pub fn clear(&self) {
        self.set(Vec::new());
    }

    pub fn get_at(&self, index: usize) -> Option<T> {
        self.with(|v| v.get(index).cloned()).unwrap_or(None)
    }

    pub fn update_at(&self, index: usize, item: T) {
        let mut vec = self.get();
        if index < vec.len() {
            vec[index] = item;
            self.set(vec);
        }
    }
}

impl<T: SignalValue + PartialEq + Clone + Ord + 'static> Signal<Vec<T>> {
    pub fn sort(&self) {
        let mut vec = self.get();
        vec.sort();
        self.set(vec);
    }

    /// Sort by a key function
    pub fn sort_by_key<K, F>(&self, f: F)
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        let mut vec = self.get();
        vec.sort_by_key(f);
        self.set(vec);
    }

    pub fn reverse(&self) {
        let mut vec = self.get();
        vec.reverse();
        self.set(vec);
    }
}

impl<T: SignalValue + Clone + 'static> Signal<Vec<T>> {
    pub fn iter(&self) -> impl Iterator<Item = T> {
        self.get().into_iter()
    }

    pub fn map<R, F>(&self, f: F) -> Vec<R>
    where
        F: FnMut(T) -> R,
    {
        self.iter().map(f).collect()
    }

    pub fn filter<F>(&self, f: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        self.iter().filter(f).collect()
    }
}

impl<T: SignalValue + 'static> Signal<T> {
    /// Map the signal value to a new computed signal
    ///
    /// # Example
    /// ```rust,no_run
    /// use momenta_core::signals::create_signal;
    ///
    /// let count = create_signal(5);
    /// let doubled = count.derive(|&x| x * 2);
    /// assert_eq!(doubled.get(), 10);
    /// ```
    #[cfg(any(feature = "computed", feature = "full-reactivity"))]
    pub fn derive<U, F>(&self, f: F) -> Signal<U>
    where
        T: Clone + PartialEq,
        U: SignalValue + PartialEq + Clone + 'static,
        F: Fn(&T) -> U + Send + 'static,
    {
        let self_clone = *self;
        create_computed(move || self_clone.with(|val| f(val)).unwrap())
    }

    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        let mut rt = RUNTIME.lock();

        // Skip dependency tracking in transient scopes
        if !rt.transient_scope {
            if let Some(current_scope) = rt.current_scope {
                let deps = rt.signal_dependencies.entry(self.id).or_default();
                if !deps.contains(&current_scope) {
                    deps.push(current_scope);
                }
                let scope_deps = rt.scope_dependencies.entry(current_scope).or_default();
                if !scope_deps.contains(&self.id) {
                    scope_deps.push(self.id);
                }
            }
        }

        rt.get_signal(self.id)
            .and_then(|stored| {
                stored
                    .value
                    .as_any()
                    .and_then(|any| any.downcast_ref::<T>())
            })
            .map(f)
    }

    #[inline]
    pub fn set(&self, value: T)
    where
        T: PartialEq,
    {
        let should_process = {
            let mut rt = RUNTIME.lock();
            let stored = match rt.get_signal_mut(self.id) {
                Some(s) => s,
                None => return,
            };

            // Try in-place update: avoids Box reallocation for same-type values
            if let Some(current) = stored
                .value
                .as_any_mut()
                .and_then(|any| any.downcast_mut::<T>())
            {
                if *current == value {
                    return;
                }
                *current = value;
            } else {
                return;
            }

            rt.scope_signal_changes.push(self.id);

            let should_process = !rt.batch_updates
                && rt.current_scope.is_none()
                && !rt.has_executing_effects()
                && rt.rendering_scope == 0;

            if should_process {
                let scopes_to_queue: Vec<_> = rt
                    .signal_dependencies
                    .get(&self.id)
                    .map(|deps| deps.iter().copied().collect())
                    .unwrap_or_default();
                for scope_id in scopes_to_queue {
                    rt.push_pending_render(scope_id);
                }
            }
            should_process
        };

        if should_process {
            process_pending_renders();
        }
    }

    #[inline]
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.with(|val| val.clone()).unwrap()
    }
}

struct StoredValue {
    value: Box<dyn SignalValue>,
}

/// Run a function with batched updates enabled
/// All signal updates within the function will be batched and applied at the end
///
/// # Example
/// ```ignore
/// use momenta_core::signals::{create_signal, batch};
///
/// let count = create_signal(0);
/// let text = create_signal("hello");
///
/// batch(|| {
///     count.set(1);  // Won't trigger re-render yet
///     count.set(2);  // Won't trigger re-render yet
///     text.set("world");  // Won't trigger re-render yet
/// }); // All changes applied and single re-render triggered here
/// ```
pub fn batch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    RUNTIME.lock().batch_updates = true;

    let result = f();

    RUNTIME.lock().batch_updates = false;

    // Process all pending renders
    process_pending_renders();

    result
}

/// Create a memoized computation that caches results based on dependencies
///
/// The computation is only re-run when dependencies change. Results are cached.
///
/// # Example
/// ```rust,no_run
/// use momenta_core::signals::{create_signal, create_memo};
///
/// let count = create_signal(5);
/// let expensive = create_memo(
///     "double_count",
///     move || {
///         // Expensive computation
///         count.get() * 2
///     }
/// );
/// ```
#[cfg(any(feature = "memoization", feature = "full-reactivity"))]
pub fn create_memo<T, F>(key: &str, computation: F) -> Signal<T>
where
    T: SignalValue + PartialEq + Clone + 'static,
    F: Fn() -> T + Send + 'static,
{
    let (scope_id, signal_id) = {
        let mut rt = RUNTIME.lock();
        let scope_id = rt
            .current_scope
            .expect("Signals can only be created within a scope context");
        let scope = rt.ensure_scope(scope_id);
        scope.signal_count += 1;
        (scope_id, scope.signal_count)
    };
    let signal = Signal {
        id: (scope_id, signal_id),
        _marker: PhantomData,
    };

    let cache_key = alloc::format!("{}-{}", key, scope_id);

    {
        let mut rt = RUNTIME.lock();
        let cached_val = rt.memo_cache.get(&cache_key).and_then(|cached| {
            cached
                .as_any()
                .and_then(|any| any.downcast_ref::<T>())
                .cloned()
        });
        if let Some(cached_val) = cached_val {
            let scope = rt.ensure_scope(scope_id);
            let idx = signal_id - 1;
            if idx >= scope.signals.len() {
                scope.signals.push(StoredValue {
                    value: Box::new(cached_val),
                });
            } else {
                scope.signals[idx] = StoredValue {
                    value: Box::new(cached_val),
                };
            }
            return signal;
        }
    }

    let initial_value = computation();
    {
        let mut rt = RUNTIME.lock();
        rt.memo_cache
            .insert(cache_key.clone(), Box::new(initial_value.clone()));
        let scope = rt.ensure_scope(scope_id);
        let idx = signal_id - 1;
        if idx >= scope.signals.len() {
            scope.signals.push(StoredValue {
                value: Box::new(initial_value),
            });
        } else {
            scope.signals[idx] = StoredValue {
                value: Box::new(initial_value),
            };
        }
    }

    let signal_clone = signal;
    create_effect(move || {
        let new_value = computation();
        {
            let mut rt = RUNTIME.lock();
            rt.memo_cache
                .insert(cache_key.clone(), Box::new(new_value.clone()));
        }
        signal_clone.set(new_value);
    });

    signal
}

pub enum SignalInit<T> {
    Value(T),
    InitFn(Box<dyn Fn() -> T + Send + 'static>),
}

impl<T: SignalValue> From<T> for SignalInit<T> {
    fn from(value: T) -> Self {
        SignalInit::Value(value)
    }
}

#[derive(Debug)]
pub enum SignalCreationError {
    OutsideScope,
}

impl core::fmt::Display for SignalCreationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Signals can only be created within a scope context")
    }
}

/// Create new signal within current scope
#[inline]
pub fn create_signal<T, I>(init: I) -> Signal<T>
where
    T: SignalValue + PartialEq + 'static,
    I: Into<SignalInit<T>>,
{
    let mut rt = RUNTIME.lock();
    let scope_id = rt
        .current_scope
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap();
    let scope = rt.ensure_scope(scope_id);
    scope.signal_count += 1;
    let signal_id = scope.signal_count;
    let idx = signal_id - 1;
    let signal = Signal {
        id: (scope_id, signal_id),
        _marker: PhantomData,
    };

    // If signal already exists (re-render), reuse it
    if idx < scope.signals.len() {
        return signal;
    }

    let initial_value = match init.into() {
        SignalInit::Value(v) => v,
        SignalInit::InitFn(f) => {
            drop(rt);
            let val = f();
            rt = RUNTIME.lock();
            val
        }
    };
    let scope = rt.ensure_scope(scope_id);
    scope.signals.push(StoredValue {
        value: Box::new(initial_value),
    });

    signal
}

/// Create a computed/derived signal that automatically updates based on dependencies
///
/// The computation function is re-run whenever any signal it reads changes.
///
/// # Example
/// ```rust,no_run
/// use momenta_core::signals::{create_signal, create_computed};
///
/// let count = create_signal(0);
/// let doubled = create_computed(move || count.get() * 2);
///
/// assert_eq!(doubled.get(), 0);
/// count.set(5);
/// assert_eq!(doubled.get(), 10);
/// ```
#[cfg(any(feature = "computed", feature = "full-reactivity"))]
pub fn create_computed<T, F>(computation: F) -> Signal<T>
where
    T: SignalValue + PartialEq + Clone + 'static,
    F: Fn() -> T + Send + 'static,
{
    let (scope_id, signal_id) = {
        let mut rt = RUNTIME.lock();
        let scope_id = rt
            .current_scope
            .expect("Signals can only be created within a scope context");
        let scope = rt.ensure_scope(scope_id);
        scope.signal_count += 1;
        (scope_id, scope.signal_count)
    };
    let signal = Signal {
        id: (scope_id, signal_id),
        _marker: PhantomData,
    };

    let initial_value = computation();
    {
        let mut rt = RUNTIME.lock();
        let scope = rt.ensure_scope(scope_id);
        scope.signals.push(StoredValue {
            value: Box::new(initial_value),
        });
    }

    let signal_clone = signal;
    create_effect(move || {
        let new_value = computation();
        signal_clone.set(new_value);
    });

    signal
}

/// Create effect that runs when dependencies change
///
/// # Example
/// ```ignore
/// use momenta_core::signals::{create_signal, create_effect};
///
/// let count = create_signal(0);
/// create_effect(move || {
///     println!("Count changed to: {}", count.get());
/// });
/// ```
#[inline]
pub fn create_effect(effect: impl Fn() + Send + 'static) {
    let mut rt = RUNTIME.lock();
    let scope_id = rt
        .current_scope
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap();
    let is_transient = rt.transient_scope;
    let scope = rt.ensure_scope(scope_id);
    scope.effect_count += 1;
    scope.effects.push(Some(Box::new(effect)));
    if !is_transient {
        scope.effect_cleanups.push(None);
        scope.effect_executing.push(false);
    }
}

/// Create effect with cleanup function
///
/// The cleanup function is called when the scope is destroyed or before the effect re-runs.
///
/// # Example
/// ```ignore
/// use momenta_core::signals::{create_signal, create_effect_with_cleanup};
///
/// let count = create_signal(0);
/// create_effect_with_cleanup(
///     move || {
///         println!("Setting up for count: {}", count.get());
///         // Return cleanup function
///         move || {
///             println!("Cleaning up");
///         }
///     }
/// );
/// ```
pub fn create_effect_with_cleanup<C>(effect: impl Fn() -> C + Send + 'static)
where
    C: FnOnce() + Send + 'static,
{
    let (scope_id, effect_idx) = {
        let mut rt = RUNTIME.lock();
        let scope_id = rt
            .current_scope
            .ok_or(SignalCreationError::OutsideScope)
            .unwrap();
        let scope = rt.ensure_scope(scope_id);
        scope.effect_count += 1;
        let idx = scope.effect_count - 1;
        (scope_id, idx)
    };

    let mut rt = RUNTIME.lock();
    let scope = rt.ensure_scope(scope_id);
    scope.effects.push(Some(Box::new(move || {
        {
            let mut rt = RUNTIME.lock();
            if let Some(scope) = rt.scope_mut(scope_id) {
                if let Some(cleanup) = scope
                    .effect_cleanups
                    .get_mut(effect_idx)
                    .and_then(|c| c.take())
                {
                    drop(rt);
                    cleanup();
                }
            }
        }

        let new_cleanup = effect();
        {
            let mut rt = RUNTIME.lock();
            if let Some(scope) = rt.scope_mut(scope_id) {
                if effect_idx < scope.effect_cleanups.len() {
                    scope.effect_cleanups[effect_idx] = Some(Box::new(new_cleanup));
                }
            }
        }
    })));
    let scope = rt.ensure_scope(scope_id);
    scope.effect_cleanups.push(None);
    scope.effect_executing.push(false);
}

pub fn run_scope(
    scope_fn: impl FnMut() -> Node + Send + 'static,
    callback: impl Fn(&Node) + Send + Sync + 'static,
) -> Node {
    let scope_id = {
        let mut rt = RUNTIME.lock();
        let id = rt.next_scope_id;
        rt.next_scope_id += 1;
        let scope = rt.ensure_scope(id);
        scope.function = Some(Box::new(scope_fn));
        scope.callback = Some(Arc::new(callback));
        id
    };

    render_scope(scope_id)
}

/// Run a scope once and dispose all reactive state created for it afterwards.
///
/// Optimized path for transient scopes: avoids storing the scope function and
/// callback in the runtime since they won't be needed for re-renders, and
/// skips re-entrancy checks and signal change detection.
#[inline]
pub fn run_scope_transient(
    scope_fn: impl FnOnce() -> Node + Send + 'static,
    callback: impl Fn(&Node) + Send + Sync + 'static,
) -> Node {
    let (scope_id, previous_scope, was_transient) = {
        let mut rt = RUNTIME.lock();
        let id = rt.next_scope_id;
        rt.next_scope_id += 1;
        rt.ensure_scope(id);
        let prev = rt.current_scope;
        let was_transient = rt.transient_scope;
        rt.current_scope = Some(id);
        rt.transient_scope = true;
        IN_SCOPE.store(true, AtomicOrdering::Relaxed);
        (id, prev, was_transient)
    };

    let node = scope_fn();

    callback(&node);

    // Single-lock fast path when there are no effects (common case).
    let effects: Vec<ScopeEffect> = {
        let mut rt = RUNTIME.lock();
        let has_effects = rt
            .scope(scope_id)
            .map(|s| s.effect_count > 0)
            .unwrap_or(false);
        if has_effects {
            rt.executing_effects_count += 1;
            let scope = rt.scope_mut(scope_id).unwrap();
            scope.effects.iter_mut().filter_map(|e| e.take()).collect()
        } else {
            // No effects: do full teardown in this single lock acquisition
            rt.current_scope = previous_scope;
            rt.transient_scope = was_transient;
            IN_SCOPE.store(previous_scope.is_some(), AtomicOrdering::Relaxed);
            if let Some(scope) = rt.scope_mut(scope_id) {
                scope.clear();
            }
            rt.scope_signal_changes.clear();
            rt.next_scope_id = scope_id;
            return node;
        }
    };

    for effect in &effects {
        effect();
    }

    {
        let mut rt = RUNTIME.lock();
        rt.executing_effects_count -= 1;
        rt.current_scope = previous_scope;
        rt.transient_scope = was_transient;
        IN_SCOPE.store(previous_scope.is_some(), AtomicOrdering::Relaxed);
        if let Some(scope) = rt.scope_mut(scope_id) {
            scope.clear();
        }

        rt.scope_signal_changes.clear();

        rt.next_scope_id = scope_id;
    }

    node
}

/// Runs a closure with a freshly reset runtime and clears all runtime state again afterwards.
pub fn with_isolated_runtime<R>(f: impl FnOnce() -> R) -> R {
    let _guard = ISOLATED_RUNTIME_LOCK.lock();
    *RUNTIME.lock() = RuntimeState::new();
    IN_SCOPE.store(false, AtomicOrdering::Relaxed);
    let result = f();
    *RUNTIME.lock() = RuntimeState::new();
    IN_SCOPE.store(false, AtomicOrdering::Relaxed);
    result
}

#[allow(dead_code)]
pub(crate) fn get_current_scope() -> Option<usize> {
    RUNTIME.lock().current_scope
}

/// Returns true when code is currently executing inside a reactive scope.
#[inline]
pub fn has_current_scope() -> bool {
    IN_SCOPE.load(AtomicOrdering::Relaxed)
}

#[allow(dead_code)]
fn clear_scope_effects(scope_id: usize) {
    let cleanups_to_call = {
        let mut rt = RUNTIME.lock();
        let mut cleanups = Vec::new();
        if let Some(scope) = rt.scope_mut(scope_id) {
            for eff in scope.effects.iter_mut() {
                *eff = None;
            }
            for cleanup in scope.effect_cleanups.iter_mut() {
                if let Some(c) = cleanup.take() {
                    cleanups.push(c);
                }
            }
        }
        cleanups
    };

    for cleanup in cleanups_to_call {
        cleanup();
    }
}

#[allow(dead_code)]
fn dispose_scope(scope_id: usize) {
    clear_scope_effects(scope_id);

    let mut rt = RUNTIME.lock();

    if scope_id <= rt.scopes.len() {
        rt.scopes[scope_id - 1] = None;
    }

    rt.pending_scope_renders.retain(|s| *s != scope_id);
    rt.scope_signal_changes.retain(|(s, _)| *s != scope_id);

    let sig_dep_keys: Vec<_> = rt
        .signal_dependencies
        .keys()
        .filter(|(s, _)| *s == scope_id)
        .copied()
        .collect();
    for key in sig_dep_keys {
        rt.signal_dependencies.remove(&key);
    }

    for subs in rt.signal_dependencies.values_mut() {
        subs.retain(|s| *s != scope_id);
    }
    rt.scope_dependencies.remove(&scope_id);
}

struct ScopeGuard {
    previous_scope: Option<usize>,
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        RUNTIME.lock().current_scope = self.previous_scope;
        IN_SCOPE.store(self.previous_scope.is_some(), AtomicOrdering::Relaxed);
    }
}

fn render_scope(scope_id: usize) -> Node {
    let (previous_scope, scope_fn) = {
        let mut rt = RUNTIME.lock();

        let previous_scope = rt.current_scope;

        if rt.rendering_scope == scope_id {
            rt.push_pending_render(scope_id);
            return Node::Empty;
        }

        rt.current_scope = Some(scope_id);
        IN_SCOPE.store(true, AtomicOrdering::Relaxed);

        let has_dependencies = rt.scope_dependencies.contains_key(&scope_id);
        if has_dependencies {
            let has_changes = rt
                .signal_dependencies
                .iter()
                .any(|(_, scopes)| scopes.contains(&scope_id));
            if !has_changes {
                rt.current_scope = previous_scope;
                return Node::Empty;
            }
        }

        rt.rendering_scope = scope_id;
        rt.scope_signal_changes
            .retain(|&(scope, _)| scope != scope_id);

        if let Some(signal_ids) = rt.scope_dependencies.remove(&scope_id) {
            for signal_id in signal_ids {
                if let Some(scopes) = rt.signal_dependencies.get_mut(&signal_id) {
                    scopes.retain(|s| *s != scope_id);
                }
            }
        }

        let scope_fn = rt.scope_mut(scope_id).and_then(|s| s.function.take());

        if let Some(scope) = rt.scope_mut(scope_id) {
            scope.effects.clear();
            scope.effect_cleanups.clear();
            scope.effect_executing.clear();
            scope.effect_count = 0;
        }

        (previous_scope, scope_fn)
    }; // Lock released here before calling user code

    let _guard = ScopeGuard { previous_scope };

    let node = scope_fn.map(|mut fnc| {
        let mut node = fnc();

        let callback = {
            let mut rt = RUNTIME.lock();
            let scope = rt.ensure_scope(scope_id);
            scope.function = Some(fnc);
            if let Some(el) = node.as_element_mut() {
                el.key = scope_id.to_string();
            }
            scope.callback.clone()
        };

        if let Some(callback) = callback {
            callback(&node);
        }

        node
    });

    run_scope_effects(scope_id);

    {
        let mut rt = RUNTIME.lock();
        if let Some(scope) = rt.scope_mut(scope_id) {
            scope.signal_count = 0;
            scope.effect_count = 0;
        }
        rt.rendering_scope = 0;

        if !rt.scope_signal_changes.is_empty() {
            let changed: Vec<_> = rt.scope_signal_changes.iter().copied().collect();
            let mut scopes_to_queue = Vec::new();
            for signal_id in changed {
                if let Some(dependent_scopes) = rt.signal_dependencies.get(&signal_id) {
                    for &dep_scope in dependent_scopes {
                        if dep_scope != scope_id {
                            scopes_to_queue.push(dep_scope);
                        }
                    }
                }
            }
            for dep_scope in scopes_to_queue {
                rt.push_pending_render(dep_scope);
            }
        }
    }

    node.unwrap_or(Node::Empty)
}

fn run_scope_effects(scope_id: usize) {
    let effect_count = {
        let rt = RUNTIME.lock();
        rt.scope(scope_id).map(|s| s.effect_count).unwrap_or(0)
    };

    if effect_count == 0 {
        return;
    }

    for idx in 0..effect_count {
        {
            let mut rt = RUNTIME.lock();
            let is_executing = rt
                .scope(scope_id)
                .and_then(|s| s.effect_executing.get(idx).copied())
                .unwrap_or(false);
            if is_executing {
                continue;
            }
            if let Some(scope) = rt.scope_mut(scope_id) {
                if idx < scope.effect_executing.len() {
                    scope.effect_executing[idx] = true;
                }
            }
            rt.executing_effects_count += 1;
        }

        let effect_fn = {
            let mut rt = RUNTIME.lock();
            rt.scope_mut(scope_id)
                .and_then(|s| s.effects.get_mut(idx).and_then(|e| e.take()))
        };
        if let Some(effect) = &effect_fn {
            effect();
        }
        if let Some(effect) = effect_fn {
            let mut rt = RUNTIME.lock();
            if let Some(scope) = rt.scope_mut(scope_id) {
                if idx < scope.effects.len() {
                    scope.effects[idx] = Some(effect);
                }
            }
        }

        {
            let mut rt = RUNTIME.lock();
            if let Some(scope) = rt.scope_mut(scope_id) {
                if idx < scope.effect_executing.len() {
                    scope.effect_executing[idx] = false;
                }
            }
            rt.executing_effects_count -= 1;
        }
    }
}

fn process_pending_renders() {
    const MAX_ITERATIONS: usize = 100;
    let mut iterations = 0;

    while let Some(scope_id) = {
        let mut rt = RUNTIME.lock();
        if rt.pending_scope_renders.is_empty() {
            None
        } else {
            // Process smallest scope_id first so parent scopes render
            // before orphaned child scopes (matches main branch BTreeSet order).
            let min_idx = rt
                .pending_scope_renders
                .iter()
                .enumerate()
                .min_by_key(|&(_, &id)| id)
                .map(|(i, _)| i)
                .unwrap();
            Some(rt.pending_scope_renders.swap_remove(min_idx))
        }
    } {
        iterations += 1;
        if iterations >= MAX_ITERATIONS {
            break;
        }
        render_scope(scope_id);
    }
}

#[cfg(test)]
fn reset_runtime_state() {
    let mut rt = RUNTIME.lock();
    *rt = RuntimeState::new();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceStatus {
    Idle,
    Pending,
    Loading,
    Resolved,
}

impl SignalValue for ResourceStatus {
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }
}

pub struct Resource<T> {
    status: Signal<ResourceStatus>,
    value: Signal<Option<T>>,
}

impl<T: SignalValue + PartialEq + 'static> Resource<T> {
    pub fn status(&self) -> Signal<ResourceStatus> {
        self.status
    }

    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.value.get()
    }

    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.value.with(|v| v.as_ref().map(f)).unwrap_or_default()
    }

    pub fn retry(&self) {
        self.status.set(ResourceStatus::Pending);
    }
}

#[allow(unused_variables)]
/// Create a resource that can be asynchronously loaded
pub fn create_resource<T, F>(fetcher: F) -> Resource<T>
where
    T: SignalValue + PartialEq + 'static,
    F: AsyncFn() -> T + Send + Clone + 'static,
{
    let value = create_signal(None);
    let status = create_signal(ResourceStatus::Idle);

    create_effect(move || {
        if status.get() == ResourceStatus::Idle || status.get() == ResourceStatus::Pending {
            #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
            status.set(ResourceStatus::Loading);

            #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
            let fetcher = fetcher.clone();

            #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
            wasm_bindgen_futures::spawn_local(async move {
                let val = fetcher().await;
                value.set(Some(val));
                status.set(ResourceStatus::Resolved);
            });

            #[cfg(not(all(feature = "wasm", target_arch = "wasm32")))]
            let _ = &fetcher;
        }
    });

    Resource { status, value }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::sync::Arc;
    use core::sync::atomic::{AtomicUsize, Ordering};

    static TEST_MUTEX: spin::Mutex<()> = spin::Mutex::new(());

    #[test]
    fn test_nested_scopes() {
        let _guard = TEST_MUTEX.lock();
        reset_runtime_state();

        run_scope(
            || {
                let outer_signal = create_signal(0);

                run_scope(
                    move || {
                        let inner_signal = create_signal("hello");
                        assert!(inner_signal.get() == "hello");
                        outer_signal.set(42);
                        Node::Empty
                    },
                    |_| {},
                );

                assert_eq!(outer_signal.get(), 42);
                Node::Empty
            },
            |_| {},
        );
    }

    #[test]
    fn test_signal_and_effect_in_scope() {
        let _guard = TEST_MUTEX.lock();
        reset_runtime_state();

        run_scope(
            move || {
                let effect_count = Arc::new(AtomicUsize::new(0));
                let effect_count_clone = effect_count.clone();
                let signal = create_signal(0);

                create_effect(move || {
                    let _ = signal.get();
                    effect_count_clone.fetch_add(1, Ordering::SeqCst);
                    assert!(effect_count.load(Ordering::SeqCst) > 0);
                    signal.set(1);
                });

                Node::Empty
            },
            |_| {},
        );
    }

    #[test]
    fn test_multiple_signals_and_dependencies() {
        let _guard = TEST_MUTEX.lock();
        reset_runtime_state();

        run_scope(
            || {
                let signal1 = create_signal("hello");
                let signal2 = create_signal(0);

                create_effect(move || {
                    let _str_val = signal1.get();
                    let _num_val = signal2.get();
                });

                signal1.set("world");
                signal2.set(42);

                assert_eq!(signal1.get(), "world");
                assert_eq!(signal2.get(), 42);

                Node::Empty
            },
            |_| {},
        );
    }

    #[test]
    fn stale_effects_are_removed_after_rerender() {
        let _guard = TEST_MUTEX.lock();
        reset_runtime_state();

        let handles = Arc::new(spin::Mutex::new(None));
        let had_effect = Arc::new(AtomicUsize::new(0));

        run_scope(
            {
                let handles = handles.clone();
                let had_effect = had_effect.clone();

                move || {
                    let enabled = create_signal(true);
                    let trigger = create_signal(0);

                    *handles.lock() = Some((enabled, trigger));

                    if enabled.get() {
                        create_effect(move || {
                            let _ = trigger.get();
                        });

                        had_effect.store(1, Ordering::SeqCst);
                    }

                    Node::Empty
                }
            },
            |_| {},
        );

        let (enabled, _) = handles.lock().as_ref().copied().unwrap();
        let scope_id = enabled.id.0;

        assert_eq!(had_effect.load(Ordering::SeqCst), 1);

        enabled.set(false);
        render_scope(scope_id);

        assert!(
            RUNTIME
                .lock()
                .scope(scope_id)
                .map(|s| s.effects.iter().all(|e| e.is_none()))
                .unwrap_or(true)
        );
    }

    #[test]
    fn transient_scope_disposes_runtime_state() {
        let _guard = TEST_MUTEX.lock();
        reset_runtime_state();

        run_scope_transient(
            || {
                let signal = create_signal(1);
                create_effect(move || {
                    let _ = signal.get();
                });
                Node::Empty
            },
            |_| {},
        );

        let rt = RUNTIME.lock();
        // Transient scopes clear their data but keep the slot for reuse
        for scope in rt.scopes.iter().flatten() {
            assert_eq!(scope.signal_count, 0);
            assert_eq!(scope.effect_count, 0);
            assert!(scope.signals.is_empty());
            assert!(scope.effects.is_empty());
        }
        // Dep tracking is skipped in transient mode, so these stay empty
        assert!(rt.scope_dependencies.is_empty());
        assert!(rt.signal_dependencies.is_empty());
    }

    #[test]
    fn non_wasm_resources_do_not_enter_loading_state() {
        let _guard = TEST_MUTEX.lock();
        reset_runtime_state();

        run_scope_transient(
            || {
                let resource = create_resource(|| async { 1_u32 });
                assert_eq!(resource.status().get(), ResourceStatus::Idle);
                Node::Empty
            },
            |_| {},
        );
    }

    #[test]
    fn isolated_runtime_clears_global_state() {
        let _guard = TEST_MUTEX.lock();
        reset_runtime_state();

        let html = with_isolated_runtime(|| {
            run_scope_transient(
                || {
                    let signal = create_signal(7);
                    Node::from(signal.get())
                },
                |_| {},
            )
            .to_html()
        });

        assert_eq!(html, "7");
        let rt = RUNTIME.lock();
        assert!(rt.scopes.iter().all(|s| s.is_none()));
        assert!(rt.scope_dependencies.is_empty());
        assert!(rt.signal_dependencies.is_empty());
    }
}
