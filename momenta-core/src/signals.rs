use crate::nodes::Node;
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::{
    any::Any,
    cmp::Ordering,
    marker::PhantomData,
    ops::{AddAssign, DivAssign, MulAssign, Not, SubAssign},
};
use spin::Mutex;

//==============================================================================
// GLOBAL STATE
//==============================================================================

/// Current scope being executed
static CURRENT_SCOPE: Mutex<Option<usize>> = Mutex::new(None);
/// Scope currently being rendered (0 = none)
static RENDERING_SCOPE: Mutex<usize> = Mutex::new(0);
/// Next available scope ID
static NEXT_SCOPE_ID: Mutex<usize> = Mutex::new(1);

/// Signal counter per scope
static SCOPE_SIGNAL_COUNTERS: Mutex<BTreeMap<usize, usize>> = Mutex::new(BTreeMap::new());
/// Effect counter per scope
static SCOPE_EFFECT_COUNTERS: Mutex<BTreeMap<usize, usize>> = Mutex::new(BTreeMap::new());
/// Batch update flag - when true, don't trigger re-renders immediately
static BATCH_UPDATES: Mutex<bool> = Mutex::new(false);

/// All signal values by (scope_id, signal_id)
static SIGNALS: Mutex<BTreeMap<(usize, usize), StoredValue>> = Mutex::new(BTreeMap::new());
/// Signals that changed during current scope execution
static SCOPE_SIGNAL_CHANGES: Mutex<BTreeSet<(usize, usize)>> = Mutex::new(BTreeSet::new());

type ScopeCallback = Arc<dyn Fn(&Node) + Send + Sync>;
type ScopeEffect = Box<dyn Fn() + Send>;
type ScopeEffectCleanup = Box<dyn FnOnce() + Send>;

/// Functions that can be re-executed per scope
static SCOPE_FUNCTIONS: Mutex<BTreeMap<usize, Box<dyn FnMut() -> Node + Send>>> =
    Mutex::new(BTreeMap::new());
/// Callbacks to run after scope renders
static SCOPE_CALLBACKS: Mutex<BTreeMap<usize, ScopeCallback>> = Mutex::new(BTreeMap::new());
/// Effects by (scope_id, effect_id)
static SCOPE_EFFECTS: Mutex<BTreeMap<(usize, usize), ScopeEffect>> = Mutex::new(BTreeMap::new());
/// Effect cleanup functions by (scope_id, effect_id)
static SCOPE_EFFECT_CLEANUPS: Mutex<BTreeMap<(usize, usize), ScopeEffectCleanup>> =
    Mutex::new(BTreeMap::new());

/// Which signals each scope depends on
static SCOPE_DEPENDENCIES: Mutex<BTreeMap<usize, BTreeSet<(usize, usize)>>> =
    Mutex::new(BTreeMap::new());
/// Which scopes depend on each signal
static SIGNAL_DEPENDENCIES: Mutex<BTreeMap<(usize, usize), BTreeSet<usize>>> =
    Mutex::new(BTreeMap::new());
/// Scopes waiting to re-render
static PENDING_SCOPE_RENDERS: Mutex<BTreeSet<usize>> = Mutex::new(BTreeSet::new());
/// Memoized computation results
#[allow(dead_code)]
static MEMO_CACHE: Mutex<BTreeMap<String, Box<dyn SignalValue>>> = Mutex::new(BTreeMap::new());
/// Currently executing effects (to prevent infinite loops)
static EXECUTING_EFFECTS: Mutex<BTreeSet<(usize, usize)>> = Mutex::new(BTreeSet::new());

//==============================================================================
// TRAITS
//==============================================================================

/// Values that can be stored in signals
pub trait SignalValue: Send {
    fn as_any(&self) -> Option<&dyn Any>;
}

