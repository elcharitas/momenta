#![no_std]

extern crate alloc;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

static GITHUB_LINK: &str = "https://github.com/elcharitas/momenta";
static CRATES_LINK: &str = "https://crates.io/crates/momenta";

#[derive(Clone, Copy, PartialEq, SignalValue)]
pub enum Page {
    Home,

    // Start Here
    GettingStarted,
    Philosophy,

    // Core Concepts
    Rsx,
    Signals,
    ComputedSignals,
    Effects,
    Resources,
    Components,
    Classes,

    // Control Flow
    When,
    Lists,

    // Guides
    Performance,
    Deployment,

    // Examples
    Counter,
    TodoMVC,
    HackerNews,
    RealWorld,
}

// Component Props
pub struct HeaderProps {
    pub current_page: Signal<Page>,
    pub theme: Signal<&'static str>,
    pub mobile_menu_open: Signal<bool>,
}

pub struct NavigationProps {
    pub current_page: Signal<Page>,
}

pub struct CodeBlockProps {
    pub code: &'static str,
    pub language: &'static str,
    pub filename: Option<&'static str>,
    pub highlight: Option<&'static str>,
}

pub struct TabsProps {
    pub tabs: Vec<(&'static str, &'static str)>,
    pub children: Vec<Node>,
}

pub struct PlaygroundProps {
    pub code: &'static str,
}

pub struct NoteProps {
    pub variant: &'static str,
    pub children: Vec<Node>,
}

// WASM bindings for highlight.js
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    pub fn highlightAll();
}

// Main App
#[component]
fn App() -> Node {
    let current_page = create_signal(Page::Home);
    let theme = create_signal("light");
    let mobile_menu_open = create_signal(false);

    create_effect(|| {
        highlightAll();
    });

    rsx! {
        <div class={format!("min-h-screen bg-white dark:bg-gray-950 {}", if theme == "dark" { "dark" } else { "" })}>
            <Header {current_page} {theme} {mobile_menu_open} />

            <div class="flex">
                // Sidebar Navigation
                {when!(current_page != Page::Home => <aside class="hidden lg:block w-64 shrink-0 border-r border-gray-200 dark:border-gray-800">
                        <div class="sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto py-8">
                            <Navigation {current_page} />
                        </div>
                    </aside>
                )}

                // Mobile Navigation
                {when!(mobile_menu_open =>
                    <div class="lg:hidden fixed inset-0 z-50 flex">
                        <div class="fixed inset-0 bg-black/20 dark:bg-black/40" on:click={move |_| mobile_menu_open.set(false)}></div>
                        <div class="relative flex w-full max-w-xs flex-col bg-white dark:bg-gray-950">
                            <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-800">
                                <span class="text-lg font-semibold">Navigation</span>
                                <button type="button" on:click={move |_| mobile_menu_open.set(false)} class="p-2">
                                    <i class="fas fa-times"></i>
                                </button>
                            </div>
                            <div class="overflow-y-auto p-4">
                                <Navigation {current_page} />
                            </div>
                        </div>
                    </div>
                )}

                // Main Content
                <main class="flex-1 min-w-0">
                    {when!(current_page.get() {
                        Page::Home => <HomePage {current_page} />,
                        Page::GettingStarted => <GettingStartedPage />,
                        Page::Philosophy => <PhilosophyPage />,
                        Page::Rsx => <RsxPage />,
                        Page::Signals => <SignalsPage />,
                        Page::ComputedSignals => <ComputedSignalsPage />,
                        Page::Effects => <EffectsPage />,
                        Page::Resources => <ResourcesPage />,
                        Page::Components => <ComponentsPage />,
                        Page::Classes => <ClassesPage />,
                        Page::When => <ShowPage />,
                        Page::Lists => <ForPage />,
                        Page::Performance => <PerformancePage />,
                        Page::Deployment => <DeploymentPage />,
                        Page::Counter => <CounterExample />,
                        Page::TodoMVC => <TodoMVCPage />,
                        Page::HackerNews => <HackerNewsPage />,
                        Page::RealWorld => <RealWorldPage />,
                    })}
                </main>

                // Right Sidebar (TOC)
                {when!(current_page != Page::Home => <aside class="hidden xl:block w-64 shrink-0">
                        <div class="sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto p-8">
                            // <TableOfContents {current_page} />
                        </div>
                    </aside>
                )}
            </div>
        </div>
    }
}

// Header Component
#[component]
fn Header(props: &HeaderProps) -> Node {
    let current_page = props.current_page;
    let theme = props.theme;
    let mobile_menu_open = props.mobile_menu_open;

    let toggle_theme = move |_| {
        theme.set(if theme == "dark" { "light" } else { "dark" });
    };

    rsx! {
        <header class={"sticky top-0 z-40 w-full border-b border-gray-200 dark:border-gray-800 bg-white/95 dark:bg-gray-950/95 backdrop-blur supports-[backdrop-filter]:bg-white/60 dark:supports-[backdrop-filter]:bg-gray-950/60"}>
            <div class="flex h-14 items-center px-4 sm:px-6 lg:px-8">
                <button
                    class="lg:hidden p-2 -ml-2"
                    on:click={move |_| mobile_menu_open.set(!mobile_menu_open)}
                >
                    <i class="fas fa-bars"></i>
                </button>

                <a href="#" on:click={move |_| current_page.set(Page::Home)} class="flex items-center space-x-2 ml-2 lg:ml-0">
                    <img src="./static/icon.svg" alt="Momenta Logo" class="w-8 h-8" />
                    <span class="font-bold text-lg">Momenta</span>
                </a>

                <div class="ml-auto flex items-center space-x-4">
                    <nav class="hidden md:flex items-center space-x-6 mr-6">
                        <a href="#" on:click={move |_| current_page.set(Page::Performance)}
                           class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400">
                            Guides
                        </a>
                        <a href="#" on:click={move |_| current_page.set(Page::GettingStarted)}
                           class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400">
                            Documentation
                        </a>
                        <a href="#"
                           class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400">
                            Playground
                        </a>
                    </nav>

                    <button
                        on:click={toggle_theme}
                        class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                    >
                        {when!(theme == "dark" =>
                            <i class="fas fa-sun text-yellow-500"></i>
                        else
                            <i class="fas fa-moon text-gray-600"></i>
                        )}
                    </button>

                    <a href={GITHUB_LINK}
                       class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors">
                        <i class="fab fa-github"></i>
                    </a>
                </div>
            </div>
        </header>
    }
}

// Navigation Component
#[component]
fn Navigation(props: &NavigationProps) -> Node {
    let current = props.current_page;

    let nav_link = move |page: Page, label: &'static str| {
        let is_active = current == page;
        let class = if is_active {
            "block px-3 py-1.5 text-sm font-medium text-blue-600 dark:text-blue-400"
        } else {
            "block px-3 py-1.5 text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200"
        };

        rsx! {
            <a href="#" on:click={move |_| current.set(page)} class={class}>
                {label}
            </a>
        }
    };

    let section = move |title: &'static str, children: Vec<Node>| {
        rsx! {
            <div class="mb-6">
                <h5 class="mb-2 px-3 text-xs font-semibold uppercase tracking-wider text-gray-900 dark:text-gray-100">
                    {title}
                </h5>
                <div class="space-y-1">
                    {children}
                </div>
            </div>
        }
    };

    rsx! {
        <nav class="px-2">
            {section("Start Here", vec![
                nav_link(Page::GettingStarted, "Getting Started"),
                nav_link(Page::Philosophy, "Philosophy"),
            ])}

            {section("Macros", vec![
                nav_link(Page::Rsx, "rsx!"),
                nav_link(Page::Components, "#[component]"),
                nav_link(Page::Classes, "class!"),
            ])}

            {section("Reactive Primitives", vec![
                nav_link(Page::Signals, "create_signal"),
                nav_link(Page::ComputedSignals, "create_computed"),
                nav_link(Page::Effects, "create_effect"),
                nav_link(Page::Resources, "create_resource"),
            ])}

            {section("Control Flow", vec![
                nav_link(Page::When, "when!"),
                nav_link(Page::Lists, ".iter().map()"),
            ])}

            {section("Guides", vec![
                nav_link(Page::Performance, "Performance"),
                nav_link(Page::Deployment, "Deployment"),
            ])}
        </nav>
    }
}

// Reusable Components
#[component]
fn CodeBlock(props: &CodeBlockProps) -> Node {
    rsx! {
        <div class="my-6 overflow-hidden rounded-lg border border-gray-200 dark:border-gray-800">
            {when!(let Some(filename) = props.filename =>
                <div class="flex items-center justify-between border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2">
                    <span class="text-xs font-medium text-gray-600 dark:text-gray-400">{filename}</span>
                    <button class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">
                        <i class="fas fa-copy text-xs">"😉"</i>
                    </button>
                </div>
            )}
            <div class="bg-gray-50 dark:bg-gray-900">
                <pre class="overflow-x-auto">
                    <code class={format!("language-{} text-sm", props.language)}>{props.code}</code>
                </pre>
            </div>
        </div>
    }
}

