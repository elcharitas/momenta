//! Server-side rendering utilities for Momenta.
//!
//! This crate provides:
//! - Buffered HTML rendering
//! - Chunked HTML rendering
//! - Hydratable HTML output with stable element markers
//! - Embedded JSON state blobs for client-side resume
//! - Thin adapters for Axum, Actix, and Hyper

use core::fmt::{self, Write};
use momenta::{
    nodes::{Component, Element, Node},
    signals::run_scope_transient,
};

pub const HYDRATION_ID_ATTR: &str = "data-momenta-hid";
pub const HYDRATION_ROOT_ATTR: &str = "data-momenta-root";
pub const DEFAULT_HYDRATION_STATE_ID: &str = "__MOMENTA_HYDRATION__";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderOptions {
    pub chunk_size: usize,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            chunk_size: 8 * 1024,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HydrationOptions {
    pub render: RenderOptions,
    pub state_script_id: String,
    pub state_json: Option<String>,
}

impl Default for HydrationOptions {
    fn default() -> Self {
        Self {
            render: RenderOptions::default(),
            state_script_id: DEFAULT_HYDRATION_STATE_ID.to_string(),
            state_json: None,
        }
    }
}

pub fn render_to_string(render: impl FnOnce() -> Node + Send + 'static) -> String {
    let node = render_node(render);
    node.to_html()
}

pub fn render_to_string_with_options(
    render: impl FnOnce() -> Node + Send + 'static,
    options: RenderOptions,
) -> String {
    let mut html = String::new();
    render_to_writer(render, &mut html, options).expect("writing HTML to a string cannot fail");
    html
}

pub fn render_to_writer(
    render: impl FnOnce() -> Node + Send + 'static,
    writer: &mut impl Write,
    options: RenderOptions,
) -> fmt::Result {
    let node = render_node(render);
    write_node_to_writer(&node, writer, options)
}

pub fn render_to_chunks(
    render: impl FnOnce() -> Node + Send + 'static,
    options: RenderOptions,
) -> Vec<String> {
    let node = render_node(render);
    let mut collector = ChunkCollector::new(options.chunk_size);
    write_node_to_collector(&node, &mut collector);
    collector.finish()
}

pub fn render_to_hydration_string(
    render: impl FnOnce() -> Node + Send + 'static,
    options: HydrationOptions,
) -> String {
    let node = render_node(render);
    let mut collector = ChunkCollector::new(options.render.chunk_size);
    write_hydratable_root(&node, &mut collector);

    if let Some(state_json) = options.state_json.as_deref() {
        collector.push_str(&render_hydration_state_script(
            &options.state_script_id,
            state_json,
        ));
    }

    collector.finish().concat()
}

pub fn render_hydration_state_script(script_id: &str, state_json: &str) -> String {
    let mut output = String::new();
    output.push_str("<script id=\"");
    output.push_str(script_id);
    output.push_str("\" type=\"application/json\">");
    output.push_str(&escape_script_content(state_json));
    output.push_str("</script>");
    output
}

pub fn render_component_to_string<C: Component>(props: C::Props) -> String
where
    C::Props: Send + Sync + 'static,
{
    render_to_string(move || C::render(&props))
}

pub fn render_component_to_chunks<C: Component>(
    props: C::Props,
    options: RenderOptions,
) -> Vec<String>
where
    C::Props: Send + Sync + 'static,
{
    render_to_chunks(move || C::render(&props), options)
}

pub fn render_component_to_hydration_string<C: Component>(
    props: C::Props,
    options: HydrationOptions,
) -> String
where
    C::Props: Send + Sync + 'static,
{
    render_to_hydration_string(move || C::render(&props), options)
}

#[cfg(feature = "axum")]
pub fn axum_html(render: impl FnOnce() -> Node + Send + 'static) -> axum::response::Html<String> {
    axum::response::Html(render_to_string(render))
}

#[cfg(feature = "axum")]
pub fn axum_stream(
    render: impl FnOnce() -> Node + Send + 'static,
    options: RenderOptions,
) -> axum::response::Response {
    use axum::{
        body::Body,
        http::{Response, header},
    };
    use bytes::Bytes;
    use core::convert::Infallible;
    use futures_util::stream;

    let chunks = render_to_chunks(render, options);
    let stream = stream::iter(
        chunks
            .into_iter()
            .map(|chunk| Ok::<Bytes, Infallible>(Bytes::from(chunk))),
    );

    Response::builder()
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from_stream(stream))
        .expect("failed to build axum response")
}