macro_rules! impl_signal_value {
    ($($t:ty),*) => {
        $(
            impl SignalValue for $t {
                fn as_any(&self) -> Option<&dyn Any> {
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
}

impl<T: SignalValue + 'static> SignalValue for Option<T> {
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}

//==============================================================================
// SIGNAL TYPE
//==============================================================================

/// Reactive value that triggers re-renders when changed
#[derive(Copy, Debug)]
pub struct Signal<T> {
    id: (usize, usize),
    _marker: PhantomData<T>,
}

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

// implement .then for Signal

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal {
            id: self.id,
            _marker: PhantomData,
        }
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

// Signal-to-Signal equality
impl<T: SignalValue + PartialEq + Clone + 'static> PartialEq<Signal<T>> for Signal<T> {
    fn eq(&self, other: &Signal<T>) -> bool {
        other
            .with(|other_val| self.with(|self_val| self_val == other_val).unwrap_or(false))
            .unwrap_or(false)
    }
}

// Boolean convenience methods
impl Signal<bool> {
    /// Toggle the boolean value
    pub fn toggle(&self) {
        self.set(!self.get());
    }

    /// Set to true
    pub fn turn_on(&self) {
        self.set(true);
    }

    /// Set to false
    pub fn turn_off(&self) {
        self.set(false);
    }
}

// Vector convenience methods
impl<T: SignalValue + PartialEq + Clone + 'static> Signal<Vec<T>> {
    /// Push an item to the vector
    pub fn push(&self, item: T) {
        let mut vec = self.get();
        vec.push(item);
        self.set(vec);
    }

    /// Pop an item from the vector
    pub fn pop(&self) -> Option<T> {
        let mut vec = self.get();
        let result = vec.pop();
        self.set(vec);
        result
    }

    /// Insert an item at the specified index
    pub fn insert(&self, index: usize, item: T) {
        let mut vec = self.get();
        vec.insert(index, item);
        self.set(vec);
    }

    /// Remove and return the item at the specified index
    pub fn remove(&self, index: usize) -> T {
        let mut vec = self.get();
        let result = vec.remove(index);
        self.set(vec);
        result
    }

    /// Retain only the elements that satisfy the predicate
    pub fn retain<F>(&self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut vec = self.get();
        vec.retain(f);
        self.set(vec);
    }

    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.with(|v| v.len()).unwrap_or(0)
    }

    /// Check if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.with(|v| v.is_empty()).unwrap_or(true)
    }

    /// Clear the vector
    pub fn clear(&self) {
        self.set(Vec::new());
    }

    /// Get element at index
    pub fn get_at(&self, index: usize) -> Option<T> {
        self.with(|v| v.get(index).cloned()).unwrap_or(None)
    }

    /// Update element at index
    pub fn update_at(&self, index: usize, item: T) {
        let mut vec = self.get();
        if index < vec.len() {
            vec[index] = item;
            self.set(vec);
        }
    }
}

// Vector sorting methods (requires Ord)
impl<T: SignalValue + PartialEq + Clone + Ord + 'static> Signal<Vec<T>> {
    /// Sort the vector in ascending order
    pub fn sort(&self) {
        let mut vec = self.get();
        vec.sort();
        self.set(vec);
    }

    /// Sort the vector by a key function
    pub fn sort_by_key<K, F>(&self, f: F)
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        let mut vec = self.get();
        vec.sort_by_key(f);
        self.set(vec);
    }

    /// Reverse the order of elements
    pub fn reverse(&self) {
        let mut vec = self.get();
        vec.reverse();
        self.set(vec);
    }
}