#[component]
fn Note(props: &NoteProps) -> Node {
    let (bg, border, icon) = match props.variant {
        "info" => (
            "bg-blue-50 dark:bg-blue-950/30",
            "border-blue-200 dark:border-blue-800",
            "fa-info-circle text-blue-600",
        ),
        "warning" => (
            "bg-amber-50 dark:bg-amber-950/30",
            "border-amber-200 dark:border-amber-800",
            "fa-exclamation-triangle text-amber-600",
        ),
        "tip" => (
            "bg-green-50 dark:bg-green-950/30",
            "border-green-200 dark:border-green-800",
            "fa-lightbulb text-green-600",
        ),
        _ => (
            "bg-gray-50 dark:bg-gray-900",
            "border-gray-200 dark:border-gray-800",
            "fa-info-circle text-gray-600",
        ),
    };

    rsx! {
        <div class={format!("my-6 rounded-lg border {} {} p-4", border, bg)}>
            <div class="flex">
                <i class={format!("fas {} mr-3 mt-0.5", icon)}></i>
                <div class="text-sm">
                    {&props.children}
                </div>
            </div>
        </div>
    }
}

#[component]
fn Playground(props: &PlaygroundProps) -> Node {
    rsx! {
        <div class="my-8 overflow-hidden rounded-lg border border-gray-200 dark:border-gray-800">
            <div class="flex flex-col md:flex-row items-stretch h-full">
                <div class="w-1/2 border-r border-gray-200 dark:border-gray-800 flex flex-col">
                    <div class="border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2">
                        <span class="text-xs font-medium text-gray-600 dark:text-gray-400">Code</span>
                    </div>
                    <div class="bg-gray-50 dark:bg-gray-900 flex-1">
                        <pre class="overflow-x-auto h-full">
                            <code class="language-rust text-xs overflow-x">{props.code}</code>
                        </pre>
                    </div>
                </div>
                <div class="w-1/2 flex flex-col">
                    <div class="border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2">
                        <span class="text-xs font-medium text-gray-600 dark:text-gray-400">Output</span>
                    </div>
                    <div class="flex-1 text-sm text-gray-600 dark:text-gray-400">
                        <CounterExample />
                    </div>
                </div>
            </div>
        </div>
    }
}

// Page Components
#[component]
fn HomePage(props: &NavigationProps) -> Node {
    let current_page = props.current_page;
    rsx! {
        <div class="mx-auto max-w-4xl px-4 py-16 sm:px-6 lg:px-8">
            <div class="text-center py-16">
                <h1 class="text-4xl font-bold tracking-tight text-gray-900 dark:text-gray-100 sm:text-5xl">
                    "Simple and performant reactivity for building user interfaces"
                </h1>
                <p class="mt-6 text-lg text-gray-600 dark:text-gray-400">
                    "Momenta makes it simple to build high-performance, reactive user interfaces using Rust's type system and ownership model."
                </p>
                <div class="mt-10 flex items-center justify-center gap-4">
                    <a href="#" on:click={move |_| current_page.set(Page::GettingStarted)} class="rounded-lg bg-blue-600 px-6 py-3 text-sm font-semibold text-white hover:bg-blue-700">
                        "Get Started"
                    </a>
                    <a href={GITHUB_LINK} class="rounded-lg border border-gray-300 dark:border-gray-700 px-6 py-3 text-sm font-semibold hover:bg-gray-50 dark:hover:bg-gray-900">
                        "View on GitHub"
                    </a>
                    <a href={CRATES_LINK} class="rounded-lg bg-yellow-600 border border-gray-300 dark:border-gray-700 px-6 py-3 text-sm font-semibold hover:bg-yellow-700 dark:hover:bg-yellow-900">
                        "View on Crates.io"
                    </a>
                </div>
            </div>

            <div class="mt-24 grid gap-8 sm:grid-cols-2 lg:grid-cols-3">
                <Feature
                    icon="fas fa-zap"
                    title="Element-Level Reactivity"
                    description="Automatically track dependencies and update only what has changed."
                />
                <Feature
                    icon="fas fa-code"
                    title="Familiar API"
                    description="Inspired by React with a Rust-first approach to reactive programming."
                />
                <Feature
                    icon="fas fa-shield-alt"
                    title="Type Safe"
                    description="Leverage Rust's type system for compile-time guarantees and better DX."
                />
                <Feature
                    icon="fas fa-feather"
                    title="Lightweight"
                    description="Small runtime with minimal overhead. Your apps stay fast."
                />
                <Feature
                    icon="fas fa-server"
                    title="SSR Ready"
                    description="Server-side rendering support out of the box for better performance."
                />
                <Feature
                    icon="fas fa-puzzle-piece"
                    title="Composable"
                    description="Build complex UIs from simple, reusable reactive primitives."
                />
            </div>

            <div class="mt-24">
                <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6">Quick Example</h2>
                <Playground
                    code={r#"use momenta::prelude::*;

#[component]
fn CounterExample() -> Node {
    let mut count = create_signal(0);
    rsx! {
        <div class="bg-gradient-to-br from-purple-400 to-blue-600 flex items-center justify-center p-4">
            <div class="bg-white/20 backdrop-blur-lg rounded-3xl p-8 shadow-2xl border border-white/30">
                <h1 class="text-3xl font-bold text-white mb-6 text-center">
                    "Momenta Counter"
                </h1>
                <div class="text-6xl font-bold text-center mb-8 transition-all duration-300 text-white">
                    {count}
                </div>
                <div class="flex gap-4 justify-center">
                    <button
                        class="px-6 py-3 bg-red-500 hover:bg-red-600 text-white font-semibold rounded-xl transition-all duration-200 transform hover:scale-105 shadow-lg"
                        on:click={move |_| count -= 1}
                    >
                        "− Decrease"
                    </button>
                    <button
                        class="px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-semibold rounded-xl transition-all duration-200 transform hover:scale-105 shadow-lg"
                        on:click={move |_| count += 1}
                    >
                        "+ Increase"
                    </button>
                </div>
                <button
                    class="w-full mt-4 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
                    on:click={move |_| count.set(0)}
                >
                    "Reset Count: " {count}
                </button>
            </div>
        </div>
    }
}"#} />
            </div>
        </div>
    }
}

#[component]
fn Feature(props: &FeatureProps) -> Node {
    rsx! {
        <div class="rounded-lg border border-gray-200 dark:border-gray-800 p-6">
            <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400">
                <i class={props.icon}></i>
            </div>
            <h3 class="mb-2 font-semibold text-gray-900 dark:text-gray-100">{props.title}</h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">{props.description}</p>
        </div>
    }
}

pub struct FeatureProps {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[component]
fn ComputedSignalsPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Computed Signals</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    Computed signals and memoization for efficient reactive computations.
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 id="introduction">Introduction</h2>
                <p>
                    Computed signals are reactive values that automatically recalculate when their dependencies change.
                    They are perfect for derived state and expensive computations that should be cached.
                </p>

                <Note variant="info">
                    <p>
                        <strong>Feature flags:</strong> Computed signals require the computed or full-reactivity feature flag to be enabled in your Cargo.toml.
                    </p>
                </Note>

                <h2 id="create-computed">create_computed</h2>
                <p>Create signals that automatically update when their dependencies change:</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn ShoppingCart() -> Node {
    let items = create_signal(vec![
        ("Apple", 1.50, 3),
        ("Banana", 0.75, 5),
        ("Orange", 2.00, 2),
    ]);

    // Computed signal automatically recalculates when items change
    let total = create_computed(move || {
        items.get()
            .iter()
            .map(|(_, price, qty)| price * (*qty as f64))
            .sum::<f64>()
    });

    let item_count = create_computed(move || {
        items.get().len()
    });

    rsx! {
        <div class="shopping-cart">
            <h2>Shopping Cart</h2>
            <div class="cart-summary">
                <p>Items: {item_count}</p>
                <p>Total: ${format!("{:.2}", total.get())}</p>
            </div>
            <button on:click={move |_| {
                items.push(("Grape", 3.00, 1));
            }}>
                Add Grape
            </button>
        </div>
    }
}"#}
                />

                <h3 id="derive-method">The derive() Method</h3>
                <p>Signals have a convenient derive() method for creating computed values:</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let count = create_signal(5);

// Using derive for simple transformations
let doubled = count.derive(|&n| n * 2);
let tripled = count.derive(|&n| n * 3);

// Computed signals update automatically
count.set(10);
assert_eq!(doubled.get(), 20);
assert_eq!(tripled.get(), 30);"#}
                />

                <h2 id="memoization">Memoization</h2>
                <p>
                    Memoization caches expensive computations and only recalculates when dependencies change:
                </p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

let count = create_signal(5);

// Create a memoized computation
let expensive = create_memo("double_count", move || {
    // This only runs when dependencies change
    println!("Computing...");
    count.get() * 2
});

// First access computes and caches
let value = expensive.get(); // Prints "Computing..."

// Second access uses cache
let value2 = expensive.get(); // No print, uses cached value

// Changing the dependency invalidates cache
count.set(10); // Next get() will recompute"#}
                />

                <Note variant="info">
                    <p>
                        <strong>Note:</strong> Memoization requires the memoization or full-reactivity feature flag.
                    </p>
                </Note>

                <h2 id="comparison">When to Use Each</h2>
                <h3>Use create_computed when:</h3>
                <ul>
                    <li>You need a derived value that updates automatically</li>
                    <li>The computation is relatively cheap</li>
                    <li>You want reactive updates throughout your app</li>
                </ul>

                <h3>Use create_memo when:</h3>
                <ul>
                    <li>The computation is expensive</li>
                    <li>You want explicit caching behavior</li>
                    <li>You need to optimize performance-critical code</li>
                </ul>

                <h2 id="best-practices">Best Practices</h2>
                <ul>
                    <li>Use create_computed for simple derived values</li>
                    <li>Use create_memo for expensive computations</li>
                    <li>Keep computation functions pure (no side effects)</li>
                    <li>Name your memos descriptively for easier debugging</li>
                    <li>Avoid circular dependencies between computed signals</li>
                </ul>
            </section>
        </article>
    }
}