#[cfg(feature = "actix")]
pub fn actix_html(render: impl FnOnce() -> Node + Send + 'static) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_to_string(render))
}

#[cfg(feature = "actix")]
pub fn actix_stream(
    render: impl FnOnce() -> Node + Send + 'static,
    options: RenderOptions,
) -> actix_web::HttpResponse {
    use actix_web::web::Bytes;
    use core::convert::Infallible;
    use futures_util::stream;

    let chunks = render_to_chunks(render, options);
    let stream = stream::iter(
        chunks
            .into_iter()
            .map(|chunk| Ok::<Bytes, Infallible>(Bytes::from(chunk))),
    );

    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .streaming(stream)
}

#[cfg(feature = "hyper")]
pub fn hyper_html(
    render: impl FnOnce() -> Node + Send + 'static,
) -> http::Response<http_body_util::Full<bytes::Bytes>> {
    use bytes::Bytes;
    use http::header;
    use http_body_util::Full;

    http::Response::builder()
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Full::new(Bytes::from(render_to_string(render))))
        .expect("failed to build hyper response")
}

#[cfg(feature = "hyper")]
pub fn hyper_stream(
    render: impl FnOnce() -> Node + Send + 'static,
    options: RenderOptions,
) -> http::Response<http_body_util::StreamBody<HyperChunkStream>> {
    use bytes::Bytes;
    use core::convert::Infallible;
    use futures_util::stream;
    use http::header;
    use http_body_util::StreamBody;
    use hyper::body::Frame;

    let chunks = render_to_chunks(render, options);
    let stream = stream::iter(
        chunks
            .into_iter()
            .map(|chunk| Ok::<Frame<Bytes>, Infallible>(Frame::data(Bytes::from(chunk)))),
    );

    http::Response::builder()
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(StreamBody::new(stream))
        .expect("failed to build hyper stream response")
}

#[cfg(feature = "hyper")]
type HyperChunkStream = futures_util::stream::Iter<
    std::vec::IntoIter<Result<hyper::body::Frame<bytes::Bytes>, core::convert::Infallible>>,
>;

fn render_node(render: impl FnOnce() -> Node + Send + 'static) -> Node {
    let mut render = Some(render);

    run_scope_transient(
        move || render.take().expect("render closure should only run once")(),
        |_| {},
    )
}

fn write_node_to_writer(
    node: &Node,
    writer: &mut impl Write,
    options: RenderOptions,
) -> fmt::Result {
    for chunk in render_existing_node_to_chunks(node, options) {
        writer.write_str(&chunk)?;
    }

    Ok(())
}

fn render_existing_node_to_chunks(node: &Node, options: RenderOptions) -> Vec<String> {
    let mut collector = ChunkCollector::new(options.chunk_size);
    write_node_to_collector(node, &mut collector);
    collector.finish()
}

fn write_hydratable_root(node: &Node, collector: &mut ChunkCollector) {
    let mut root_nodes = Vec::new();
    collect_materialized_nodes(node, &mut root_nodes);

    for (index, child) in root_nodes.into_iter().enumerate() {
        write_node_to_collector_hydratable(child, collector, &index.to_string(), true);
    }
}

