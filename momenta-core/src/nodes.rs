// For no_std, we need to use alloc collections instead of std
use crate::signals::{Signal, SignalValue};
use alloc::{
    borrow::Cow,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt::Display, iter::FromIterator};

pub use momenta_macros::{component, rsx, when};

/// Optimized HTML writer that minimizes allocations
///
/// This struct provides efficient HTML generation by:
/// - Using a pre-allocated buffer with estimated capacity
/// - Writing directly to the buffer without intermediate string creation
/// - Leveraging fmt::Write for zero-allocation formatting
pub struct HtmlWriter {
    buffer: String,
}

impl HtmlWriter {
    /// Creates a new HtmlWriter with the specified capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
        }
    }

    /// Creates a new HtmlWriter with default capacity
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(512)
    }

    /// Writes an opening tag with attributes
    #[inline]
    fn write_open_tag(&mut self, tag: &str, attributes: &BTreeMap<String, String>) {
        self.buffer.push('<');
        self.buffer.push_str(tag);

        if !attributes.is_empty() {
            for (key, value) in attributes {
                self.buffer.push(' ');
                self.buffer.push_str(key);
                self.buffer.push_str("=\"");
                self.buffer.push_str(value);
                self.buffer.push('"');
            }
        }

        self.buffer.push('>');
    }

    /// Fast path for tags without attributes
    #[inline]
    fn write_open_tag_no_attrs(&mut self, tag: &str) {
        self.buffer.push('<');
        self.buffer.push_str(tag);
        self.buffer.push('>');
    }

    /// Writes a closing tag
    #[inline]
    fn write_close_tag(&mut self, tag: &str) {
        self.buffer.push_str("</");
        self.buffer.push_str(tag);
        self.buffer.push('>');
    }

    /// Writes sanitized text content
    #[inline]
    fn write_text(&mut self, text: &str) {
        // Fast path: check if text needs escaping at all
        let needs_escape = text
            .bytes()
            .any(|b| matches!(b, b'<' | b'>' | b'&' | b'"' | b'/'));

        if !needs_escape {
            // Zero-cost: no escaping needed
            self.buffer.push_str(text);
            return;
        }

        // Slow path: escape as needed
        for c in text.chars() {
            match c {
                '<' => self.buffer.push_str("&lt;"),
                '>' => self.buffer.push_str("&gt;"),
                '&' => self.buffer.push_str("&amp;"),
                '"' => self.buffer.push_str("&quot;"),
                '/' => self.buffer.push_str("&#x2F;"),
                _ => self.buffer.push(c),
            }
        }
    }

    /// Writes a node to the buffer
    #[inline]
    fn write_node(&mut self, node: &Node) {
        match node {
            Node::Element(el) => {
                // Fast path for elements without attributes
                if el.attributes.is_empty() {
                    self.write_open_tag_no_attrs(&el.tag);
                } else {
                    self.write_open_tag(&el.tag, &el.attributes);
                }

                if el.children.is_empty() && !el.inner_html.is_empty() {
                    self.buffer.push_str(&el.inner_html);
                } else {
                    for child in &el.children {
                        self.write_node(child);
                    }
                }
                self.write_close_tag(&el.tag);
            }
            Node::Text(text) => {
                self.write_text(text);
            }
            Node::Fragment(nodes) => {
                for node in nodes {
                    self.write_node(node);
                }
            }
            Node::Comment(comment) => {
                self.buffer.push_str("<!--");
                self.buffer.push_str(comment);
                self.buffer.push_str("-->");
            }
            Node::Static(html) => {
                // Zero-cost: pre-rendered HTML, no escaping needed
                self.buffer.push_str(html);
            }
            Node::Empty => {}
        }
    }

    /// Consumes the writer and returns the generated HTML
    #[inline]
    pub fn finish(self) -> String {
        self.buffer
    }
}