#[component]
fn ClassesPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Dynamic Classes</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    Learn how to work with dynamic CSS classes using the class! macro and classes() function.
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Introduction</h2>
                <p>
                    Momenta provides two utilities for working with dynamic CSS classes: the class! macro for ergonomic class composition and the classes() function for conditional class application.
                </p>

                <h2 id="class-macro">The class! Macro</h2>
                <p>The class! macro provides an elegant way to mix static and conditional CSS classes:</p>

                <h3>Basic Usage</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn Button() -> Node {
    let is_active = create_signal(true);
    let is_disabled = create_signal(false);

    rsx! {
        <button class={class!(
            "btn",
            "btn-primary",
            is_active.get() => "btn-active",
            is_disabled.get() => "btn-disabled"
        )}>
            Click me
        </button>
    }
}"#}
                />

                <Note variant="info">
                    <p>
                        <strong>How it works:</strong> The class! macro evaluates conditions at runtime and includes classes only when their conditions are true. Static classes are always included.
                    </p>
                </Note>

                <h3>Syntax Patterns</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let is_active = true;
let is_large = false;
let has_border = true;

// Mix static and conditional classes
let class_name = class!(
    "component",              // Always included
    "component-base",         // Always included
    is_active => "active",    // Included if is_active is true
    is_large => "large",      // Included if is_large is true
    has_border => "bordered"  // Included if has_border is true
);

// Result: "component component-base active bordered""#}
                />

                <h2 id="classes-function">The classes() Function</h2>
                <p>
                    The classes() function provides a more explicit way to conditionally apply classes.
                    It takes a slice of tuples with class names and boolean conditions:
                </p>

                <h3>Basic Usage</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::nodes::classes;

let is_active = true;
let is_disabled = false;

let class_name = classes(&[
    ("btn", true),
    ("btn-active", is_active),
    ("btn-disabled", is_disabled),
]);

// Result: "btn btn-active""#}
                />

                <h3>Using in Components</h3>
                <CodeBlock
                    language="rust"
                    filename="src/components/badge.rs"
                    highlight=""
                    code={r#"use momenta::nodes::classes;

pub struct BadgeProps {
    pub variant: &'static str,
    pub is_pill: bool,
    pub children: Vec<Node>,
}

#[component]
fn Badge(props: &BadgeProps) -> Node {
    let class_name = classes(&[
        ("badge", true),
        ("badge-primary", props.variant == "primary"),
        ("badge-secondary", props.variant == "secondary"),
        ("badge-success", props.variant == "success"),
        ("badge-danger", props.variant == "danger"),
        ("badge-pill", props.is_pill),
    ]);

    rsx! {
        <span class={class_name}>
            {&props.children}
        </span>
    }
}

// Usage
rsx! {
    <Badge variant="success" is_pill={true}>
        Active
    </Badge>
}"#}
                />

                <h2 id="comparison">class! vs classes()</h2>
                <p>Choose the right tool for your use case:</p>

                <h3>Use class! when:</h3>
                <ul>
                    <li>You want concise, inline class composition</li>
                    <li>Mixing static and conditional classes</li>
                    <li>Working directly in RSX</li>
                    <li>Conditions are simple boolean expressions</li>
                </ul>

                <h3>Use classes() when:</h3>
                <ul>
                    <li>You need to compute classes outside of RSX</li>
                    <li>Working with arrays or dynamic conditions</li>
                    <li>Sharing class logic across components</li>
                    <li>Building reusable class utilities</li>
                </ul>

                <h2 id="real-world-example">Real-World Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components/nav.rs"
                    highlight=""
                    code={r#"#[component]
fn Navigation() -> Node {
    let is_mobile_open = create_signal(false);
    let current_route = create_signal("/");

    let nav_link = move |href: &'static str, label: &'static str| {
        let is_active = current_route.get() == href;

        rsx! {
            <a
                href={href}
                class={class!(
                    "nav-link",
                    is_active => "nav-link-active",
                    is_active => "font-bold"
                )}
                on:click={move |_| current_route.set(href)}
            >
                {label}
            </a>
        }
    };

    rsx! {
        <nav class={class!(
            "navbar",
            is_mobile_open.get() => "navbar-expanded"
        )}>
            <button
                class="navbar-toggle"
                on:click={move |_| is_mobile_open.set(!is_mobile_open.get())}
            >
                Menu
            </button>

            <div class={class!(
                "navbar-menu",
                is_mobile_open.get() => "block",
                !is_mobile_open.get() => "hidden"
            )}>
                {nav_link("/", "Home")}
                {nav_link("/about", "About")}
                {nav_link("/contact", "Contact")}
            </div>
        </nav>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>Use class! for inline, component-local class logic</li>
                    <li>Use classes() for computed or reusable class utilities</li>
                    <li>Keep conditional logic simple and readable</li>
                    <li>Consider extracting complex class logic into helper functions</li>
                    <li>Combine with Tailwind CSS or other utility frameworks</li>
                </ul>

                <Note variant="tip">
                    <p>
                        <strong>Performance:</strong> Both class! and classes() are efficient and compile to optimized code. The class! macro evaluates at runtime but has minimal overhead.
                    </p>
                </Note>
            </section>
        </article>
    }
}

#[component]
fn ForPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">List Rendering</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Rendering lists of items efficiently using Rust's iterators."
                </p>
            </header>
            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "List rendering is a common task in UI development. Momenta leverages Rust's powerful iterator system to provide efficient and type-safe list rendering."
                </p>

                <h2 id="basic-example">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn FruitList() -> Node {
    let mut fruits = create_signal(vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
    ]);

    rsx! {
        <div>
            <h2 class="font-bold uppercase">"Fruit List"</h2>
            <ul>
                {fruits.map(|fruit| rsx! {
                    <li>{fruit}</li>
                })}
            </ul>
            <button on:click={move |_| {
                fruits.push("Orange".to_string());
            }}>
                "Add Orange"
            </button>
        </div>
    }
}"#}
                />

                <Note variant="info">
                    <p>
                        <strong>"Good to know:"</strong> " When rendering lists, Momenta efficiently updates only the items that have changed, added, or removed."
                    </p>
                </Note>

                <h2 id="syntax">Basic Syntax</h2>
                <p>"The basic pattern for rendering lists in Momenta uses Rust's iterator methods:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic list rendering pattern
let items = vec!["Item 1", "Item 2", "Item 3"];

rsx! {
    <ul>
        {items.map(|item| rsx! {
            <li>{item}</li>
        })}
    </ul>
}

// With a signal
let items = create_signal(vec!["Item 1", "Item 2", "Item 3"]);

rsx! {
    <ul>
        {items.map(|item| rsx! {
            <li>{item}</li>
        })}
    </ul>
}"#}
                />

                <h2 id="advanced-patterns">Advanced Patterns</h2>

                <h3 id="with-indices">Working with Indices</h3>
                <p>"Sometimes you need the index of each item in the list:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Using enumerate() to get indices
let items = create_signal(vec!["Apple", "Banana", "Cherry"]);

rsx! {
    <ul>
        {items.enumerate().map(|(index, item)| rsx! {
            <li>"Item #" {index + 1} ": " {item}</li>
        })}
    </ul>
}"#}
                />

                <h3 id="complex-items">Complex Item Types</h3>
                <p>"You can render lists of complex types:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Define a struct for list items
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[component]
fn TodoList() -> Node {
    let todos = create_signal(vec![
        Todo { id: 1, text: "Learn Momenta".to_string(), completed: false },
        Todo { id: 2, text: "Build an app".to_string(), completed: false },
        Todo { id: 3, text: "Share with friends".to_string(), completed: true },
    ]);

    let new_todo_text = create_signal(String::new());

    let add_todo = move |_| {
        let text = new_todo_text.get();
        if !text.is_empty() {
            let next_id = todos.map(|todo| todo.id).max().unwrap_or(0) + 1;
            todos.push(Todo {
                id: next_id,
                completed: false,
                text,
            });
            new_todo_text.set(String::new());
        }
    };

    let toggle_todo = move |id: usize| {
        if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
            todo.completed = !todo.completed;
        }
    };

    rsx! {
        <div>
            <h2 class="font-bold uppercase">"Todo List"</h2>
            <ul class="space-y-2">
                {todos.map(|todo| {
                    let id = todo.id;
                    rsx! {
                        <li class={format!("flex items-center {}",
                            if todo.completed { "line-through text-gray-400" } else { "" }
                        )}>
                            <input
                                type="checkbox"
                                checked={todo.completed}
                                on:change={move |_| toggle_todo(id)}
                                class="mr-2"
                            />
                            {todo.text}
                        </li>
                    }
                })}
            </ul>

            <div class="mt-4 flex">
                <input
                    type="text"
                    value={new_todo_text}
                    on:input={move |e| new_todo_text.set(e.value())}
                    placeholder="Add a new todo"
                    class="border p-2 rounded-l"
                />
                <button
                    on:click={add_todo}
                    class="bg-blue-500 text-white p-2 rounded-r"
                >
                    "Add"
                </button>
            </div>
        </div>
    }
}"#}
                />

                <h3 id="filtering-sorting">Filtering and Sorting</h3>
                <p>"You can use Rust's iterator methods to filter and sort items before rendering:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Filtering and sorting example
