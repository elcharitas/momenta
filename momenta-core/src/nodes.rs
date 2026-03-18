// For no_std, we need to use alloc collections instead of std
use crate::signals::{Signal, SignalValue};
use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt::Display, iter::FromIterator};

pub use momenta_macros::{component, rsx, when};

/// Optimized HTML writer that minimizes allocations by pre-allocating capacity.
pub struct HtmlWriter {
    buffer: String,
}

impl HtmlWriter {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(512)
    }

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

/// Write escaped text content to any `fmt::Write` sink.
#[inline]
fn write_escaped_text(text: &str, w: &mut impl core::fmt::Write) -> core::fmt::Result {
    if !text
        .bytes()
        .any(|b| matches!(b, b'<' | b'>' | b'&' | b'"' | b'/'))
    {
        return w.write_str(text);
    }
    for c in text.chars() {
        match c {
            '<' => w.write_str("&lt;")?,
            '>' => w.write_str("&gt;")?,
            '&' => w.write_str("&amp;")?,
            '"' => w.write_str("&quot;")?,
            '/' => w.write_str("&#x2F;")?,
            _ => w.write_char(c)?,
        }
    }
    Ok(())
}

/// Write a node's HTML to any `fmt::Write` sink (String, Formatter, etc.).
#[inline]
fn write_node_html(node: &Node, w: &mut impl core::fmt::Write) -> core::fmt::Result {
    match node {
        Node::Element(el) => {
            w.write_str("<")?;
            w.write_str(&el.tag)?;
            for (key, value) in &el.attributes {
                w.write_str(" ")?;
                w.write_str(key)?;
                w.write_str("=\"")?;
                w.write_str(value)?;
                w.write_str("\"")?;
            }
            w.write_str(">")?;
            if !el.inner_html.is_empty() {
                w.write_str(&el.inner_html)?;
            } else {
                for child in &el.children {
                    write_node_html(child, w)?;
                }
            }
            w.write_str("</")?;
            w.write_str(&el.tag)?;
            w.write_str(">")?;
        }
        Node::Text(text) => write_escaped_text(text, w)?,
        Node::Fragment(nodes) => {
            for node in nodes {
                write_node_html(node, w)?;
            }
        }
        Node::Comment(comment) => {
            w.write_str("<!--")?;
            w.write_str(comment)?;
            w.write_str("-->")?;
        }
        Node::Static(html) => w.write_str(html)?,
        Node::Empty => {}
    }
    Ok(())
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
    pub key: String,
    pub tag: Cow<'static, str>,
    pub attributes: Vec<(String, String)>,
    pub inner_html: String,
    pub children: Vec<Node>,
    #[cfg(feature = "wasm")]
    pub events: Vec<(String, EventCallback)>,
    #[cfg(not(feature = "wasm"))]
    #[allow(unused)]
    pub events: Vec<(String, String)>,
}

impl Element {
    pub fn parse_tag_with_attributes(
        key: &str,
        tag: &'static str,
        attributes: Vec<(String, String)>,
        #[cfg(feature = "wasm")] events: Vec<(String, EventCallback)>,
        #[cfg(not(feature = "wasm"))] events: Vec<(String, String)>,
        inner_html: &str,
        children: Vec<Node>,
    ) -> Node {
        Node::Element(Element {
            tag: Cow::Borrowed(tag),
            key: if key.is_empty() {
                String::new()
            } else {
                key.to_string()
            },
            attributes,
            events,
            children,
            inner_html: if inner_html.is_empty() {
                String::new()
            } else {
                inner_html.to_string()
            },
        })
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn attributes(&self) -> &[(String, String)] {
        &self.attributes
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn html(&self) -> &String {
        &self.inner_html
    }

    #[cfg(not(feature = "wasm"))]
    pub fn events(&self) -> &[(String, String)] {
        &self.events
    }

    #[cfg(feature = "wasm")]
    pub fn events(&self) -> &[(String, EventCallback)] {
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

    /// Renders the component by taking ownership of props (avoids cloning children).
    /// Default implementation delegates to `render(&props)`.
    fn render_owned(props: Self::Props) -> Node
    where
        Self::Props: Sized,
    {
        Self::render(&props)
    }
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
        let mut buffer = String::with_capacity(capacity);
        write_node_html(self, &mut buffer).unwrap();
        buffer
    }

    /// Writes this node's HTML to an existing HtmlWriter.
    pub fn write_to(&self, writer: &mut HtmlWriter) {
        write_node_html(self, &mut writer.buffer).unwrap();
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
                } else {
                    // Children
                    for child in &el.children {
                        size += child.estimate_html_size();
                    }
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
        write_node_html(self, f)
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{borrow::Cow, string::String, vec};

    #[test]
    fn inner_html_takes_precedence_over_children_in_html_output() {
        let node = Node::Element(Element {
            key: String::new(),
            tag: Cow::Borrowed("div"),
            attributes: Vec::new(),
            inner_html: "<strong>unsafe</strong>".to_string(),
            children: vec![Node::Text("child".to_string())],
            #[cfg(feature = "wasm")]
            events: Vec::new(),
            #[cfg(not(feature = "wasm"))]
            events: Vec::new(),
        });

        assert_eq!(node.to_html(), "<div><strong>unsafe</strong></div>");
    }
}
