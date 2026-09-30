#![no_std]

use momenta::prelude::*;
use momenta_router::Router;

#[derive(Clone, PartialEq, SignalValue)]
struct Status(u8);

#[component]
fn Counter() -> Node {
    let count = create_signal(1_u32);
    let status = create_signal(Status(2));
    rsx!(<button class="count" data_count={count.get()} data_status={status.get().0}>{count}</button>)
}

pub fn render() -> Node {
    rsx!(<main id="app"><h1>"Hello"</h1><Counter /></main>)
}

pub fn render_comment() -> Node {
    rsx!(<!-- Embedded comment -->)
}

pub fn render_fragment() -> Node {
    rsx!(<><span>"A"</span><span>"B"</span></>)
}

pub fn route_matches() -> bool {
    let mut router = Router::new();
    router.insert("/items/{id}", 7_u8).unwrap();
    router
        .at("/items/42")
        .map(|matched| *matched.value == 7 && matched.params.get("id") == Some("42"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::{render, render_comment, render_fragment, route_matches};

    #[test]
    fn renders_markup_and_matches_routes() {
        let html = render().to_html();
        assert_eq!(
            html,
            "<main id=\"app\"><h1>Hello</h1><button class=\"count\" data-count=\"1\" data-status=\"2\">1</button></main>"
        );
        assert_eq!(render_comment().to_html(), "<!-- Embedded comment -->");
        assert_eq!(render_fragment().to_html(), "<span>A</span><span>B</span>");
        assert!(route_matches());
    }
}