let numbers = create_signal(vec![5, 2, 8, 1, 9, 3, 7, 4, 6]);
let show_even_only = create_signal(false);
let sort_ascending = create_signal(true);

rsx! {
    <div>
        <div class="mb-4">
            <label class="mr-4">
                <input
                    type="checkbox"
                    checked={show_even_only}
                    on:change={move |_| show_even_only.set(!show_even_only.get())}
                    class="mr-2"
                />
                "Show even numbers only"
            </label>

            <label>
                <input
                    type="checkbox"
                    checked={sort_ascending}
                    on:change={move |_| sort_ascending.set(!sort_ascending.get())}
                    class="mr-2"
                />
                "Sort ascending"
            </label>
        </div>

        <ul class="grid grid-cols-3 gap-2">
            {{
                let mut filtered = numbers.get();

                // Apply filtering if needed
                if show_even_only.get() {
                    filtered.retain(|n| n % 2 == 0);
                }

                // Apply sorting
                if sort_ascending.get() {
                    filtered.sort();
                } else {
                    filtered.sort_by(|a, b| b.cmp(a));
                }

                // Map to nodes
                filtered.iter().map(|n| rsx! {
                    <li class="bg-gray-100 dark:bg-gray-800 p-2 rounded text-center">
                        {n}
                    </li>
                })
            }}
        </ul>
    </div>
}"#}
                />

                <h2 id="performance">Performance Considerations</h2>
                <p>"When rendering lists, keep these performance considerations in mind:"</p>
                <ul>
                    <li>"Use keys for list items when possible to help with efficient updates"</li>
                    <li>"Avoid recreating the entire list when only a few items change"</li>
                    <li>"For large lists, consider pagination or virtualization"</li>
                    <li>"Minimize the amount of work done in the map function"</li>
                    <li>"Pre-compute derived values outside of the render function"</li>
                </ul>

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Use .iter().map() pattern for list rendering"</li>
                    <li>"Extract complex item rendering into separate components"</li>
                    <li>"Use signals for list data that changes over time"</li>
                    <li>"Leverage Rust's powerful iterator methods for filtering, sorting, and transforming data"</li>
                    <li>"Consider memoizing expensive computations for list items"</li>
                </ul>

                <div class="mt-12 flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-6">
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "← Conditional Rendering"
                    </a>
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "Reactivity →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
fn ResourcesPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 title="" class="text-4xl font-bold text-gray-900 dark:text-gray-100">Resources</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Resources provide a way to handle asynchronous data loading with built-in loading and error states."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Introduction</h2>
                <p>
                    "Resources in Momenta are reactive primitives designed for handling asynchronous operations like API calls,
                    file loading, or any other async task. They automatically manage loading states, errors, and data updates."
                </p>

                <h2 class="font-bold uppercase">Basic Usage</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn UserProfile() -> Node {
    let user_resource = create_resource(|| async {
        // Simulate API call
        fetch_user_data().await
    });

    rsx! {
        <div>
            {when!(user_resource.loading() =>
                <div class="loading">"Loading user data..."</div>
            else when!(user_resource.error().is_some() =>
                <div class="error">"Error loading user"</div>
            ) else
                <div>
                    <h1>"User: " {user_resource.get().unwrap_or_default()}</h1>
                </div>
            )}
        </div>
    }
}

async fn fetch_user_data() -> String {
    "John Doe".to_string()
}"#}
                />

                <Note variant="info">
                    <p>
                        <strong>"Automatic State Management:"</strong> " Resources automatically handle loading, error, and success states.
                        You don't need to manually manage these states with separate signals."
                    </p>
                </Note>

                <h2 class="font-bold uppercase">Creating Resources</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

// Simple resource
let user_data = create_resource(|| async {
    fetch_user().await
});

// Resource with parameters
let user_posts = create_resource(move || {
    let user_id = user_id.get();
    async move {
        fetch_user_posts(user_id).await
    }
});

// Resource that depends on signals
let search_results = create_resource(move || {
    let query = search_query.get();
    async move {
        if query.is_empty() {
            Vec::new()
        } else {
            search_api(query).await
        }
    }
});"#}
                />

                <h2 class="font-bold uppercase">Resource States</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"#[component]
fn ResourceStates() -> Node {
    let data_resource = create_resource(|| async {
        // Simulate API call that might fail
        fetch_data_with_potential_error().await
    });

    rsx! {
        <div>
            {when!(data_resource.loading() =>
                <div class="loading">"Loading data..."</div>
            else when!(data_resource.error().is_some() =>
                <div class="error">
                    "Error: " {data_resource.error().unwrap_or_default()}
                    <button on:click={move |_| data_resource.refetch()}>"Retry"</button>
                </div>
            ) else
                <div class="success">
                    "Data: " {data_resource.get().unwrap_or_default()}
                </div>
            )}
        </div>
    }
}

async fn fetch_data_with_potential_error() -> Result<String, String> {
    // Your async logic here
    Ok("Successfully loaded data".to_string())
}"#}
                />

                <h2 class="font-bold uppercase">Retrying Resources</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"#[component]
