use alloc::{format, string::String, vec, vec::Vec};
use momenta::prelude::*;
use momenta_router::RouterContext;
use wasm_bindgen::JsCast;

pub static GITHUB_LINK: &str = "https://github.com/elcharitas/momenta";
pub static CRATES_LINK: &str = "https://crates.io/crates/momenta";

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    pub fn highlightAll();
}

pub fn toggle_dark_mode(theme: Signal<&'static str>) {
    let is_dark = theme.get() == "dark";
    let new_theme = if is_dark { "light" } else { "dark" };
    theme.set(new_theme);
    if let Some(window) = web_sys::window() {
        if let Some(doc) = window.document() {
            if let Some(el) = doc.document_element() {
                let cl = el.class_list();
                if new_theme == "dark" {
                    let _ = cl.add_1("dark");
                } else {
                    let _ = cl.remove_1("dark");
                }
            }
        }
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("theme", new_theme);
        }
    }
}

pub struct HeaderProps {
    pub theme: Signal<&'static str>,
    pub mobile_menu_open: Signal<bool>,
}

#[component]
pub fn Header(props: &HeaderProps) -> Node {
    let theme = props.theme;
    let mobile_menu_open = props.mobile_menu_open;
    let search_open = create_signal(false);
    let query = create_signal(String::new());

    let toggle_theme = move |_| {
        toggle_dark_mode(theme);
    };

    let nav_items: Vec<(&str, &str, &str)> = vec![
        (
            "/getting-started",
            "Getting Started",
            "Install Momenta and build your first app",
        ),
        (
            "/philosophy",
            "Philosophy",
            "Design principles behind Momenta",
        ),
        ("/rsx", "rsx!", "JSX-like syntax macro for building UI"),
        ("/signals", "Signals", "Fine-grained reactive primitives"),
        (
            "/computed-signals",
            "Computed Signals",
            "Derived reactive state",
        ),
        (
            "/effects",
            "Effects",
            "Side effects and reactive subscriptions",
        ),
        ("/resources", "Resources", "Async data loading primitives"),
        ("/components", "Components", "Reusable composable UI pieces"),
        (
            "/classes",
            "Dynamic Classes",
            "Conditional CSS class composition",
        ),
        (
            "/when",
            "Conditional Rendering",
            "when! macro for control flow",
        ),
        (
            "/lists",
            "List Rendering",
            "Efficient list rendering with iterators",
        ),
        (
            "/routing",
            "Routing",
            "Client-side routing with momenta-router",
        ),
        ("/performance", "Performance", "Optimization guide"),
        ("/deployment", "Deployment", "Deploy your Momenta app"),
        ("/examples", "Examples", "Browse all example apps"),
        ("/examples/counter", "Counter", "Simple counter example"),
        ("/examples/todomvc", "TodoMVC", "TodoMVC implementation"),
        ("/examples/hackernews", "Hacker News", "Hacker News client"),
        (
            "/examples/realworld",
            "RealWorld",
            "Full-stack blog platform",
        ),
    ];

    let modal_class = if search_open.get() {
        "fixed inset-0 z-50 flex items-start justify-center pt-[15vh]"
    } else {
        "hidden"
    };

    rsx! {
        <div>
        <header class="fixed inset-x-0 top-0 z-40 w-full border-b border-border/50 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/80">
            <div class="flex h-14 items-center px-4 sm:px-6 lg:px-8">
                <button
                    class="lg:hidden p-2 -ml-2 rounded-md hover:bg-muted transition-colors"
                    on:click={move |_| mobile_menu_open.set(!mobile_menu_open)}
                >
                    <i class="fas fa-bars text-sm"></i>
                </button>

                <a href="/" class="flex items-center gap-2 ml-2 lg:ml-0">
                    <img src="./static/icon.svg" alt="Momenta" class="w-6 h-6" />
                    <span class="font-semibold text-[15px]">Momenta</span>
                </a>

                <nav class="hidden md:flex items-center gap-0.5 ml-8">
                    <a href="/getting-started" class="px-3 py-1.5 text-[13px] font-medium rounded-md transition-colors hover:bg-muted text-muted-foreground hover:text-foreground">
                        Docs
                    </a>
                    <a href="/signals" class="px-3 py-1.5 text-[13px] font-medium rounded-md transition-colors hover:bg-muted text-muted-foreground hover:text-foreground">
                        Signals
                    </a>
                    <a href="/examples" class="px-3 py-1.5 text-[13px] font-medium rounded-md transition-colors hover:bg-muted text-muted-foreground hover:text-foreground">
                        Examples
                    </a>
                </nav>

                <button
                    on:click={move |_| search_open.set(true)}
                    class="hidden sm:flex items-center gap-2 ml-auto mr-2 px-3 py-1.5 rounded-lg border border-border bg-muted/50 hover:bg-muted text-muted-foreground text-[13px] transition-colors w-56"
                >
                    <i class="fas fa-search text-xs"></i>
                    <span class="flex-1 text-left">Search docs...</span>
                    <kbd class="hidden lg:inline-flex items-center gap-0.5 rounded border border-border bg-background px-1.5 py-0.5 text-[10px] font-mono text-muted-foreground">
                        <span>"⌘"</span>K
                    </kbd>
                </button>

                <div class={format!("{} flex items-center gap-0.5", if search_open.get() == false { "ml-auto sm:ml-0" } else { "sm:ml-0" })}>
                    <button
                        on:click={move |_| search_open.set(true)}
                        class="sm:hidden p-2 rounded-md hover:bg-muted transition-colors text-muted-foreground hover:text-foreground"
                    >
                        <i class="fas fa-search text-sm"></i>
                    </button>

                    <button
                        on:click={toggle_theme}
                        class="p-2 rounded-md hover:bg-muted transition-colors text-muted-foreground hover:text-foreground"
                    >
                        {when!(theme == "dark" =>
                            <i class="fas fa-sun text-sm"></i>
                        else
                            <i class="fas fa-moon text-sm"></i>
                        )}
                    </button>

                    <a href={GITHUB_LINK}
                       class="p-2 rounded-md hover:bg-muted transition-colors text-muted-foreground hover:text-foreground">
                        <i class="fab fa-github text-sm"></i>
                    </a>
                </div>
            </div>
        </header>

            <div class={modal_class}>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm" on:click={move |_| { search_open.set(false); query.set(String::new()); }}></div>
                <div class="relative w-full max-w-lg mx-4 bg-background border border-border rounded-xl shadow-2xl overflow-hidden fade-in">
                    <div class="flex items-center gap-3 px-4 py-3 border-b border-border">
                        <i class="fas fa-search text-sm text-muted-foreground"></i>
                        <input
                            type="text"
                            placeholder="Search documentation..."
                            class="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
                            value={query.get()}
                            on:input={move |e: web_sys::Event| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                    query.set(input.value());
                                }
                            }}
                        />
                        <button on:click={move |_| { search_open.set(false); query.set(String::new()); }} class="p-1 rounded-md hover:bg-muted transition-colors text-muted-foreground hover:text-foreground">
                            <i class="fas fa-times text-sm"></i>
                        </button>
                    </div>
                    <div class="max-h-80 overflow-y-auto p-2">
                        {{
                            let q = query.get().to_lowercase();
                            let filtered: Vec<_> = nav_items.iter()
                                .filter(|(_, title, desc)| {
                                    q.is_empty() || title.to_lowercase().contains(&q) || desc.to_lowercase().contains(&q)
                                })
                                .collect();

                            if filtered.is_empty() {
                                vec![rsx! {
                                    <div class="px-3 py-6 text-center text-sm text-muted-foreground">
                                        "No results found."
                                    </div>
                                }]
                            } else {
                                filtered.iter().map(|(path, title, desc)| {
                                    let path_owned = format!("{}", path);
                                    rsx! {
                                        <a href={path_owned} on:click={move |_| { search_open.set(false); query.set(String::new()); }} class="flex items-center gap-3 px-3 py-2.5 rounded-lg hover:bg-muted transition-colors group">
                                            <i class="fas fa-file-alt text-xs text-muted-foreground group-hover:text-primary"></i>
                                            <div>
                                                <div class="text-sm font-medium">{*title}</div>
                                                <div class="text-xs text-muted-foreground">{*desc}</div>
                                            </div>
                                        </a>
                                    }
                                }).collect::<Vec<_>>()
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

pub struct NavigationProps {
    pub router: RouterContext,
}

#[component]
pub fn Navigation(props: &NavigationProps) -> Node {
    let current_path = props.router.current_path();

    let nav_link = move |path: &'static str, label: &'static str| {
        let is_active = current_path.get() == path;
        let class = if is_active {
            "flex items-center px-3 py-1.5 text-[13px] font-medium text-primary border-l-2 border-primary -ml-px transition-colors"
        } else {
            "flex items-center px-3 py-1.5 text-[13px] text-muted-foreground hover:text-foreground border-l-2 border-transparent -ml-px transition-colors"
        };

        rsx! {
            <a href={path} class={class}>
                {label}
            </a>
        }
    };

    let section = move |title: &'static str, children: Vec<Node>| {
        rsx! {
            <div class="mb-6">
                <h5 class="mb-2 px-3 text-[11px] font-semibold uppercase tracking-[0.08em] text-muted-foreground/60">
                    {title}
                </h5>
                <div class="border-l border-border/50">
                    {children}
                </div>
            </div>
        }
    };

    rsx! {
        <nav class="px-2">
            {section("Start Here", vec![
                nav_link("/getting-started", "Getting Started"),
                nav_link("/philosophy", "Philosophy"),
            ])}

            {section("Macros", vec![
                nav_link("/rsx", "rsx!"),
                nav_link("/components", "#[component]"),
                nav_link("/classes", "class!"),
            ])}

            {section("Reactive Primitives", vec![
                nav_link("/signals", "create_signal"),
                nav_link("/computed-signals", "create_computed"),
                nav_link("/effects", "create_effect"),
                nav_link("/resources", "create_resource"),
            ])}

            {section("Control Flow", vec![
                nav_link("/when", "when!"),
                nav_link("/lists", ".iter().map()"),
            ])}

            {section("Guides", vec![
                nav_link("/routing", "Routing"),
                nav_link("/performance", "Performance"),
                nav_link("/ssr", "SSR & Hydration"),
                nav_link("/deployment", "Deployment"),
            ])}

            {section("Examples", vec![
                nav_link("/examples", "All Examples"),
                nav_link("/examples/counter", "Counter"),
                nav_link("/examples/todomvc", "TodoMVC"),
                nav_link("/examples/hackernews", "Hacker News"),
                nav_link("/examples/realworld", "RealWorld"),
            ])}
        </nav>
    }
}

pub fn docs_on_this_page_sections(path: &str) -> Vec<(&'static str, &'static str)> {
    if path.starts_with("/routing") {
        return vec![
            ("introduction", "Introduction"),
            ("setup", "Setup"),
            ("router-context", "RouterContext"),
            ("router-modes", "Router Modes"),
            ("dynamic-routes", "Dynamic Routes"),
            ("programmatic-navigation", "Programmatic Navigation"),
            ("active-links", "Active Links"),
            ("route-match", "RouteMatch API"),
            ("best-practices", "Best Practices"),
        ];
    }

    match path {
        "/getting-started" => vec![
            ("prerequisites", "Prerequisites"),
            ("installation", "Installation"),
            ("create-index-html", "Create index.html"),
            ("first-component", "Your First Component"),
            ("project-structure", "Project Structure"),
            ("run-your-app", "Run Your App"),
            ("next-steps", "Next Steps"),
        ],
        "/philosophy" => vec![
            ("mental-model", "Mental Model"),
            ("core-principles", "Core Principles"),
            ("why-not-virtual-dom", "Why Not Virtual DOM?"),
            ("framework-comparison", "Comparison"),
        ],
        "/rsx" => vec![
            ("introduction", "Introduction"),
            ("basic-example", "Basic Example"),
            ("api-reference", "API Reference"),
            ("creating-elements", "Creating Elements"),
            ("attributes", "Dynamic Attributes"),
            ("children", "Dynamic Children"),
            ("fragments", "Fragments"),
        ],
        "/signals" => vec![
            ("introduction", "Introduction"),
            ("basic-example", "Basic Example"),
            ("api-reference", "API Reference"),
            ("creating-signals", "Creating Signals"),
            ("reading-values", "Reading Values"),
            ("updating-values", "Updating Values"),
            ("operator-overloads", "Operator Overloads"),
            ("with-method", "The .with() Method"),
            ("boolean-signals", "Boolean Signals"),
            ("then-method", "The .then() Method"),
            ("vec-signals", "Vector Signals"),
            ("signal-value", "SignalValue"),
        ],
        "/computed-signals" => vec![
            ("introduction", "Introduction"),
            ("create-computed", "create_computed"),
            ("derive-method", "derive()"),
            ("memoization", "Memoization"),
            ("comparison", "When to Use Each"),
            ("best-practices", "Best Practices"),
        ],
        "/effects" => vec![
            ("introduction", "Introduction"),
            ("basic-example", "Basic Example"),
            ("api-reference", "API Reference"),
            ("creating-effects", "Creating Effects"),
            ("advanced-patterns", "Advanced Patterns"),
            ("best-practices", "Best Practices"),
        ],
        "/resources" => vec![
            ("introduction", "Introduction"),
            ("basic-usage", "Basic Usage"),
            ("creating-resources", "Creating Resources"),
            ("resource-states", "Resource States"),
            ("retrying-resources", "Retrying Resources"),
            ("reactive-dependencies", "Reactive Dependencies"),
            ("reading-without-cloning", "Reading Without Cloning"),
            ("combining-with-effects", "Combining with Effects"),
            ("best-practices", "Best Practices"),
        ],
        "/components" => vec![
            ("introduction", "Introduction"),
            ("basic-component", "Basic Component"),
            ("components-with-props", "Components with Props"),
            ("components-with-state", "Components with State"),
            ("component-composition", "Component Composition"),
            ("best-practices", "Best Practices"),
        ],
        "/classes" => vec![
            ("class-macro", "The class! Macro"),
            ("classes-function", "The classes() Function"),
            ("comparison", "class! vs classes()"),
            ("real-world-example", "Real-World Example"),
        ],
        "/when" => vec![
            ("introduction", "Introduction"),
            ("basic-usage", "Basic Usage"),
            ("complex-conditions", "Complex Conditions"),
            ("show-components", "Show Components"),
            ("advanced-patterns", "Advanced Patterns"),
            ("best-practices", "Best Practices"),
        ],
        "/lists" => vec![
            ("introduction", "Introduction"),
            ("basic-example", "Basic Example"),
            ("syntax", "Basic Syntax"),
            ("advanced-patterns", "Advanced Patterns"),
            ("performance", "Performance Considerations"),
        ],
        "/performance" => vec![
            ("introduction", "Introduction"),
            ("signal-optimization", "Signal Optimization"),
            ("component-optimization", "Component Optimization"),
            ("list-rendering-performance", "List Rendering Performance"),
            ("effect-optimization", "Effect Optimization"),
            ("memory-management", "Memory Management"),
            ("bundle-size-optimization", "Bundle Size"),
            ("best-practices", "Best Practices"),
            ("performance-monitoring", "Performance Monitoring"),
        ],
        "/ssr" => vec![
            ("overview", "Overview"),
            ("install-server-crate", "Install the Server Crate"),
            ("buffered-ssr", "Buffered SSR"),
            ("streaming-html", "Streaming HTML"),
            ("hydratable-ssr", "Hydratable SSR"),
            ("client-resume-example", "Client Resume Example"),
            ("framework-adapters", "Framework Adapters"),
        ],
        "/deployment" => vec![
            ("build-for-production", "Build for Production"),
            ("static-hosting", "Static Hosting"),
            ("best-practices", "Best Practices"),
        ],
        _ => vec![],
    }
}

pub struct OnThisPageProps {
    pub current_path: String,
    pub compact: bool,
}

#[component]
pub fn OnThisPage(props: &OnThisPageProps) -> Node {
    let sections = docs_on_this_page_sections(&props.current_path);
    if sections.is_empty() {
        return rsx! { <></> };
    }

    let container_class = if props.compact {
        "on-this-page on-this-page-compact"
    } else {
        "on-this-page"
    };

    rsx! {
        <div class={container_class}>
            <div class="on-this-page-header">
                <h3 class="on-this-page-title">"On this page"</h3>
            </div>
            <nav class={if props.compact { "on-this-page-links on-this-page-links-compact" } else { "on-this-page-links" }}>
                {sections.iter().map(|(id, label)| rsx! {
                    <a href={format!("#{}", id)} class="on-this-page-link">{*label}</a>
                }).collect::<Vec<_>>()}
            </nav>
        </div>
    }
}