// impl iter for Signal where T is a vec
impl<T: SignalValue + Clone + 'static> Signal<Vec<T>> {
    /// Get an iterator over the vector contents (creates a snapshot)
    pub fn iter(&self) -> impl Iterator<Item = T> {
        self.get().into_iter()
    }

    /// Apply a function to each element and collect results
    pub fn map<R, F>(&self, f: F) -> Vec<R>
    where
        F: FnMut(T) -> R,
    {
        self.iter().map(f).collect()
    }

    /// Filter elements and return a new vector
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
        let self_clone = self.clone();
        create_computed(move || self_clone.with(|val| f(val)).unwrap())
    }

    /// Access signal value immutably
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        if let Some(current_scope) = get_current_scope() {
            {
                let mut signal_deps = SIGNAL_DEPENDENCIES.lock();
                signal_deps
                    .entry(self.id)
                    .or_default()
                    .insert(current_scope);
            }
            {
                let mut scope_deps = SCOPE_DEPENDENCIES.lock();
                scope_deps.entry(current_scope).or_default().insert(self.id);
            }
        }

        let signals = SIGNALS.lock();
        signals
            .get(&self.id)
            .and_then(|stored| {
                stored
                    .value
                    .as_any()
                    .and_then(|any| any.downcast_ref::<T>())
            })
            .map(f)
    }

    /// Update signal value and trigger re-renders if changed
    pub fn set(&self, value: T)
    where
        T: PartialEq,
    {
        let mut changed = false;

        {
            let mut signals = SIGNALS.lock();
            if let Some(stored) = signals.get_mut(&self.id) {
                if let Some(current_val) = stored
                    .value
                    .as_any()
                    .and_then(|any| any.downcast_ref::<T>())
                {
                    if current_val != &value {
                        *stored = StoredValue {
                            value: Box::new(value),
                        };
                        changed = true;
                    }
                }
            }
        }

        if changed {
            {
                let mut changes = SCOPE_SIGNAL_CHANGES.lock();
                changes.insert(self.id);
            }

            // Only render immediately if we're not batching and not inside a scope or effect
            let should_batch = *BATCH_UPDATES.lock();
            let is_in_effect = !EXECUTING_EFFECTS.lock().is_empty();
            if !should_batch && get_current_scope().is_none() && !is_in_effect {
                render_scope(self.id.0);
            }
        }
    }

    /// Get cloned value
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

//==============================================================================
// BATCH UPDATES
//==============================================================================

/// Run a function with batched updates enabled
/// All signal updates within the function will be batched and applied at the end
///
/// # Example
/// ```ignore
/// use momenta_core::signals::{create_signal, batch};
///
/// // Within a component scope:
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
    {
        let mut batch_flag = BATCH_UPDATES.lock();
        *batch_flag = true;
    }

    let result = f();

    {
        let mut batch_flag = BATCH_UPDATES.lock();
        *batch_flag = false;
    }

    // Process all pending renders
    process_pending_renders();

    result
}

//==============================================================================
// MEMOIZATION
//==============================================================================

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
    let scope_id = get_current_scope()
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap();
    let signal_id = get_next_signal_id_for_scope(scope_id);
    let signal = Signal {
        id: (scope_id, signal_id),
        _marker: PhantomData,
    };

    let cache_key = alloc::format!("{}-{}", key, scope_id);

    // Check cache first
    {
        let cache = MEMO_CACHE.lock();
        if let Some(cached) = cache.get(&cache_key) {
            if let Some(cached_val) = cached.as_any().and_then(|any| any.downcast_ref::<T>()) {
                let mut signals = SIGNALS.lock();
                signals.insert(
                    signal.id,
                    StoredValue {
                        value: Box::new(cached_val.clone()),
                    },
                );
                return signal;
            }
        }
    }

    // Compute and cache
    let initial_value = computation();
    {
        let mut cache = MEMO_CACHE.lock();
        cache.insert(cache_key.clone(), Box::new(initial_value.clone()));
    }

    {
        let mut signals = SIGNALS.lock();
        signals.insert(
            signal.id,
            StoredValue {
                value: Box::new(initial_value),
            },
        );
    }

    // Update cache when dependencies change
    let signal_clone = signal.clone();
    create_effect(move || {
        let new_value = computation();
        {
            let mut cache = MEMO_CACHE.lock();
            cache.insert(cache_key.clone(), Box::new(new_value.clone()));
        }
        signal_clone.set(new_value);
    });

    signal
}

