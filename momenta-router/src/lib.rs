#![no_std]

extern crate alloc;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use momenta::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Event, MouseEvent};

pub use matchit::Router;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RouterMode {
    Hash,
    Pathname,
}

#[derive(Clone, Copy)]
pub struct RouterContext {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    mode: RouterMode,
    base_path: &'static str,
    current_path: Signal<String>,
}

impl RouterContext {
    pub fn new(mode: RouterMode) -> Self {
        Self::with_base(mode, "")
    }

    pub fn with_base(mode: RouterMode, base_path: &'static str) -> Self {
        let normalized_base = Self::normalize_base_path(base_path);
        let current_path = create_signal(Self::get_initial_path(mode, &normalized_base));
        Self::setup_listener(mode, current_path, normalized_base);
        Self {
            mode,
            base_path: normalized_base,
            current_path,
        }
    }

    pub fn with_path(mode: RouterMode, path: &'static str) -> Self {
        Self::with_base_and_path(mode, "", path)
    }

    pub fn with_base_and_path(
        mode: RouterMode,
        base_path: &'static str,
        path: &'static str,
    ) -> Self {
        let normalized_base = Self::normalize_base_path(base_path);
        let current_path = create_signal(Self::normalize_path(path));
        Self::setup_listener(mode, current_path, normalized_base);
        Self {
            mode,
            base_path: normalized_base,
            current_path,
        }
    }

    fn normalize_path(path: &str) -> String {
        let normalized = path.trim_start_matches('#');

        if normalized.is_empty() {
            "/".to_string()
        } else if normalized.starts_with('/') {
            normalized.to_string()
        } else {
            format!("/{}", normalized)
        }
    }