fn RetryableResource() -> Node {
    let api_resource = create_resource(|| async {
        fetch_api_data().await
    });

    rsx! {
        <div>
            {when!(api_resource.loading() =>
                <div>"Fetching data..."</div>
            else when!(api_resource.error().is_some() =>
                <div>
                    <p>"Failed to load data"</p>
                    <button on:click={move |_| api_resource.refetch()}>"Try Again"</button>
                </div>
            ) else
                <div>{api_resource.get().unwrap_or_default()}</div>
            )}
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Reactive Dependencies</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"#[component]
fn ReactiveResource() -> Node {
    let user_id = create_signal(1);

    // Resource automatically refetches when user_id changes
    let user_profile = create_resource(move || {
        let id = user_id.get();
        async move {
            fetch_user_profile(id).await
        }
    });

    rsx! {
        <div>
            <select on:change={move |e| user_id.set(e.target.value.parse().unwrap_or(1))}>
                <option value="1">"User 1"</option>
                <option value="2">"User 2"</option>
                <option value="3">"User 3"</option>
            </select>

            {when!(user_profile.loading() =>
                <div>"Loading profile..."</div>
            else
                <div>{user_profile.get().unwrap_or_default()}</div>
            )}
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Combining with Effects</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"#[component]
fn ResourceWithEffects() -> Node {
    let data_resource = create_resource(|| async {
        fetch_important_data().await
    });

    // React to resource state changes
    create_effect(move || {
        if let Some(data) = data_resource.get() {
            // Process successful data
            log!("Data loaded successfully: {}", data);
        }

        if let Some(error) = data_resource.error() {
            // Handle errors
            log!("Error occurred: {}", error);
        }
    });

    rsx! {
        <div>
            {when!(data_resource.loading() =>
                <div>"Processing..."</div>
            else
                <div>{data_resource.get().unwrap_or_default()}</div>
            )}
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Use create_resource for any asynchronous data loading"</li>
                    <li>"Resources automatically handle loading, error, and success states"</li>
                    <li>"Use refetch() method to retry failed requests"</li>
                    <li>"Resources automatically re-run when their dependencies change"</li>
                    <li>"Combine resources with effects for complex state management"</li>
                    <li>"Use conditional rendering with when! for different resource states"</li>
                </ul>

                <Note variant="tip">
                    <p>
                        <strong>"Performance:"</strong> " Resources are optimized for client-side rendering and
                        integrate seamlessly with Momenta's reactive system for efficient updates."
                    </p>
                </Note>
            </section>
        </article>
    }
}

#[component]
fn PhilosophyPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Philosophy</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Understanding the principles and design decisions behind Momenta."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Core Principles</h2>

                <h3>1. Element-Level Reactivity</h3>
                <p>
                    "Momenta uses element-level reactivity, which means only the specific parts of your UI that depend on
                    changed data will be updated. This is more efficient than virtual DOM diffing and provides consistent performance."
                </p>

                <h3>2. Rust-First Design</h3>
                <p>
                    "Rather than porting concepts from JavaScript frameworks, Momenta embraces Rust's ownership model and
                    type system. This leads to better performance and fewer runtime errors."
                </p>

                <h3>3. Explicit Reactivity</h3>
                <p>
                    "Reactivity is explicit in Momenta. You explicitly create signals, effects, and resources. This makes
                    the reactive system predictable and debuggable."
                </p>

                <h3>4. Composability Over Configuration</h3>
                <p>
                    "Momenta provides primitive building blocks that can be composed to create complex applications.
                    There's no magic configuration or conventions - just composable primitives."
                </p>

                <h2 class="font-bold uppercase">Why Not Virtual DOM?</h2>
                <p>
                    "Virtual DOM was designed to solve a specific problem: making imperative DOM updates manageable.
                    However, with element-level reactivity, we can track exactly what changed and update the DOM directly."
                </p>

                <h2 class="font-bold uppercase">Comparison with Other Frameworks</h2>
                <p>
                    "Momenta draws inspiration from SolidJS's reactivity model but implements it in Rust with zero-cost
                    abstractions. Unlike React, there are no re-renders or reconciliation phases."
                </p>

                <Note variant="tip">
                    <p>
                        <strong>Performance:</strong> Because Momenta compiles to efficient native code and uses
                        element-level updates, your applications will be fast by default.
                    </p>
                </Note>
            </section>
        </article>
    }
}

#[component]
fn RsxPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">rsx!</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    rsx! is a built in macro that allows you to write HTML-like syntax inside Rust code.
                    "It's a way to declaratively describe the structure of your UI."
                </p>
            </header>
            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "RSX allows you to write HTML-like syntax inside Rust code. It's a way to declaratively describe the structure of your UI."
                </p>
                <h2 id="basic-example">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn HelloWorld() -> Node {
    let name = create_signal("World");

    rsx! {
        <div>
            <h1>"Hello, " {name} "!"</h1>
            <p>Welcome to Momenta.</p> // see how quotes are totally optional?
            <p>"😉"</p> // N/B: currently, Momenta requires quotes for emojis
        </div>
    }
}"#}
                />
                <Note variant="info">
                    <p>
                        <strong>"Good to know:"</strong> " The rsx! macro returns a Node type that can be rendered to the DOM. Nodes are lightweight and can be composed together to build complex UIs."
                    </p>
                </Note>
                <h2 id="api-reference">API Reference</h2>
                <h3 id="creating-elements">Creating Elements</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic element
let div = rsx! { <div></div> };

// Element with attributes
let button = rsx! { <button type="button" class="primary"></button> };

// Self-closing element
let input = rsx! { <input type="text" /> };

// Note: HTML attributes with hyphens use underscores in RSX
// e.g., `data-id` becomes `data_id`
let custom = rsx! { <div data_id="123"></div> };

// Attributes that conflict with Rust keywords use trailing underscore
// e.g., `type` becomes `type_`
let input = rsx! { <input type="text" /> };"#}
                />
                <h3 id="attributes">Dynamic Attributes</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Dynamic class names
let is_active = create_signal(true);
let element = rsx! {
    <div class={format!("container {}", if is_active.get() { "active" } else { "" })}>
        <p>"Hello, world!"</p>
    </div>
};

// Conditional attributes
let disabled = create_signal(false);
let button = rsx! {
    <button
        class="btn"
        disabled={disabled.get()}
    >
        "Submit"
    </button>
};

// Event handlers
let count = create_signal(0);
let button = rsx! {
    <button on:click={move |_| count += 1}>
        "Clicked " {count} " times"
    </button>
};"#}
                />
                <h3 id="children">Dynamic Children</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Text nodes
let name = "World";
let element = rsx! {
    <div>
        "Hello, " {name} "!"
    </div>
};

// Signal values
let count = create_signal(0);
let element = rsx! {
    <div>
        "Count: " {count}
    </div>
};

// Conditional rendering with when! macro
let is_logged_in = create_signal(true);
let element = rsx! {
    <div>
        {when!(is_logged_in =>
            <p>"Welcome back!"</p>
        else
            <p>"Please log in"</p>
        )}
    </div>
};

// Lists with iterators
let items = create_signal(vec!["Apple", "Banana", "Cherry"]);
let list = rsx! {
    <ul>
        {items.map(|item| rsx! {
            <li>{item}</li>
        })}
    </ul>
};"#}
                />

                <h3 id="fragments">Fragments</h3>
                <p>"When you need to return multiple elements without a wrapper, you can use fragments:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Using fragments
let elements = rsx! {
    <>
        <h1>"Title"</h1>
        <p>"Paragraph 1"</p>
        <p>"Paragraph 2"</p>
    </>
};"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Keep your components small and focused on a single responsibility"</li>
                    <li>"Use signals for state that changes over time"</li>
                    <li>"Extract repeated patterns into reusable components"</li>
                    <li>"Use the when! macro for conditional rendering"</li>
                    <li>"Use iterators with .map() for rendering lists"</li>
                </ul>

                <div class="mt-12 flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-6">
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "← Getting Started"
                    </a>
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "Signals →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
pub fn EffectsPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Effects</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Effects are the building blocks of reactivity in Momenta. They run code in response to changes in signals."
                </p>
            </header>
            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "Effects are functions that run when their dependencies change. They are the building blocks of reactivity in Momenta. Effects automatically track any signals accessed during their execution and re-run when those signals change."
                </p>
                <h2 id="basic-example">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn Counter() -> Node {
    let count = create_signal(0);

    // This effect will run whenever count changes
    create_effect(move || {
        log!("Count changed to: {}", count.get());
    });

    rsx! {
        <div>
            <p>"Current count: " {count}</p>
            <button on:click={move |_| count += 1}>"Increment"</button>
        </div>
    }
}"#}
                />
                <h2 id="api-reference">API Reference</h2>
                <h3 id="creating-effects">Creating Effects</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic effect creation
let name = create_signal("Alice".to_string());

// This effect will run once immediately and then
// whenever any of its dependencies change
create_effect(move || {
    log!("Hello, {}", name.get());
});

// Effects can access multiple signals
let count = create_signal(0);
let multiplier = create_signal(2);

create_effect(move || {
    let result = count.get() * multiplier.get();
    log!("Result: {}", result);
});"#}
                />
                <h2 class="font-bold uppercase">Advanced Patterns</h2>
                <h3>Effect Dependencies</h3>
                <p>"Effects can depend on multiple signals, and they will only run when any of their dependencies change:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Effect with multiple dependencies
let first_name = create_signal("John".to_string());
let last_name = create_signal("Doe".to_string());

create_effect(move || {
    // This effect depends on both first_name and last_name
    let full_name = format!("{} {}", first_name.get(), last_name.get());
    log!("Full name: {}", full_name);
});

// Changing either signal will trigger the effect
first_name.set("Jane".to_string()); // Effect runs
last_name.set("Smith".to_string()); // Effect runs again"#}
                />
                <h3>Effect Ordering</h3>
                <p>"Effects are executed in the order they are created"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Effect ordering
let count = create_signal(0);

create_effect(move || {
    log!("Effect 1: {}", count.get());
});

create_effect(move || {
    log!("Effect 2: {}", count.get());
});

// When count changes, the output will be:
// Effect 1: 1
// Effect 2: 1"#}
                />
                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Keep effects as lightweight as possible"</li>
                    <li>"Avoid creating effects inside loops or other complex logic"</li>
                    <li>"Use " <code>"create_effect"</code> " for simple effects"</li>
                    <li>"Don't modify signals that you're tracking in the same effect to avoid infinite loops"</li>
                    <li>"Group related effects together for better code organization"</li>
                </ul>
                <div class="mt-12 flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-6">
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "← Signals"
                    </a>
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "Resources →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
fn SignalsPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Signals</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Signals are the most basic reactive primitive in Momenta. They contain values that change over time."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "When you create a signal, you get a getter and setter function. The getter tracks any scope it's called in,
                    and the setter triggers updates to any computations that depend on the signal's value."
                </p>

                <h2 id="basic-example">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn App() -> Node {
    // Create a signal with initial value 0
    let count = create_signal(0);

    rsx! {
        <div>
            <p>Count: {count}</p>
            <button on:click={move |_| count += 1}>
                "Increment"
            </button>
        </div>
    }
}"#}
                />

                <Note variant="info">
                    <p>
                        <strong>"Good to know:"</strong> " Unlike other frameworks, You  accessing a signal's value requires calling "
                        <code>".get()"</code> ". This explicit call enables Momenta's element-level reactivity system to track dependencies precisely."
                    </p>
                </Note>

                <h2 id="api-reference">API Reference</h2>

                <h3 id="creating-signals">Creating Signals</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic signal creation
let count = create_signal(0);
let name = create_signal("Alice".to_string());
let todos = create_signal(vec![]);

// With type annotations
let typed: Signal<i32> = create_signal(0);
let items: Signal<Vec<String>> = create_signal(vec![]);"#}
                />

                <h3 id="reading-values">Reading Values</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let count = create_signal(5);

// Get current value
let value = count.get(); // 5

// Use in reactive context
create_effect(move || {
    log!("Count is: {}", count);
});

// Use with closures
let doubled = move || count * 2;"#}
                />

                <h3 id="updating-values">Updating Values</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let mut count = create_signal(0);

// Override the value
count.set(5); // Now count is 5

// Update based on previous value
count += 1; // Now count is 6
"#}
                />
                <Note variant="tip">
                    <p>
                        <strong>"Performance tip:"</strong> Avoid creating derived signals for every possible combination of signals.
                    </p>
                </Note>

                <h3>Signal Utilities</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Check if signals are equal
let a = create_signal(5);
let b = create_signal(5);
let are_equal = a == b; // true
"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Keep signals at the appropriate scope - not everything needs to be global state"</li>
                    <li>"Prefer fine-grained signals over large state objects for better performance"</li>
                    <li>"Group related signals into structs for better organization"</li>
                    <li>"Use derived values (closures that read signals) instead of creating redundant signals"</li>
                    <li>"Consider using custom signal types for domain-specific state management"</li>
                </ul>

                <div class="mt-12 flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-6">
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "← Getting Started"
                    </a>
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "Effects →"
                    </a>
                </div>
            </section>
        </article>
    }
}

