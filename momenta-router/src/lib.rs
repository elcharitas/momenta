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
use web_sys::Event;

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
    current_path: Signal<String>,
}

impl RouterContext {
    pub fn new(mode: RouterMode) -> Self {
        let current_path = create_signal(Self::get_initial_path(mode));
        Self::setup_listener(mode, current_path);
        Self { mode, current_path }
    }
    pub fn with_path(mode: RouterMode, path: &'static str) -> Self {
        let current_path = create_signal(Self::normalize_path(path));
        Self::setup_listener(mode, current_path);
        Self { mode, current_path }
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

    fn get_initial_path(mode: RouterMode) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let location = window.location();

            let raw_path = match mode {
                RouterMode::Hash => location.hash().unwrap_or_default().to_string(),
                RouterMode::Pathname => location.pathname().unwrap_or_default().to_string(),
            };

            return Self::normalize_path(&raw_path);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = mode;
            "/".to_string()
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn setup_listener(mode: RouterMode, current_path: Signal<String>) {
        let window = web_sys::window().unwrap();

        let closure = Closure::wrap(alloc::boxed::Box::new(move |_event: Event| {
            let new_path = Self::get_initial_path(mode);
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
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn setup_listener(_mode: RouterMode, _current_path: Signal<String>) {}

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
                    history
                        .push_state_with_url(&JsValue::NULL, "", Some(&normalized_path))
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
}

#[cfg(test)]
mod tests {
    use super::RouterContext;
    use crate::RouterMode;
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