//==============================================================================
// SIGNAL CREATION
//==============================================================================

/// Signal initialization options
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
pub fn create_signal<T, I>(init: I) -> Signal<T>
where
    T: SignalValue + PartialEq + 'static,
    I: Into<SignalInit<T>>,
{
    let scope_id = get_current_scope()
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap(); // safe, we want to panic if not in scope
    let signal_id = get_next_signal_id_for_scope(scope_id);
    let signal = Signal {
        id: (scope_id, signal_id),
        _marker: PhantomData,
    };

    {
        let mut signals = SIGNALS.lock();
        if signals.get(&signal.id).is_none() {
            let initial_value = match init.into() {
                SignalInit::Value(v) => v,
                SignalInit::InitFn(f) => f(),
            };
            signals.insert(
                signal.id,
                StoredValue {
                    value: Box::new(initial_value),
                },
            );
        }
    }

    // Note: We don't automatically add the creating scope as a dependent
    // Dependencies are only established when the signal is actually read via .with() or .get()

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
    let scope_id = get_current_scope()
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap();
    let signal_id = get_next_signal_id_for_scope(scope_id);
    let signal = Signal {
        id: (scope_id, signal_id),
        _marker: PhantomData,
    };

    // Initialize with first computation
    {
        let mut signals = SIGNALS.lock();
        let initial_value = computation();
        signals.insert(
            signal.id,
            StoredValue {
                value: Box::new(initial_value),
            },
        );
    }

    // Create effect to recompute when dependencies change
    let signal_clone = signal.clone();
    create_effect(move || {
        let new_value = computation();
        signal_clone.set(new_value);
    });

    signal
}

//==============================================================================
// EFFECTS
//==============================================================================

#[derive(Clone, Copy, Debug)]
struct Effect {
    id: (usize, usize),
}

/// Create effect that runs when dependencies change
///
/// # Example
/// ```ignore
/// use momenta_core::signals::{create_signal, create_effect};
///
/// // Within a component scope:
/// let count = create_signal(0);
/// create_effect(move || {
///     println!("Count changed to: {}", count.get());
/// });
/// ```
pub fn create_effect(effect: impl Fn() + Send + 'static) {
    let scope_id = get_current_scope()
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap(); // safe, we want to panic if not in scope
    let effect_id = get_next_effect_id_for_scope(scope_id);
    let effect_struct = Effect {
        id: (scope_id, effect_id),
    };

    {
        let mut effects = SCOPE_EFFECTS.lock();
        effects.insert(effect_struct.id, Box::new(effect));
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
/// // Within a component scope:
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
    let scope_id = get_current_scope()
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap();
    let effect_id = get_next_effect_id_for_scope(scope_id);
    let effect_id_tuple = (scope_id, effect_id);

    {
        let mut effects = SCOPE_EFFECTS.lock();

        effects.insert(
            effect_id_tuple,
            Box::new(move || {
                // Run previous cleanup if exists
                {
                    let mut cleanups = SCOPE_EFFECT_CLEANUPS.lock();
                    if let Some(cleanup) = cleanups.remove(&effect_id_tuple) {
                        cleanup();
                    }
                }

                // Run effect and store new cleanup
                let new_cleanup = effect();
                {
                    let mut cleanups = SCOPE_EFFECT_CLEANUPS.lock();
                    cleanups.insert(effect_id_tuple, Box::new(new_cleanup));
                }
            }),
        );
    }
}

//==============================================================================
// SCOPE MANAGEMENT
//==============================================================================

/// Run function within new reactive scope
pub fn run_scope(
    scope_fn: impl FnMut() -> Node + Send + 'static,
    callback: impl Fn(&Node) + Send + Sync + 'static,
) -> Node {
    let scope_id = {
        let mut next_id = NEXT_SCOPE_ID.lock();
        let current = *next_id;
        *next_id = current + 1;
        current
    };

    {
        let mut scope_functions = SCOPE_FUNCTIONS.lock();
        scope_functions.insert(scope_id, Box::new(scope_fn));
    }

    {
        let mut scope_callbacks = SCOPE_CALLBACKS.lock();
        scope_callbacks.insert(scope_id, Arc::new(callback));
    }

    render_scope(scope_id)
}

//==============================================================================
// INTERNAL FUNCTIONS
//==============================================================================

pub(crate) fn get_current_scope() -> Option<usize> {
    *CURRENT_SCOPE.lock()
}

fn set_current_scope(scope_id: Option<usize>) {
    *CURRENT_SCOPE.lock() = scope_id;
}

fn get_next_signal_id_for_scope(scope_id: usize) -> usize {
    let mut counters = SCOPE_SIGNAL_COUNTERS.lock();
    let counter = counters.entry(scope_id).or_insert(0);
    *counter += 1;
    *counter
}

fn get_next_effect_id_for_scope(scope_id: usize) -> usize {
    let mut counters = SCOPE_EFFECT_COUNTERS.lock();
    let counter = counters.entry(scope_id).or_insert(0);
    *counter += 1;
    *counter
}

fn reset_signal_counters(scope_id: usize) {
    SCOPE_SIGNAL_COUNTERS.lock().remove(&scope_id);
}

fn reset_effect_counters(scope_id: usize) {
    SCOPE_EFFECT_COUNTERS.lock().remove(&scope_id);
}

struct ScopeGuard {
    previous_scope: Option<usize>,
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        set_current_scope(self.previous_scope);
    }
}