impl Default for HtmlWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to conditionally join CSS classes
///
/// # Example
/// ```rust
/// use momenta_core::nodes::classes;
///
/// let is_active = true;
/// let is_disabled = false;
/// let class_name = classes(&[
///     ("btn", true),
///     ("btn-active", is_active),
///     ("btn-disabled", is_disabled),
/// ]);
/// assert_eq!(class_name, "btn btn-active");
/// ```
pub fn classes(items: &[(&str, bool)]) -> String {
    items
        .iter()
        .filter_map(|(class, condition)| if *condition { Some(*class) } else { None })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Macro for creating conditional classes more ergonomically
///
/// # Example
/// ```rust
/// use momenta_core::class;
///
/// let is_active = true;
/// let class_name = class!("btn", is_active => "btn-active", "btn-primary");
/// ```
#[macro_export]
macro_rules! class {
    // Mixed static and conditional classes
    ($($item:tt)*) => {{
        let mut result = String::new();
        $crate::__class_internal!(result; $($item)*);
        result
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __class_internal {
    // Base case: empty
    ($result:ident;) => {};

    // Conditional class: condition => "class"
    ($result:ident; $cond:expr => $class:expr $(, $($rest:tt)*)?) => {
        if $cond {
            if !$result.is_empty() {
                $result.push(' ');
            }
            $result.push_str($class);
        }
        $($crate::__class_internal!($result; $($rest)*);)?
    };

    // Static class: "class"
    ($result:ident; $class:expr $(, $($rest:tt)*)?) => {
        if !$result.is_empty() {
            $result.push(' ');
        }
        $result.push_str($class);
        $($crate::__class_internal!($result; $($rest)*);)?
    };
}

#[cfg(feature = "wasm")]
use alloc::{boxed::Box, sync::Arc};

/// A trait for converting values into HTML attribute strings.
///
/// This trait is automatically implemented for any type that implements `ToString`,
/// making it easy to use various types as attribute values.
///
/// # Example
///
/// ```rust,ignore
/// use momenta_core::prelude::*;
///
/// let element = rsx!(<div id="my-id" hidden={true} />);
/// ```
pub trait Attribute {
    fn value(&self) -> String;
}

/// A trait for handling optional attribute values.
///
/// This trait is automatically implemented for `Option<T>` where T implements `ToString`.
/// It allows for graceful handling of optional attributes, rendering them as empty strings when None.
///
/// # Example
///
/// ```rust
/// use momenta_core::nodes::OptionAttribute;
///
/// let maybe_title: Option<&str> = Some("Hello");
/// assert_eq!(maybe_title.value(), "Hello");
///
/// let no_title: Option<&str> = None;
/// assert_eq!(no_title.value(), "");
/// ```
pub trait OptionAttribute {
    fn value(&self) -> String;
}

impl<T: ToString> Attribute for T {
    fn value(&self) -> String {
        self.to_string()
    }
}

impl<T: ToString> OptionAttribute for Option<T> {
    fn value(&self) -> String {
        match self {
            Some(t) => t.to_string(),
            None => String::new(),
        }
    }
}

#[derive(Clone)]
/// Represents an HTML element with its tag name, attributes, and children.
///
/// Elements are the building blocks of the RSX tree structure. Each element
/// can have attributes (like class, id, etc.) and can contain other elements
/// or text nodes as children.
///
/// You typically won't create Elements directly, but rather use the `rsx!` macro:
///
/// ```rust,ignore
/// use momenta_core::prelude::*;
///
/// let element = rsx!(
///     <div class="container">
///         <p>Hello world!</p>
///     </div>
/// );
/// ```
pub struct Element {
    pub(crate) key: String,
    tag: Cow<'static, str>,
    attributes: BTreeMap<String, String>,
    inner_html: String,
    children: Vec<Node>,
    #[cfg(feature = "wasm")]
    events: BTreeMap<String, EventCallback>,
    #[cfg(not(feature = "wasm"))]
    #[allow(unused)]
    events: BTreeMap<String, String>,
}

impl Element {
    pub fn parse_tag_with_attributes(
        key: &str,
        tag: &'static str,
        attributes: BTreeMap<String, String>,
        #[cfg(feature = "wasm")] events: BTreeMap<String, EventCallback>,
        #[cfg(not(feature = "wasm"))] events: BTreeMap<String, String>,
        inner_html: &str,
        children: Vec<Node>,
    ) -> Node {
        Node::Element(Element {
            tag: Cow::Borrowed(tag),
            key: key.to_string(),
            attributes,
            events,
            children,
            inner_html: inner_html.to_string(),
        })
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn html(&self) -> &String {
        &self.inner_html
    }

    #[cfg(not(feature = "wasm"))]
    pub fn events(&self) -> &BTreeMap<String, String> {
        &self.events
    }

    #[cfg(feature = "wasm")]
    pub fn events(&self) -> &BTreeMap<String, EventCallback> {
        &self.events
    }
}

/// A trait for creating reusable components.
///
/// Components are the heart of RSX's reusability model. They allow you to create
/// custom elements with their own logic and state.
///
/// # Example
///
/// ```rust,ignore
/// use momenta_core::prelude::*;
///
/// struct Card;
/// #[derive(Default)]
/// struct CardProps {
///     title: String,
///     children: Vec<Node>,
/// }
///
/// impl Component for Card {
///     type Props = CardProps;
///     fn render(props: &Self::Props) -> Node {
///         rsx!(
///             <div class="card">
///                 <h2>{&props.title}</h2>
///                 <div class="card-content">{&props.children}</div>
///             </div>
///         )
///     }
/// }
/// ```
pub trait Component {
    /// The type of props this component accepts
    type Props;

    /// Renders the component with the given props
    fn render(props: &Self::Props) -> Node;
}

#[derive(Default)]
pub struct DefaultProps;

#[derive(Clone)]
/// Represents a node in the RSX tree.
///
/// Nodes are the fundamental building blocks of RSX. They can be:
/// - Elements (like `<div>` or `<p>`)
/// - Text content
/// - Fragments (groups of nodes)
/// - Comments
/// - Static pre-rendered HTML
///
/// # Example
///
/// ```rust
/// use momenta_core::prelude::*;
///
/// let text_node = Node::Text("Hello".to_string());
/// let fragment = Node::Fragment(vec![text_node]);
/// ```
pub enum Node {
    /// An HTML element with a tag name, attributes, and children
    Element(Element),
    /// Plain text content
    Text(String),
    /// A group of nodes without a wrapper element
    Fragment(Vec<Node>),
    /// An HTML comment
    Comment(String),
    /// Pre-rendered static HTML (zero-cost, no escaping needed)
    Static(&'static str),
    Empty,
}

impl Node {
    /// Attempts to get a mutable reference to the underlying Element if this node is an Element.
    ///
    /// Returns None if the node is not an Element (e.g., if it's Text or Fragment).
    pub fn as_element_mut(&mut self) -> Option<&mut Element> {
        match self {
            Node::Element(el) => Some(el),
            _ => None,
        }
    }

    /// Attempts to get a reference to the underlying Element if this node is an Element.
    ///
    /// Returns None if the node is not an Element (e.g., if it's Text or Fragment).
    pub fn as_element(&self) -> Option<&Element> {
        match self {
            Node::Element(el) => Some(el),
            _ => None,
        }
    }

    /// Efficiently converts the node to HTML using optimized string building.
    ///
    /// This method provides lazy HTML generation with minimal allocations:
    /// - Uses a pre-allocated buffer to minimize reallocations
    /// - Writes directly to the buffer without intermediate string creation
    /// - Estimates capacity based on node structure
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use momenta_core::prelude::*;
    ///
    /// let node = rsx!(<div class="container">Hello</div>);
    /// let html = node.to_html();
    /// ```
    pub fn to_html(&self) -> String {
        let capacity = self.estimate_html_size();
        let mut writer = HtmlWriter::with_capacity(capacity);
        writer.write_node(self);
        writer.finish()
    }

    /// Writes this node's HTML to an existing HtmlWriter.
    ///
    /// This is useful for composing multiple nodes efficiently.
    pub fn write_to(&self, writer: &mut HtmlWriter) {
        writer.write_node(self);
    }

    /// Estimates the HTML size for capacity pre-allocation.
    ///
    /// This provides a rough estimate to minimize reallocations during rendering.
    #[inline]
    fn estimate_html_size(&self) -> usize {
        match self {
            Node::Element(el) => {
                // Tag overhead: <tag> + </tag> = 5 + 2*tag_len
                let mut size = 5 + (el.tag.len() * 2);

                // Attributes: key="value"
                for (key, value) in &el.attributes {
                    size += key.len() + value.len() + 4; // key="value" + space
                }

                // Inner HTML
                if !el.inner_html.is_empty() {
                    size += el.inner_html.len();
                }

                // Children
                for child in &el.children {
                    size += child.estimate_html_size();
                }

                size
            }
            Node::Text(text) => text.len() + (text.len() >> 2), // Add 25% using bitshift
            Node::Fragment(nodes) => {
                let mut size = 0;
                for node in nodes {
                    size += node.estimate_html_size();
                }
                size
            }
            Node::Comment(comment) => comment.len() + 7, // <!-- -->
            Node::Static(html) => html.len(),            // Exact size, no escaping
            Node::Empty => 0,
        }
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::Text(value)
    }
}

impl From<&String> for Node {
    fn from(value: &String) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<&&str> for Node {
    fn from(value: &&str) -> Self {
        Node::Text(value.to_string())
    }
}

impl<T> From<Vec<T>> for Node
where
    Node: From<T>,
{
    fn from(value: Vec<T>) -> Self {
        Node::Fragment(value.into_iter().map(|t| Node::from(t)).collect())
    }
}

impl FromIterator<Node> for Node {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        Node::Fragment(iter.into_iter().collect())
    }
}

impl<T: Clone> From<&Vec<T>> for Node
where
    Node: From<T>,
{
    fn from(value: &Vec<T>) -> Self {
        Node::Fragment(value.clone().into_iter().map(|t| Node::from(t)).collect())
    }
}

impl<T> From<Option<T>> for Node
where
    Node: From<T>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Node::from(t),
            _ => Node::Empty,
        }
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<u32> for Node {
    fn from(value: u32) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<u64> for Node {
    fn from(value: u64) -> Self {
        Node::Text(value.to_string())
    }
}

impl FromIterator<u32> for Node {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut result = Vec::new();
        for item in iter {
            result.push(Node::Text(item.to_string()));
        }
        Node::Fragment(result)
    }
}

impl FromIterator<String> for Node {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut result = Vec::new();
        for item in iter {
            result.push(Node::Text(item));
        }
        Node::Fragment(result)
    }
}

impl FromIterator<u64> for Node {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut result = Vec::new();
        for item in iter {
            result.push(Node::Text(item.to_string()));
        }
        Node::Fragment(result)
    }
}

impl FromIterator<i32> for Node {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut result = Vec::new();
        for item in iter {
            result.push(Node::Text(item.to_string()));
        }
        Node::Fragment(result)
    }
}

impl From<f32> for Node {
    fn from(value: f32) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        Node::Text(value.to_string())
    }
}

impl<T: SignalValue + PartialEq + Clone + 'static> From<Signal<T>> for Node
where
    T: Into<Node>,
{
    fn from(value: Signal<T>) -> Self {
        value.get().into()
    }
}

impl<I, F, R> From<core::iter::Map<I, F>> for Node
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
    R: Into<Node>,
    Vec<Node>: FromIterator<R>,
{
    fn from(iter: core::iter::Map<I, F>) -> Self {
        let nodes: Vec<Node> = iter.collect();
        Node::from(nodes)
    }
}

impl<I, F, R> From<&core::iter::Map<I, F>> for Node
where
    I: Iterator + Clone,
    F: FnMut(I::Item) -> R + Clone,
    R: Into<Node>,
    Vec<Node>: FromIterator<R>,
{
    fn from(iter: &core::iter::Map<I, F>) -> Self {
        let nodes: Vec<Node> = iter.clone().collect();
        Node::from(nodes)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Use the optimized to_html() method and write the result
        write!(f, "{}", self.to_html())
    }
}

#[cfg(feature = "wasm")]
type EventCallbackInner = Arc<spin::Mutex<Box<dyn FnMut(web_sys::Event) + Send + Sync>>>;

#[cfg(feature = "wasm")]
#[derive(Default)]
pub struct EventCallback(Option<EventCallbackInner>);

#[cfg(feature = "wasm")]
impl EventCallback {
    pub fn new<F>(callback: F) -> Self
    where
        F: FnMut(web_sys::Event) + Send + Sync + 'static,
    {
        Self(Some(Arc::new(spin::Mutex::new(Box::new(callback)))))
    }

    pub fn has_callback(&self) -> bool {
        self.0.is_some()
    }

    pub fn call(&mut self, event: web_sys::Event) {
        if let Some(cb) = &mut self.0 {
            let mut cb = cb.lock();
            cb(event);
        }
    }
}

#[cfg(feature = "wasm")]
impl Clone for EventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// For convenience with Fn closures
#[cfg(feature = "wasm")]
impl<F> From<F> for EventCallback
where
    F: FnMut(web_sys::Event) + Send + Sync + 'static,
{
    fn from(callback: F) -> Self {
        Self::new(callback)
    }
}