    fn normalize_base_path(base_path: &'static str) -> &'static str {
        let trimmed = base_path.trim();

        if trimmed.is_empty() || trimmed == "/" {
            ""
        } else {
            debug_assert!(
                trimmed.starts_with('/'),
                "RouterContext base paths must start with '/': {trimmed}"
            );

            trimmed.trim_end_matches('/')
        }
    }

    fn strip_base_path(path: &str, base_path: &str) -> String {
        let normalized_path = Self::normalize_path(path);

        if base_path.is_empty() {
            return normalized_path;
        }

        if normalized_path == base_path {
            return "/".to_string();
        }

        let prefix = format!("{}/", base_path);
        if normalized_path.starts_with(&prefix) {
            return format!(
                "/{}",
                normalized_path[prefix.len()..].trim_start_matches('/')
            );
        }

        normalized_path
    }

    fn join_base_path(base_path: &str, path: &str) -> String {
        let normalized_path = Self::normalize_path(path);

        if base_path.is_empty() {
            return normalized_path;
        }

        if normalized_path == "/" {
            format!("{}/", base_path)
        } else {
            format!("{}{}", base_path, normalized_path)
        }
    }

    fn get_initial_path(mode: RouterMode, base_path: &str) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let location = window.location();

            let raw_path = match mode {
                RouterMode::Hash => location.hash().unwrap_or_default().to_string(),
                RouterMode::Pathname => location.pathname().unwrap_or_default().to_string(),
            };

            return match mode {
                RouterMode::Hash => Self::normalize_path(&raw_path),
                RouterMode::Pathname => Self::strip_base_path(&raw_path, base_path),
            };
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (mode, base_path);
            "/".to_string()
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn setup_listener(mode: RouterMode, current_path: Signal<String>, base_path: &'static str) {
        let window = web_sys::window().unwrap();

        let closure = Closure::wrap(alloc::boxed::Box::new(move |_event: Event| {
            let new_path = Self::get_initial_path(mode, base_path);
            current_path.set(new_path);
        }) as alloc::boxed::Box<dyn FnMut(_)>);

        let event_name = match mode {
            RouterMode::Hash => "hashchange",
            RouterMode::Pathname => "popstate",
        };

        window
            .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();

        if mode == RouterMode::Pathname {
            Self::setup_pathname_click_listener(current_path, base_path);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn setup_pathname_click_listener(current_path: Signal<String>, base_path: &'static str) {
        let document = web_sys::window().unwrap().document().unwrap();

        let closure = Closure::wrap(alloc::boxed::Box::new(move |event: Event| {
            let Some(path) = Self::pathname_from_click_event(&event, &base_path) else {
                return;
            };

            let window = web_sys::window().unwrap();
            let history = window.history().unwrap();
            let external_path = Self::join_base_path(&base_path, &path);

            history
                .push_state_with_url(&JsValue::NULL, "", Some(&external_path))
                .unwrap();

            current_path.set(path);
        }) as alloc::boxed::Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();
    }

    #[cfg(target_arch = "wasm32")]
    fn pathname_from_click_event(event: &Event, base_path: &str) -> Option<String> {
        let mouse_event = event.dyn_ref::<MouseEvent>()?;

        if !Self::is_unmodified_primary_click(event, mouse_event) {
            return None;
        }

        let target = event.target()?;
        let element = target.dyn_ref::<web_sys::Element>().cloned().or_else(|| {
            target
                .dyn_ref::<web_sys::Node>()
                .and_then(|node| node.parent_element())
        })?;

        let anchor = element
            .closest("a[href]")
            .ok()
            .flatten()?
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .ok()?;

        let origin = web_sys::window()?.location().origin().ok()?;
        let path = Self::pathname_from_anchor_click(
            &anchor.get_attribute("href").unwrap_or_default(),
            &anchor.pathname(),
            base_path,
            anchor.origin() == origin,
            &anchor.target(),
            anchor.has_attribute("download"),
        )?;

        event.prevent_default();
        Some(path)
    }

    #[cfg(target_arch = "wasm32")]
    fn is_unmodified_primary_click(event: &Event, mouse_event: &MouseEvent) -> bool {
        !event.default_prevented()
            && mouse_event.button() == 0
            && !mouse_event.meta_key()
            && !mouse_event.ctrl_key()
            && !mouse_event.shift_key()
            && !mouse_event.alt_key()
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn pathname_from_anchor_click(
        href_attr: &str,
        pathname: &str,
        base_path: &str,
        same_origin: bool,
        target: &str,
        has_download: bool,
    ) -> Option<String> {
        if !same_origin || has_download {
            return None;
        }

        if !target.is_empty() && target != "_self" {
            return None;
        }

        if href_attr.is_empty()
            || href_attr.starts_with('#')
            || href_attr.starts_with('?')
            || href_attr.contains('#')
            || href_attr.contains('?')
        {
            return None;
        }

        Some(Self::strip_base_path(pathname, base_path))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn setup_listener(_mode: RouterMode, _current_path: Signal<String>, _base_path: &'static str) {}

    pub fn navigate(&self, path: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let history = window.history().unwrap();
            let normalized_path = Self::normalize_path(path);

            match self.mode {
                RouterMode::Hash => {
                    let hash_path = format!("#{}", normalized_path);
                    window.location().set_hash(&hash_path).unwrap();
                    self.current_path.set(normalized_path);
                }
                RouterMode::Pathname => {
                    let external_path = Self::join_base_path(&self.base_path, &normalized_path);
                    history
                        .push_state_with_url(&JsValue::NULL, "", Some(&external_path))
                        .unwrap();
                    self.current_path.set(normalized_path);
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let normalized_path = Self::normalize_path(path);
            self.current_path.set(normalized_path);
        }
    }

    pub fn current_path(&self) -> Signal<String> {
        self.current_path
    }

    pub fn href(&self, path: &str) -> String {
        let normalized_path = Self::normalize_path(path);

        match self.mode {
            RouterMode::Hash => format!("#{}", normalized_path),
            RouterMode::Pathname => Self::join_base_path(&self.base_path, &normalized_path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RouterContext;
    use crate::RouterMode;
    use alloc::string::String;
    use momenta::prelude::Node;
    use momenta::signals::run_scope_transient;

    #[test]
    fn normalize_path_preserves_root() {
        assert_eq!(RouterContext::normalize_path(""), "/");
        assert_eq!(RouterContext::normalize_path("#"), "/");
        assert_eq!(RouterContext::normalize_path("/"), "/");
    }

    #[test]
    fn normalize_path_adds_missing_leading_slash() {
        assert_eq!(RouterContext::normalize_path("about"), "/about");
        assert_eq!(RouterContext::normalize_path("#/docs"), "/docs");
    }

    #[test]
    fn normalize_path_keeps_existing_leading_slash() {
        assert_eq!(RouterContext::normalize_path("/guide"), "/guide");
        assert_eq!(RouterContext::normalize_path("#/guide"), "/guide");
    }

    #[test]
    fn with_path_and_navigate_work_without_browser_runtime() {
        run_scope_transient(
            || {
                let router = RouterContext::with_path(RouterMode::Pathname, "/docs");

                assert_eq!(router.current_path().get(), "/docs");

                router.navigate("guide/getting-started");
                assert_eq!(router.current_path().get(), "/guide/getting-started");

                Node::Empty
            },
            |_| {},
        );
    }

    #[test]
    fn base_path_is_stripped_from_initial_paths() {
        assert_eq!(RouterContext::strip_base_path("/momenta", "/momenta"), "/");
        assert_eq!(RouterContext::strip_base_path("/momenta/", "/momenta"), "/");
        assert_eq!(
            RouterContext::strip_base_path("/momenta/getting-started", "/momenta"),
            "/getting-started"
        );
    }

    #[test]
    fn base_path_is_joined_for_external_urls() {
        assert_eq!(RouterContext::join_base_path("", "/signals"), "/signals");
        assert_eq!(RouterContext::join_base_path("/momenta", "/"), "/momenta/");
        assert_eq!(
            RouterContext::join_base_path("/momenta", "/signals"),
            "/momenta/signals"
        );
    }

    #[test]
    fn href_uses_base_path_for_pathname_mode() {
        run_scope_transient(
            || {
                let router =
                    RouterContext::with_base_and_path(RouterMode::Pathname, "/momenta", "/");

                assert_eq!(router.href("/"), "/momenta/");
                assert_eq!(router.href("/examples"), "/momenta/examples");

                Node::Empty
            },
            |_| {},
        );
    }

    #[test]
    fn pathname_click_intercepts_same_origin_internal_links() {
        assert_eq!(
            RouterContext::pathname_from_anchor_click("/guide", "/guide", "", true, "", false),
            Some(String::from("/guide"))
        );
        assert_eq!(
            RouterContext::pathname_from_anchor_click(
                "https://example.com/docs",
                "/docs",
                "",
                true,
                "_self",
                false,
            ),
            Some(String::from("/docs"))
        );
        assert_eq!(
            RouterContext::pathname_from_anchor_click(
                "/momenta/guide",
                "/momenta/guide",
                "/momenta",
                true,
                "",
                false,
            ),
            Some(String::from("/guide"))
        );
    }

    #[test]
    fn pathname_click_ignores_special_case_links() {
        assert_eq!(
            RouterContext::pathname_from_anchor_click("#section", "/guide", "", true, "", false),
            None
        );
        assert_eq!(
            RouterContext::pathname_from_anchor_click(
                "/guide?tab=api",
                "/guide",
                "",
                true,
                "",
                false
            ),
            None
        );
        assert_eq!(
            RouterContext::pathname_from_anchor_click("/guide#api", "/guide", "", true, "", false),
            None
        );
        assert_eq!(
            RouterContext::pathname_from_anchor_click("/guide", "/guide", "", false, "", false),
            None
        );
        assert_eq!(
            RouterContext::pathname_from_anchor_click(
                "/guide", "/guide", "", true, "_blank", false
            ),
            None
        );
        assert_eq!(
            RouterContext::pathname_from_anchor_click("/guide", "/guide", "", true, "", true),
            None
        );
    }
}

pub struct RouteMatch {
    pub params: Vec<(String, String)>,
}

impl RouteMatch {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

#[macro_export]
macro_rules! routes {
    (
        $router:expr,
        $path:expr,
        {
            $($pattern:literal => $handler:expr),* $(,)?
        }
    ) => {{
        use alloc::string::ToString;

        let mut matcher = $crate::Router::new();
        $(
            matcher.insert($pattern, $pattern).unwrap();
        )*

        let path = $path.get();
        let matched = matcher.at(&path);

        match matched {
            Ok(m) => {
                let route_pattern = *m.value;
                let params: alloc::vec::Vec<(alloc::string::String, alloc::string::String)> = m
                    .params
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();

                let route_match = $crate::RouteMatch { params };

                match route_pattern {
                    $(
                        $pattern => $handler(route_match),
                    )*
                    _ => momenta::nodes::Node::Empty,
                }
            }
            Err(_) => momenta::nodes::Node::Empty,
        }
    }};
}