fn scope_has_signal_changes(scope_id: usize) -> bool {
    // Return true for initial render (when scope has no dependencies yet)
    let has_dependencies = SCOPE_DEPENDENCIES.lock().contains_key(&scope_id);
    if !has_dependencies {
        return true;
    }

    SIGNAL_DEPENDENCIES
        .lock()
        .iter()
        .any(|(&(_, _), scopes)| scopes.contains(&scope_id))
}

fn render_scope(scope_id: usize) -> Node {
    let _guard = ScopeGuard {
        previous_scope: get_current_scope(),
    };

    // Check if this scope is already being rendered to prevent infinite loops
    {
        let rendering_flag = RENDERING_SCOPE.lock();
        if *rendering_flag == scope_id {
            return Node::Empty;
        }
    }

    set_current_scope(Some(scope_id));

    if !scope_has_signal_changes(scope_id) {
        return Node::Empty;
    }

    let (should_clear_deps, was_rendering) = {
        let mut rendering_flag = RENDERING_SCOPE.lock();
        let mut changes = SCOPE_SIGNAL_CHANGES.lock();
        let was_rendering = *rendering_flag;
        *rendering_flag = scope_id;
        // clear scope signals
        changes.retain(|&(scope, _)| scope != scope_id);
        (was_rendering == 0, was_rendering)
    };

    if should_clear_deps {
        let signal_ids = {
            let mut scope_deps = SCOPE_DEPENDENCIES.lock();
            scope_deps.remove(&scope_id)
        };

        if let Some(signal_ids) = signal_ids {
            let mut signal_deps = SIGNAL_DEPENDENCIES.lock();
            for signal_id in signal_ids {
                if let Some(scopes) = signal_deps.get_mut(&signal_id) {
                    scopes.remove(&scope_id);
                }
            }
        }
    }

    let scope_fn = {
        let mut scope_functions = SCOPE_FUNCTIONS.lock();
        scope_functions.remove(&scope_id)
    };

    let node = scope_fn.map(|mut fnc| {
        let mut node = fnc();
        {
            let mut scope_functions = SCOPE_FUNCTIONS.lock();
            scope_functions.insert(scope_id, fnc);
        }
        if let Some(el) = node.as_element_mut() {
            el.key = scope_id.to_string();
        }
        node
    });

    if let Some(ref node) = node {
        let scope_callbacks = SCOPE_CALLBACKS.lock();
        if let Some(callback) = scope_callbacks.get(&scope_id) {
            callback(node);
        }
    }

    reset_signal_counters(scope_id);
    run_scope_effects(scope_id);
    reset_effect_counters(scope_id);

    let signal_changes = {
        let mut changes = SCOPE_SIGNAL_CHANGES.lock();
        let mut rendering_flag = RENDERING_SCOPE.lock();
        let result = if !changes.is_empty() {
            Some(core::mem::take(&mut *changes))
        } else {
            None
        };
        *rendering_flag = was_rendering;
        result
    };

    if let Some(changes) = signal_changes {
        let mut pending = PENDING_SCOPE_RENDERS.lock();
        let signal_deps = SIGNAL_DEPENDENCIES.lock();
        for signal_id in changes {
            if let Some(dependent_scopes) = signal_deps.get(&signal_id) {
                for &dependent_scope in dependent_scopes {
                    if dependent_scope != scope_id {
                        pending.insert(dependent_scope);
                    }
                }
            }
        }

        if was_rendering == 0 {
            drop(pending);
            drop(signal_deps);
            process_pending_renders();
        }
    }

    node.unwrap_or(Node::Empty)
}