fn write_node_to_collector(node: &Node, collector: &mut ChunkCollector) {
    match node {
        Node::Element(element) => {
            write_element_open_tag(element, collector);

            if !element.html().is_empty() {
                collector.push_str(element.html());
            } else {
                for child in element.children() {
                    write_node_to_collector(child, collector);
                }
            }

            write_element_close_tag(element, collector);
        }
        Node::Text(text) => write_escaped_text(text, collector),
        Node::Fragment(children) => {
            for child in children {
                write_node_to_collector(child, collector);
            }
        }
        Node::Comment(comment) => {
            collector.push_str("<!--");
            collector.push_str(comment);
            collector.push_str("-->");
        }
        Node::Static(html) => collector.push_str(html),
        Node::Empty => {}
    }
}

fn write_node_to_collector_hydratable(
    node: &Node,
    collector: &mut ChunkCollector,
    path: &str,
    is_root: bool,
) {
    match node {
        Node::Element(element) => {
            write_element_open_tag_hydratable(element, collector, path, is_root);

            if !element.html().is_empty() {
                collector.push_str(element.html());
            } else {
                write_hydratable_children(element.children(), collector, path);
            }

            write_element_close_tag(element, collector);
        }
        Node::Text(text) => write_escaped_text(text, collector),
        Node::Fragment(children) => {
            let mut materialized = Vec::new();
            collect_materialized_children(children, &mut materialized);
            for (index, child) in materialized.into_iter().enumerate() {
                let child_path = format!("{}.{}", path, index);
                write_node_to_collector_hydratable(child, collector, &child_path, false);
            }
        }
        Node::Comment(comment) => {
            collector.push_str("<!--");
            collector.push_str(comment);
            collector.push_str("-->");
        }
        Node::Static(html) => collector.push_str(html),
        Node::Empty => {}
    }
}

fn write_hydratable_children(children: &[Node], collector: &mut ChunkCollector, parent_path: &str) {
    let mut materialized = Vec::new();
    collect_materialized_children(children, &mut materialized);

    for (index, child) in materialized.into_iter().enumerate() {
        let child_path = format!("{}.{}", parent_path, index);
        write_node_to_collector_hydratable(child, collector, &child_path, false);
    }
}

fn write_element_open_tag(element: &Element, collector: &mut ChunkCollector) {
    collector.push_str("<");
    collector.push_str(element.tag());

    for (key, value) in element.attributes() {
        write_attribute(collector, key, value);
    }

    collector.push_str(">");
}

fn write_element_open_tag_hydratable(
    element: &Element,
    collector: &mut ChunkCollector,
    path: &str,
    is_root: bool,
) {
    collector.push_str("<");
    collector.push_str(element.tag());
    write_attribute(collector, HYDRATION_ID_ATTR, path);

    if is_root {
        write_attribute(collector, HYDRATION_ROOT_ATTR, "true");
    }

    for (key, value) in element.attributes() {
        write_attribute(collector, key, value);
    }

    collector.push_str(">");
}

fn write_element_close_tag(element: &Element, collector: &mut ChunkCollector) {
    collector.push_str("</");
    collector.push_str(element.tag());
    collector.push_str(">");
}

fn write_attribute(collector: &mut ChunkCollector, key: &str, value: &str) {
    collector.push_str(" ");
    collector.push_str(key);
    collector.push_str("=\"");
    collector.push_str(value);
    collector.push_str("\"");
}

fn collect_materialized_children<'a>(children: &'a [Node], out: &mut Vec<&'a Node>) {
    for child in children {
        collect_materialized_nodes(child, out);
    }
}

fn collect_materialized_nodes<'a>(node: &'a Node, out: &mut Vec<&'a Node>) {
    match node {
        Node::Fragment(children) => collect_materialized_children(children, out),
        Node::Empty => {}
        _ => out.push(node),
    }
}

fn write_escaped_text(text: &str, collector: &mut ChunkCollector) {
    let needs_escape = text
        .bytes()
        .any(|b| matches!(b, b'<' | b'>' | b'&' | b'"' | b'/'));

    if !needs_escape {
        collector.push_str(text);
        return;
    }

    for ch in text.chars() {
        match ch {
            '<' => collector.push_str("&lt;"),
            '>' => collector.push_str("&gt;"),
            '&' => collector.push_str("&amp;"),
            '"' => collector.push_str("&quot;"),
            '/' => collector.push_str("&#x2F;"),
            _ => collector.push_char(ch),
        }
    }
}