// Add more page implementations...
#[component]
fn GettingStartedPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Getting Started</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Get up and running with Momenta in minutes."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Installation</h2>
                <p>"Add Momenta to your " <code>"Cargo.toml"</code> ":"</p>
                <CodeBlock
                    filename="Cargo.toml"
                    highlight=""
                    language="toml"
                    code={r#"[dependencies]
momenta = "0.2"

# For web projects
[dependencies.web-sys]
version = "0.3"
features = ["Document", "Element", "HtmlElement"]"#}
                />

                <h2 class="font-bold uppercase">Your First Component</h2>
                <CodeBlock
                    filename="src/main.rs"
                    highlight=""
                    language="rust"
                    code={r#"use momenta::prelude::*;

#[component]
fn App() -> Node {
    let name = create_signal("World");

    rsx! {
        <div class="container">
            <h1>Hello, {name}!</h1>
            <input
                type="text"
                value={name}
                on:input={move |e| name.set(e.value())}
                placeholder="Enter your name"
            />
        </div>
    }
}

fn main() {
    mount_to_body::<App>();
}"#}
                />

                <h2 class="font-bold uppercase">Project Structure</h2>
                <p>"A typical Momenta project structure looks like this:"</p>
                <CodeBlock
                    language="text"
                    highlight=""
                    filename=""
                    code={r#"my-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   └── footer.rs
│   └── pages/
│       ├── mod.rs
│       └── home.rs
├── static/
│   └── index.html
└── style/
    └── main.css"#}
                />

                <Note variant="tip">
                    <p>
                        <strong>"Tip:"</strong> add a <code>"static"</code> folder to your project to serve static files like images, CSS, and JavaScript.
                    </p>
                </Note>
            </section>
        </article>
    }
}

#[component]
fn ComponentsPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Components</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Components are reusable pieces of UI logic marked with the #[component] attribute."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Introduction</h2>
                <p>
                    "Components in Momenta are functions that return a Node. They can accept props and maintain
                    internal state using signals and other reactive primitives."
                </p>

                <h2 class="font-bold uppercase">Basic Component</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components/button.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn Button() -> Node {
    rsx! {
        <button class="btn">
            "Click me"
        </button>
    }
}

// Usage
#[component]
fn App() -> Node {
    rsx! {
        <div>
            <Button />
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Components with Props</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components/button.rs"
                    highlight=""
                    code={r#"pub struct ButtonProps {
    pub text: &'static str,
    pub variant: &'static str,
    pub on:click: Box<dyn Fn()>,
}

#[component]
fn Button(props: &ButtonProps) -> Node {
    let class = format!("btn btn-{}", props.variant);

    rsx! {
        <button class={class} on:click={props.on:click}>
            {props.text}
        </button>
    }
}

// Usage
#[component]
fn App() -> Node {
    let count = create_signal(0);

    rsx! {
        <div>
            <p>Count: {count}</p>
            <Button
                text="Increment"
                variant="primary"
                on:click={move || count += 1}
            />
        </div>
    }
}"#}
                />
                <Note variant="info">
                    <p>
                        <strong>"Good to know:"</strong> " Props must always be passed by reference or excluded totally. This ensures that the component can be re-rendered when the props change. "
                    </p>
                </Note>

                <h2 class="font-bold uppercase">Components with State</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components/toggle.rs"
                    highlight=""
                    code={r#"#[component]
fn Toggle() -> Node {
    let is_on = create_signal(false);

    let toggle = move |_| {
        is_on.set(!is_on);
    };

    rsx! {
        <div class="toggle">
            <button
                class={when!(is_on => "toggle-on" else "toggle-off")}
                on:click={toggle}
            >
                {when!(is_on => "On" else "Off")}
            </button>
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Component Composition</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components/card.rs"
                    highlight=""
                    code={r#"pub struct CardProps {
    pub title: &'static str,
    pub children: Vec<Node>,
}

#[component]
fn Card(props: &CardProps) -> Node {
    rsx! {
        <div class="card">
            <div class="card-header">
                <h3>{&props.title}</h3>
            </div>
            <div class="card-body">
                {&props.children}
            </div>
        </div>
    }
}

// Usage
#[component]
fn App() -> Node {
    rsx! {
        <Card title="User Profile">
            <p>"Name: John Doe"</p>
            <p>"Email: john@example.com"</p>
            <Button text="Edit Profile" variant="secondary" />
        </Card>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Keep components focused on a single responsibility"</li>
                    <li>"Use props for data that changes between instances"</li>
                    <li>"Use signals for component-local state"</li>
                    <li>"Prefer composition over complex prop drilling"</li>
                    <li>"Name components with PascalCase"</li>
                </ul>

                <Note variant="tip">
                    <p>
                        <strong>"Performance:"</strong> " Components in Momenta have minimal overhead.
                        They're just functions that return JSX, so don't hesitate to break your UI into small, reusable pieces."
                    </p>
                </Note>
            </section>
        </article>
    }
}

#[component]
fn PerformancePage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Performance</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Optimize your Momenta applications for maximum performance and efficiency."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Introduction</h2>
                <p>
                    "Momenta is designed for performance from the ground up. Its fine-grained reactivity system
                    ensures that only the parts of your UI that actually need to update will re-render."
                </p>

                <h2 class="font-bold uppercase">Signal Optimization</h2>
                <h3>Fine-grained Signals</h3>
                <p>"Use specific signals instead of large state objects for better performance:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// ❌ Avoid: Large state object
let app_state = create_signal(AppState {
    user: User { name: "John".to_string(), email: "john@example.com".to_string() },
    settings: Settings { theme: "dark".to_string(), notifications: true },
    data: vec![/* large dataset */],
});

// ✅ Better: Fine-grained signals
let user_name = create_signal("John".to_string());
let user_email = create_signal("john@example.com".to_string());
let theme = create_signal("dark".to_string());
let notifications = create_signal(true);
let data = create_signal(vec![/* large dataset */]);"#}
                />

                <h3>Derived Values</h3>
                <p>"Use closures for computed values instead of creating additional signals:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let first_name = create_signal("John".to_string());
let last_name = create_signal("Doe".to_string());

// ✅ Use closure for derived value
let full_name = move || format!("{} {}", first_name.get(), last_name.get());

rsx! {
    <div>
        <p>"Welcome, " {full_name()}</p>
    </div>
}"#}
                />

                <h2 class="font-bold uppercase">Component Optimization</h2>
                <h3>Component Splitting</h3>
                <p>"Break large components into smaller ones to minimize re-renders:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// ✅ Split components for better performance
#[component]
fn UserProfile() -> Node {
    rsx! {
        <div>
            <UserAvatar />
            <UserInfo />
            <UserActions />
        </div>
    }
}

#[component]
fn UserAvatar() -> Node {
    let avatar_url = create_signal("avatar.jpg".to_string());
    rsx! {
        <img src={avatar_url} alt="User Avatar" />
    }
}

#[component]
fn UserInfo() -> Node {
    let name = create_signal("John Doe".to_string());
    rsx! {
        <div>
            <h2>{name}</h2>
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">List Rendering Performance</h2>
                <h3>Efficient List Updates</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// ✅ Efficient list rendering with keys
let items = create_signal(vec!["Item 1", "Item 2", "Item 3"]);

rsx! {
    <ul>
        {items.get().iter().enumerate().map(|(index, item)| {
            rsx! {
                <li key={index}>{item}</li>
            }
        })}
    </ul>
}"#}
                />

                <h3>Virtualization for Large Lists</h3>
                <p>"For very large lists, consider implementing virtualization:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"#[component]
fn VirtualizedList() -> Node {
    let items = create_signal((0..10000).map(|i| format!("Item {}", i)).collect::<Vec<_>>());
    let visible_start = create_signal(0);
    let visible_count = 20;

    rsx! {
        <div class="h-96 overflow-y-auto" on:scroll={move |e| {
            let scroll_top = e.scroll_top();
            let item_height = 40;
            visible_start.set(scroll_top / item_height);
        }}>
            <div style={format!("height: {}px", items.get().len() * 40)}>
                <div style={format!("transform: translateY({}px)", visible_start.get() * 40)}>
                    {items.get().iter().skip(visible_start.get()).take(visible_count).enumerate().map(|(i, item)| {
                        rsx! {
                            <div key={visible_start.get() + i} class="h-10 flex items-center px-4 border-b">
                                {item}
                            </div>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Effect Optimization</h2>
                <h3>Minimize Effect Dependencies</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let user_id = create_signal(1);
let user_name = create_signal("John".to_string());
let last_login = create_signal("2024-01-01".to_string());

// ❌ Avoid: Effect depends on unnecessary signals
create_effect(move || {
    log!("User {} logged in at {}", user_name.get(), last_login.get());
    // This effect will run when user_name changes, even though we only care about last_login
});

// ✅ Better: Only depend on what you need
create_effect(move || {
    log!("User logged in at {}", last_login.get());
});"#}
                />

                <h2 class="font-bold uppercase">Memory Management</h2>
                <h3>Avoid Memory Leaks</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"#[component]
fn CounterComponent() -> Node {
    let count = create_signal(0);

    // ✅ Effects automatically clean up when signals change
    create_effect(move || {
        // This effect will re-run when count changes
        log!("Count updated: {}", count.get());
    });

    rsx! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click={move |_| count.set(count.get() + 1)}>"Increment"</button>
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Bundle Size Optimization</h2>
                <h3>Code Splitting</h3>
                <CodeBlock
                    language="rust"
                    filename="Cargo.toml"
                    highlight=""
                    code={r#"[profile.release]
opt-level = "s"  # Optimize for size
lto = true        # Link-time optimization
codegen-units = 1 # Better optimization
panic = "abort"   # Smaller binary size"#}
                />

                <h3>Feature Flags</h3>
                <CodeBlock
                    language="rust"
                    filename="Cargo.toml"
                    highlight=""
                    code={r#"[dependencies.momenta]
version = \"0.2\"
default-features = false
features = [\"web\", \"signals\"]  # Only include what you need"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>Use fine-grained signals instead of large state objects</li>
                    <li>Prefer closures for derived values over additional signals</li>
                    <li>Split large components into smaller, focused components</li>
                    <li>Minimize dependencies in effects</li>
                    <li>Use keys for list items to help with efficient updates</li>
                    <li>Consider virtualization for very large lists</li>
                    <li>Clean up resources and intervals in effects</li>
                    <li>Use release profile optimizations for production builds</li>
                    <li>Only include necessary features to reduce bundle size</li>
                </ul>

                <h2 class="font-bold uppercase">Performance Monitoring</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// ✅ Monitor performance in development
#[cfg(debug_assertions)]
create_effect(move || {
    let start = performance.now();

    // Your reactive code here
    expensive_computation();

    let end = performance.now();
    if end - start > 16.0 { // More than one frame (60fps)
        log!("Slow update detected: {}ms", end - start);
    }
});"#}
                />

                <div class="mt-12 flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-6">
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "← Lists"
                    </a>
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "Rust Integration →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
fn DeploymentPage() -> Node {
    rsx! {
        <div class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8 space-y-8">
            <div>
                <h1 class="text-4xl font-bold mb-4">Deployment</h1>
                <p class="text-lg text-gray-600 mb-8">
                    Deploy your Momenta applications to production.
                </p>
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Build for Production</h2>
                <p class="mb-4">
                    Optimize your build for production deployment.
                </p>
                <CodeBlock
                    language="bash"
                    filename="terminal"
                    highlight=""
                    code={r#"# Build with optimizations
cargo build --release --target wasm32-unknown-unknown

# Generate bindings
wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/release/your_app.wasm"#}
                />
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Static Hosting</h2>
                <p class="mb-4">
                    Deploy to static hosting platforms like Netlify or Vercel.
                </p>
                <CodeBlock
                    language="toml"
                    filename="netlify.toml"
                    highlight=""
                    code={r#"[build]
  command = "cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/release/your_app.wasm"
  publish = "pkg"

[[headers]]
  for = "*.wasm"
  [headers.values]
    Content-Type = "application/wasm""#}
                />
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Best Practices</h2>
                <ul class="list-disc list-inside space-y-2 text-gray-700">
                    <li>Use release builds for production</li>
                    <li>Enable WASM optimizations</li>
                    <li>Set proper MIME types for .wasm files</li>
                    <li>Use CDN for faster loading</li>
                    <li>Enable gzip compression</li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn TodoMVCPage() -> Node {
    rsx! {
        <div class="space-y-8">
            <div>
                <h1 class="text-4xl font-bold mb-4">TodoMVC Example</h1>
                <p class="text-lg text-gray-600 mb-8">
                    "A complete TodoMVC implementation showcasing Momenta's capabilities."
                </p>
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Todo State Management</h2>
                <CodeBlock
                    language="rust"
                    filename="src/todo.rs"
                    highlight=""
                    code={r#"#[derive(Clone, PartialEq)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

let todos = create_signal(Vec::<Todo>::new());
let filter = create_signal(Filter::All);"#}
                />
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Add Todo Component</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components.rs"
                    highlight=""
                    code={r#"#[component]
fn AddTodo(todos: Signal<Vec<Todo>>) -> Node {
    let input_text = create_signal(String::new());

    rsx! {
        <input
            type="text"
            placeholder="What needs to be done?"
            value={input_text.get()}
            oninput={move |e| input_text.set(e.target.value)}
            onkeydown={move |e| {
                if e.key == "Enter" && !input_text.get().trim().is_empty() {
                    let new_todo = Todo {
                        id: generate_id(),
                        text: input_text.get().trim().to_string(),
                        completed: false,
                    };
                    todos.update(|list| list.push(new_todo));
                    input_text.set(String::new());
                }
            }}
        />
    }
}"#}
                />
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Key Features</h2>
                <ul class="list-disc list-inside space-y-2 text-gray-700">
                    <li>Add, edit, and delete todos</li>
                    <li>Mark todos as complete/incomplete</li>
                    <li>Filter todos by status (All, Active, Completed)</li>
                    <li>Clear completed todos</li>
                    <li>Persistent state management</li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn HackerNewsPage() -> Node {
    rsx! {
        <div class="space-y-8">
            <div>
                <h1 class="text-4xl font-bold mb-4">HackerNews Clone</h1>
                <p class="text-lg text-gray-600 mb-8">
                    An advanced example showing API integration and real-time updates.
                </p>
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">API Integration</h2>
                <CodeBlock
                    language="rust"
                    filename="src/api.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[derive(Clone, PartialEq)]
struct Story {
    id: u32,
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
}

// Use create_resource for async data loading
let top_stories = create_resource(|| async {
    // In a real app, you'd use fetch API or HTTP client
    fetch_top_stories().await
});

async fn fetch_top_stories() -> Result<Vec<Story>, String> {
    // Simulate API call
    Ok(vec![
        Story {
            id: 1,
            title: "Example Story".to_string(),
            url: Some("https://example.com".to_string()),
            score: 100,
            by: "user123".to_string(),
        }
    ])
}"#}
                />
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Story List Component</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components.rs"
                    highlight=""
                    code={r#"#[component]
fn StoryList() -> Node {
    let stories_resource = create_resource(|| async {
        fetch_stories().await
    });

    rsx! {
        <div class="story-list">
            {when!(stories_resource.loading() =>
                <div>"Loading stories..."</div>
            else when!(stories_resource.error().is_some() =>
                <div class="error">"Failed to load stories"</div>
            ) else
                <div>
                    {stories_resource.get().unwrap_or_default().iter().map(|story| {
                        rsx! {
                            <div key={story.id} class="story-item">
                                <h3>{story.title.clone()}</h3>
                                <p>"Score: " {story.score} " by " {story.by.clone()}</p>
                                {when!(story.url.is_some() =>
                                    <a href={story.url.clone().unwrap()} target="_blank">"Read more"</a>
                                )}
                            </div>
                        }
                    })}
                </div>
            )}
        </div>
    }
}

async fn fetch_stories() -> Result<Vec<Story>, String> {
    // Simulate API call to HackerNews
    Ok(vec![
        Story {
            id: 1,
            title: "Show HN: My new project".to_string(),
            url: Some("https://example.com".to_string()),
            score: 42,
            by: "developer".to_string(),
        },
        Story {
            id: 2,
            title: "Ask HN: What's your favorite tool?".to_string(),
            url: None,
            score: 15,
            by: "curious_user".to_string(),
        },
    ])
}"#}
                />
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Features</h2>
                <ul class="list-disc list-inside space-y-2 text-gray-700">
                    <li>Real-time story fetching from HackerNews API</li>
                    <li>Infinite scrolling and pagination</li>
                    <li>Story details and comments</li>
                    <li>Search and filtering</li>
                    <li>Responsive design</li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn RealWorldPage() -> Node {
    rsx! {
        <div class="space-y-8">
            <div>
                <h1 class="text-4xl font-bold mb-4">RealWorld App</h1>
                <p class="text-lg text-gray-600 mb-8">
                    A full-featured blogging platform demonstrating complex state management.
                </p>
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Authentication</h2>
                <CodeBlock
                    language="rust"
                    filename="src/auth.rs"
                    highlight=""
                    code={r#"#[derive(Clone, PartialEq)]
struct User {
    username: String,
    email: String,
    token: String,
}

let user = create_signal(Option::<User>::None);
let is_authenticated = create_signal(false);

// Update authentication status when user changes
create_effect(move || {
    is_authenticated.set(user.get().is_some());
});"#}
                />
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Article Management</h2>
                <CodeBlock
                    language="rust"
                    filename="src/articles.rs"
                    highlight=""
                    code={r#"#[component]
fn ArticleEditor() -> Node {
    let title = create_signal(String::new());
    let body = create_signal(String::new());
    let tags = create_signal(Vec::<String>::new());

    let submit_article = move || {
        let article = Article {
            title: title.get(),
            body: body.get(),
            tag_list: tags.get(),
        };
        // Submit to API
    };

    rsx! {
        <form onsubmit={move |_| submit_article()}>
            // Form fields
        </form>
    }
}"#}
                />
            </div>

            <div>
                <h2 class="text-2xl font-bold mb-4">Key Features</h2>
                <ul class="list-disc list-inside space-y-2 text-gray-700">
                    <li>User authentication and profiles</li>
                    <li>Article creation, editing, and deletion</li>
                    <li>Comments and favorites</li>
                    <li>Following users and tags</li>
                    <li>Global feed and personal feed</li>
                    <li>Responsive design with routing</li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn ShowPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Conditional Rendering</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Use when! macro for conditional rendering based on reactive values."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Introduction</h2>
                <p>
                    "The when! macro provides a clean way to conditionally render different UI based on
                    reactive values. It's similar to ternary operators but integrates seamlessly with Momenta's reactivity."
                </p>

                <h2 class="font-bold uppercase">Basic Usage</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn App() -> Node {
    let is_logged_in = create_signal(false);

    rsx! {
        <div>
            {when!(is_logged_in =>
                <div>
                    <h1>Welcome back!</h1>
                    <button on:click={move |_| is_logged_in.set(false)}>
                        "Logout"
                    </button>
                </div>
            else
                <div>
                    <h1>Please log in</h1>
                    <button on:click={move |_| is_logged_in.set(true)}>
                        "Login"
                    </button>
                </div>
            )}
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Complex Conditions</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let user_role = create_signal("guest");
let is_loading = create_signal(false);

rsx! {
    <div>
        {when!(is_loading =>
            <div class="spinner">Loading...</div>
        else when!(user_role == "admin" =>
            <AdminPanel />
        else when!(user_role == "user" =>
            <UserDashboard />
        else
            <GuestLanding />
        )))}
    </div>
}"#}
                />

                <h2 class="font-bold uppercase">Show Components</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"pub struct ShowProps {
    pub when: bool,
    pub children: Vec<Node>,
}

#[component]
fn Show(props: &ShowProps) -> Node {
    if props.when {
        rsx! { <div>{&props.children}</div> }
    } else {
        rsx! { <div></div> }
    }
}

// Usage
let show_details = create_signal(false);

rsx! {
    <div>
        <button on:click={move |_| show_details.set(!show_details)}>
            "Toggle Details"
        </button>
        <Show when={show_details.get()}>
            <p>These are the details!</p>
            <p>Only visible when show_details is true.</p>
        </Show>
    </div>
}"#}
                />

                <h2 class="font-bold uppercase">Advanced Patterns</h2>

                <h3>Loading States</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"#[derive(Clone, Copy, PartialEq)]
enum LoadingState {
    Idle,
    Loading,
    Success,
    Error,
}

let state = create_signal(LoadingState::Idle);

rsx! {
    <div>
        {when!(state.get() {
            LoadingState::Loading => <div class="loading">
                <i class="fas fa-spinner fa-spin"></i>
                " Loading..."
            </div>,
            LoadingState::Success => <div class="success">
                <i class="fas fa-check"></i>
                " Success!"
            </div>,
            LoadingState::Error => <div class="error">
                <i class="fas fa-exclamation-triangle"></i>
                " Something went wrong"
            </div>,
            _ => <button on:click={move |_| state.set(LoadingState::Loading)}>
                "Start Operation"
            </button>
        })}
    </div>
}"#}
                />

                <h3>Permission-Based Rendering</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"pub struct PermissionProps {
    pub required_permission: &'static str,
    pub user_permissions: Vec<String>,
    pub children: Vec<Node>,
}

#[component]
fn RequirePermission(props: &PermissionProps) -> Node {
    let has_permission = props.user_permissions
        .iter()
        .any(|p| p == props.required_permission);

    rsx! {
        {when!(has_permission =>
            <div>{&props.children}</div>
        else
            <div class="permission-denied">
                "You don't have permission to view this content."
            </div>
        )}
    }
}

// Usage
let user_permissions = vec!["admin".to_string(), "user".to_string()];
rsx! {
    <RequirePermission
        required_permission="admin"
        user_permissions={user_permissions}
    >
        <AdminSettings />
    </RequirePermission>
}"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Use when! for simple boolean conditions"</li>
                    <li>"Consider creating Show/Hide components for reusable patterns"</li>
                    <li>"Avoid deeply nested conditional rendering"</li>
                    <li>"Use match expressions for complex state-based rendering"</li>
                    <li>"Keep condition logic readable and maintainable"</li>
                </ul>
            </section>
        </article>
    }
}

#[component]
fn CounterExample() -> Node {
    let mut count = create_signal(0);

    rsx! {
        <div class="min-h-full bg-gradient-to-br from-purple-400 to-blue-600 flex items-center justify-center p-4">
            <div class="bg-white/20 backdrop-blur-lg rounded-3xl p-8 shadow-2xl border border-white/30">
                <h1 class="text-3xl font-bold text-white mb-6 text-center">
                    "Momenta Counter"
                </h1>

                <div class="text-6xl font-bold text-center mb-8 transition-all duration-300 text-white">
                    {count}
                </div>

                <div class="flex gap-4 justify-center">
                    <button
                        class="px-6 py-3 bg-red-500 hover:bg-red-600 text-white font-semibold rounded-xl transition-all duration-200 transform hover:scale-105 shadow-lg"
                        on:click={move |_| count -= 1}
                    >
                        "− Decrease"
                    </button>

                    <button
                        class="px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-semibold rounded-xl transition-all duration-200 transform hover:scale-105 shadow-lg"
                        on:click={move |_| count += 1}
                    >
                        "+ Increase"
                    </button>
                </div>

                <button
                    class="w-full mt-4 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
                    on:click={move |_| count.set(0)}
                >
                    "Reset Count: " {count}
                </button>
            </div>
        </div>
    }
}

fn main() {
    render_root::<App>("#app");
}