fn run_scope_effects(scope_id: usize) {
    // Collect effect IDs first to avoid holding the lock during execution
    let effect_ids: Vec<_> = {
        let effects = SCOPE_EFFECTS.lock();
        effects
            .iter()
            .filter(|((effect_scope_id, _), _)| *effect_scope_id == scope_id)
            .map(|(id, _)| *id)
            .collect()
    };

    // Run each effect without holding the lock
    for effect_id in effect_ids {
        // Check if this effect is already executing
        {
            let mut executing = EXECUTING_EFFECTS.lock();
            if executing.contains(&effect_id) {
                continue; // Skip to prevent infinite loop
            }
            executing.insert(effect_id);
        }

        // Run the effect
        {
            let effects = SCOPE_EFFECTS.lock();
            if let Some(effect) = effects.get(&effect_id) {
                effect();
            }
        }

        // Remove from executing set
        {
            let mut executing = EXECUTING_EFFECTS.lock();
            executing.remove(&effect_id);
        }
    }
}

fn process_pending_renders() {
    const MAX_ITERATIONS: usize = 100;
    let mut iterations = 0;

    while let Some(scope_id) = {
        let mut pending = PENDING_SCOPE_RENDERS.lock();
        pending.iter().next().copied().inspect(|id| {
            pending.remove(id);
        })
    } {
        iterations += 1;
        if iterations >= MAX_ITERATIONS {
            // Break to prevent infinite loops
            // In debug builds, this indicates a potential reactivity cycle
            break;
        }
        render_scope(scope_id);
    }
}

//==============================================================================
// RESOURCE
//==============================================================================

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
    Option<T>: Copy,
{
    let value = create_signal(None);
    let status = create_signal(ResourceStatus::Idle);

    create_effect(move || {
        if status.get() == ResourceStatus::Idle || status.get() == ResourceStatus::Pending {
            status.set(ResourceStatus::Loading);

            #[cfg(feature = "wasm")]
            let fetcher = fetcher.clone();

            #[cfg(feature = "wasm")]
            wasm_bindgen_futures::spawn_local(async move {
                let val = fetcher().await;
                value.set(Some(val));
                status.set(ResourceStatus::Resolved);
            });
        }
    });

    Resource { status, value }
}

//==============================================================================
// TESTS
//==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::sync::Arc;
    use core::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_nested_scopes() {
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
}