fn escape_script_content(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());

    for ch in text.chars() {
        match ch {
            '<' => escaped.push_str("\\u003c"),
            '>' => escaped.push_str("\\u003e"),
            '&' => escaped.push_str("\\u0026"),
            '\u{2028}' => escaped.push_str("\\u2028"),
            '\u{2029}' => escaped.push_str("\\u2029"),
            _ => escaped.push(ch),
        }
    }

    escaped
}

struct ChunkCollector {
    chunk_size: usize,
    current: String,
    chunks: Vec<String>,
}

impl ChunkCollector {
    fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size: chunk_size.max(1),
            current: String::new(),
            chunks: Vec::new(),
        }
    }

    fn push_char(&mut self, ch: char) {
        self.current.push(ch);
        self.flush_if_needed();
    }

    fn push_str(&mut self, value: &str) {
        self.current.push_str(value);
        self.flush_if_needed();
    }

    fn flush_if_needed(&mut self) {
        if self.current.len() >= self.chunk_size {
            self.flush();
        }
    }

    fn flush(&mut self) {
        if !self.current.is_empty() {
            self.chunks.push(core::mem::take(&mut self.current));
        }
    }

    fn finish(mut self) -> Vec<String> {
        self.flush();
        self.chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use momenta::{nodes::Element, prelude::*};
    use std::{string::String, vec};

    fn element(tag: &'static str, children: Vec<Node>) -> Node {
        Element::parse_tag_with_attributes("", tag, Vec::new(), Vec::new(), "", children)
    }

    #[test]
    fn render_to_string_isolated_runtime_works() {
        let html = render_to_string(|| {
            let count = create_signal(3);
            element("div", vec![Node::from(count.get())])
        });

        assert_eq!(html, "<div>3</div>");
    }

    #[test]
    fn render_to_chunks_splits_output() {
        let chunks = render_to_chunks(
            || {
                element(
                    "div",
                    vec![
                        element("span", vec![Node::from("Hello")]),
                        element("span", vec![Node::from("World")]),
                    ],
                )
            },
            RenderOptions { chunk_size: 8 },
        );

        assert!(chunks.len() > 1);
        assert_eq!(
            chunks.concat(),
            "<div><span>Hello</span><span>World</span></div>"
        );
    }

    #[test]
    fn render_to_writer_escapes_text() {
        let mut output = String::new();
        render_to_writer(
            || element("p", vec![Node::from("<unsafe>")]),
            &mut output,
            RenderOptions::default(),
        )
        .unwrap();

        assert_eq!(output, "<p>&lt;unsafe&gt;</p>");
    }

    #[test]
    fn render_to_hydration_string_includes_markers_and_state() {
        let html = render_to_hydration_string(
            || {
                element(
                    "div",
                    vec![
                        element("span", vec![Node::from("Hello")]),
                        Node::from("World"),
                    ],
                )
            },
            HydrationOptions {
                state_json: Some("{\"count\":1}".to_string()),
                ..HydrationOptions::default()
            },
        );

        assert!(html.contains("data-momenta-hid=\"0\""));
        assert!(html.contains("data-momenta-root=\"true\""));
        assert!(html.contains("data-momenta-hid=\"0.0\""));
        assert!(html.contains(
            "id=\"__MOMENTA_HYDRATION__\" type=\"application/json\">{\"count\":1}</script>"
        ));
    }

    #[test]
    fn hydration_state_script_escapes_script_breakouts() {
        let script = render_hydration_state_script("state", "</script><div>");

        assert_eq!(
            script,
            "<script id=\"state\" type=\"application/json\">\\u003c/script\\u003e\\u003cdiv\\u003e</script>"
        );
    }
}
