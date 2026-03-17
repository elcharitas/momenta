#![no_std]

extern crate alloc;
mod components;

use alloc::{format, vec};
use components::*;
use momenta::prelude::*;
use momenta_router::{RouterContext, RouterMode, routes};

// Main App
#[component]
fn App() -> Node {
    let router = RouterContext::new(RouterMode::Hash);
    let current_path = router.current_path();
    let theme = create_signal("dark");
    let mobile_menu_open = create_signal(false);
    create_effect(|| {
        highlightAll();
    });

    // Detect initial theme from localStorage
    create_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved)) = storage.get_item("theme") {
                    if saved == "light" {
                        theme.set("light");
                        if let Some(doc) = window.document() {
                            if let Some(el) = doc.document_element() {
                                let _ = el.class_list().remove_1("dark");
                            }
                        }
                    }
                }
            }
        }
    });

    rsx! {
        <div class="min-h-screen bg-background text-foreground transition-colors duration-200">
            <Header {theme} {mobile_menu_open} />

            <div class="flex pt-14">
                // Sidebar Navigation
                {when!(current_path.get() != "/" => <aside class="hidden lg:block w-64 shrink-0 border-r border-border/50">
                        <div class="sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto py-6 px-1">
                            <Navigation {router} />
                        </div>
                    </aside>
                )}

                // Mobile Navigation
                {when!(mobile_menu_open =>
                    <div class="lg:hidden fixed inset-0 z-50 flex">
                        <div class="fixed inset-0 bg-black/40 backdrop-blur-sm" on:click={move |_| mobile_menu_open.set(false)}></div>
                        <div class="relative flex w-full max-w-xs flex-col bg-background border-r border-border shadow-xl fade-in">
                            <div class="flex items-center justify-between px-5 py-4 border-b border-border/50">
                                <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground">Navigation</span>
                                <button type="button" on:click={move |_| mobile_menu_open.set(false)} class="p-2 rounded-md hover:bg-muted transition-colors">
                                    <i class="fas fa-times text-sm"></i>
                                </button>
                            </div>
                            <div class="overflow-y-auto p-4">
                                <Navigation {router} />
                            </div>
                        </div>
                    </div>
                )}

                // Main Content
                <main class="flex-1 min-w-0">
                    {when!(current_path.get() != "/" && !docs_on_this_page_sections(&current_path.get()).is_empty() =>
                        <div class="xl:hidden px-6 pt-5 sm:px-8 lg:px-10">
                            <OnThisPage current_path={current_path.get()} compact={true} />
                        </div>
                    )}

                    {routes!(router, current_path, {
                        "/" => |_| rsx! { <HomePage {router} /> },
                        "/getting-started" => |_| rsx! { <GettingStartedPage /> },
                        "/philosophy" => |_| rsx! { <PhilosophyPage /> },
                        "/rsx" => |_| rsx! { <RsxPage /> },
                        "/signals" => |_| rsx! { <SignalsPage /> },
                        "/computed-signals" => |_| rsx! { <ComputedSignalsPage /> },
                        "/effects" => |_| rsx! { <EffectsPage /> },
                        "/resources" => |_| rsx! { <ResourcesPage /> },
                        "/components" => |_| rsx! { <ComponentsPage /> },
                        "/classes" => |_| rsx! { <ClassesPage /> },
                        "/when" => |_| rsx! { <ShowPage /> },
                        "/lists" => |_| rsx! { <ForPage /> },
                        "/performance" => |_| rsx! { <PerformancePage /> },
                        "/deployment" => |_| rsx! { <DeploymentPage /> },
                        "/examples" => |_| rsx! { <ExamplesPage /> },
                        "/routing" => |_| rsx! { <RoutingPage /> },
                        "/routing/:section" => |_| rsx! { <RoutingPage /> },
                        "/examples/counter" => |_| rsx! { <CounterExample /> },
                        "/examples/todomvc" => |_| rsx! { <TodoMVCPage /> },
                        "/examples/hackernews" => |_| rsx! { <HackerNewsPage /> },
                        "/examples/realworld" => |_| rsx! { <RealWorldPage /> },
                    })}
                </main>

                {when!(current_path.get() != "/" && !docs_on_this_page_sections(&current_path.get()).is_empty() =>
                    <aside class="hidden xl:block w-72 shrink-0 border-l border-border/50 bg-background/55">
                        <div class="sticky top-14 max-h-[calc(100vh-3.5rem)] overflow-y-auto px-5 py-6">
                            <OnThisPage current_path={current_path.get()} compact={false} />
                        </div>
                    </aside>
                )}
            </div>
        </div>
    }
}

// Page Components
#[component]
fn HomePage(_props: &NavigationProps) -> Node {
    rsx! {
        <div class="fade-in">
            <div class="px-6 pt-20 pb-16 sm:px-8 text-center">
                <div class="inline-flex items-center gap-2 rounded-full border border-border/50 bg-card px-3 py-1 mb-6">
                    <span class="h-1.5 w-1.5 rounded-full bg-primary"></span>
                    <span class="text-xs font-medium text-muted-foreground">Built for humans and AI</span>
                </div>
                <h1 class="text-4xl font-bold tracking-tight sm:text-5xl lg:text-6xl">
                    "Rust UI that feels familiar" <br class="hidden sm:block" />
                    <span class="text-primary">"from the first read"</span>
                </h1>
                <p class="mt-4 text-sm text-muted-foreground max-w-2xl mx-auto leading-relaxed">
                    "If you already know Rust and can read HTML or JSX, Momenta keeps the mental model close to what you already know: components, markup, signals, derived values, and effects."
                </p>
                <div class="mt-8 flex flex-wrap items-center justify-center gap-3">
                    <a href="#/getting-started" class="inline-flex items-center gap-2 rounded-lg bg-primary px-5 py-2.5 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
                        "Read the Docs"
                        <i class="fas fa-arrow-right text-xs"></i>
                    </a>
                    <a href={GITHUB_LINK} class="inline-flex items-center gap-2 rounded-lg border border-border px-5 py-2.5 text-sm font-medium hover:bg-muted transition-colors">
                        <i class="fab fa-github"></i>
                        "GitHub"
                    </a>
                    <a href={CRATES_LINK} class="inline-flex items-center gap-2 rounded-lg border border-border px-5 py-2.5 text-sm font-medium hover:bg-muted transition-colors">
                        <i class="fas fa-cube"></i>
                        "Crates.io"
                    </a>
                </div>
                <div class="mt-6 inline-flex items-center gap-3 bg-card border border-border/50 rounded-lg px-4 py-2">
                    <span class="text-muted-foreground text-xs">"$"</span>
                    <code class="text-sm font-mono">"cargo add momenta"</code>
                </div>
            </div>

            <div class="mx-auto max-w-6xl px-6 sm:px-8 pb-16">
                <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
                    <Feature
                        icon="fas fa-code"
                        title="Familiar Syntax"
                        description="rsx! reads like HTML or JSX, so the shape of the UI is easy to scan at a glance."
                    />
                    <Feature
                        icon="fas fa-seedling"
                        title="Low Learning Curve"
                        description="Signals, computed values, and effects map cleanly to concepts most frontend developers already know."
                    />
                    <Feature
                        icon="fas fa-shield-alt"
                        title="AI-Friendly Structure"
                        description="State, derivation, and side effects stay explicit, which makes generated code easier to verify and maintain."
                    />
                    <Feature
                        icon="fas fa-cubes"
                        title="Rust-Native Components"
                        description="Momenta feels like Rust, not a JavaScript framework translated line by line into Rust syntax."
                    />
                    <Feature
                        icon="fas fa-bolt"
                        title="Direct Reactive Updates"
                        description="Track dependencies automatically and update the bindings that changed instead of rerunning everything."
                    />
                    <Feature
                        icon="fas fa-puzzle-piece"
                        title="Scales Without Magic"
                        description="Use the same primitives for a quick component, a routed app, or a larger codebase without hidden conventions."
                    />
                </div>
            </div>

            <div class="mx-auto max-w-6xl px-6 sm:px-8 pb-16">
                <div class="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
                    <div>
                        <h2 class="text-lg font-semibold">Why Momenta feels easy to adopt</h2>
                        <p class="mt-2 max-w-2xl text-sm leading-6 text-muted-foreground">
                            "The goal is not to invent a new language for UI. The goal is to let Rust developers move fast with patterns that are already widely understood by people, editors, and coding agents."
                        </p>
                    </div>
                </div>

                <div class="mt-6 grid gap-4 lg:grid-cols-3">
                    <div class="theory-panel not-prose">
                        <div class="flex h-10 w-10 items-center justify-center rounded-2xl border border-primary/15 bg-primary/10 text-primary">
                            <i class="fas fa-eye text-sm"></i>
                        </div>
                        <h3 class="mt-4 text-base font-semibold tracking-tight">Readable on the first pass</h3>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Markup looks like markup. Components look like Rust functions. Dynamic behavior is local to the expressions that actually change."
                        </p>
                    </div>
                    <div class="theory-panel not-prose">
                        <div class="flex h-10 w-10 items-center justify-center rounded-2xl border border-primary/15 bg-primary/10 text-primary">
                            <i class="fas fa-robot text-sm"></i>
                        </div>
                        <h3 class="mt-4 text-base font-semibold tracking-tight">Easy for AI to generate well</h3>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Clear separation between signals, computed values, effects, and components means less ambiguity when an AI suggests new code or edits existing code."
                        </p>
                    </div>
                    <div class="theory-panel not-prose">
                        <div class="flex h-10 w-10 items-center justify-center rounded-2xl border border-primary/15 bg-primary/10 text-primary">
                            <i class="fas fa-person-walking-arrow-right text-sm"></i>
                        </div>
                        <h3 class="mt-4 text-base font-semibold tracking-tight">No steep framework cliff</h3>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "You do not have to internalize a large runtime model before being productive. If you understand Rust ownership and basic reactive ideas, you can start shipping quickly."
                        </p>
                    </div>
                </div>
            </div>

            <div class="mx-auto max-w-6xl px-6 sm:px-8 pb-16">
                <h2 class="text-lg font-semibold mb-4">Explore</h2>
                <div class="grid gap-3 sm:grid-cols-2">
                    <a href="#/getting-started" class="card-link group">
                        <div class="flex items-start gap-3">
                            <div class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-primary/10 text-primary">
                                <i class="fas fa-rocket text-sm"></i>
                            </div>
                            <div>
                                <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Getting Started</h3>
                                <p class="text-xs text-muted-foreground mt-0.5">Install Momenta and build your first app.</p>
                            </div>
                        </div>
                    </a>
                    <a href="#/signals" class="card-link group">
                        <div class="flex items-start gap-3">
                            <div class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-primary/10 text-primary">
                                <i class="fas fa-wave-square text-sm"></i>
                            </div>
                            <div>
                                <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Signals</h3>
                                <p class="text-xs text-muted-foreground mt-0.5">Learn about fine-grained reactive primitives.</p>
                            </div>
                        </div>
                    </a>
                    <a href="#/components" class="card-link group">
                        <div class="flex items-start gap-3">
                            <div class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-primary/10 text-primary">
                                <i class="fas fa-cubes text-sm"></i>
                            </div>
                            <div>
                                <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Components</h3>
                                <p class="text-xs text-muted-foreground mt-0.5">Create reusable, composable UI components.</p>
                            </div>
                        </div>
                    </a>
                    <a href="#/examples" class="card-link group">
                        <div class="flex items-start gap-3">
                            <div class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-primary/10 text-primary">
                                <i class="fas fa-play text-sm"></i>
                            </div>
                            <div>
                                <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Examples</h3>
                                <p class="text-xs text-muted-foreground mt-0.5">See Momenta in action with live demos.</p>
                            </div>
                        </div>
                    </a>
                </div>
            </div>

            // Playground section
            <div class="mx-auto max-w-6xl px-6 sm:px-8 pb-16">
                <h2 class="text-lg font-semibold mb-4">Quick Example</h2>
                <p class="max-w-3xl text-sm leading-6 text-muted-foreground">
                    "This is the whole pitch in code form: a component, a signal, event handlers, and markup that still looks like the interface you are building."
                </p>
                <Playground
                    code={r#"use momenta::prelude::*;

#[component]
fn CounterExample() -> Node {
    let mut count = create_signal(0);
    rsx! {
        <div class="flex items-center justify-center p-6">
            <div class="text-center space-y-4">
                <h1 class="text-2xl font-bold">
                    "Momenta Counter"
                </h1>
                <div class="text-5xl font-bold">
                    {count}
                </div>
                <div class="flex gap-3 justify-center">
                    <button
                        class="px-5 py-2 bg-red-500 text-white rounded-lg"
                        on:click={move |_| count -= 1}
                    >
                        "Decrease"
                    </button>
                    <button
                        class="px-5 py-2 bg-green-500 text-white rounded-lg"
                        on:click={move |_| count += 1}
                    >
                        "Increase"
                    </button>
                </div>
                <button
                    class="px-4 py-1.5 border rounded-lg text-sm"
                    on:click={move |_| count.set(0)}
                >
                    "Reset"
                </button>
            </div>
        </div>
    }
}"#} />
            </div>
            <div class="mx-auto max-w-6xl px-6 sm:px-8 pb-16">
                <h2 class="text-lg font-semibold">Momenta compared to familiar alternatives</h2>
                <p class="mt-2 max-w-3xl text-sm leading-6 text-muted-foreground">
                    "Momenta is designed to feel approachable whether you are coming from React, exploring fine-grained Rust frameworks, or trying to avoid low-level DOM boilerplate. The tradeoff is deliberate explicitness instead of hidden framework magic."
                </p>

                <div class="mt-6 grid gap-4 lg:grid-cols-3">
                    <div class="theory-panel not-prose">
                        <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">If you know React or Preact</div>
                        <h3 class="mt-3 text-base font-semibold tracking-tight">Familiar view code, less rerender-oriented thinking</h3>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "You still write components and HTML-like markup, but Momenta prefers explicit signals and targeted reactive updates over hook-heavy rerender cycles."
                        </p>
                    </div>
                    <div class="theory-panel not-prose">
                        <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">If you are evaluating Leptos or Solid-style ideas</div>
                        <h3 class="mt-3 text-base font-semibold tracking-tight">Comparable fine-grained mindset, Rust-first surface area</h3>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Momenta keeps fine-grained reactivity front and center, while aiming for an API that stays close to ordinary Rust and remains easy to explain from the code itself."
                        </p>
                    </div>
                    <div class="theory-panel not-prose">
                        <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">If you are considering lower-level or heavier Rust UI stacks</div>
                        <h3 class="mt-3 text-base font-semibold tracking-tight">Less boilerplate, fewer abstractions to fight</h3>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Momenta gives you components, routing, and reactivity primitives without forcing you into a large conceptual layer between your Rust code and the UI you want to build."
                        </p>
                    </div>
                </div>
            </div>

            // Footer
            <footer class="border-t border-border/50 mt-8">
                <div class="px-6 sm:px-8 py-12">
                    <div class="grid gap-8 sm:grid-cols-2 lg:grid-cols-4">
                        <div>
                            <div class="flex items-center gap-2 mb-4">
                                <img src="./static/icon.svg" alt="Momenta" class="w-5 h-5" />
                                <span class="font-semibold text-sm">Momenta</span>
                            </div>
                            <p class="text-xs text-muted-foreground leading-relaxed">
                                "A Rust UI framework with familiar syntax, explicit reactivity, and code that is comfortable for both humans and AI to work with."
                            </p>
                        </div>
                        <div>
                            <h4 class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-3">Documentation</h4>
                            <div class="space-y-2">
                                <a href="#/getting-started" class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Getting Started</a>
                                <a href="#/signals" class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Signals</a>
                                <a href="#/components" class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Components</a>
                                <a href="#/rsx" class="block text-sm text-muted-foreground hover:text-foreground transition-colors">RSX Syntax</a>
                            </div>
                        </div>
                        <div>
                            <h4 class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-3">Examples</h4>
                            <div class="space-y-2">
                                <a href="#/examples/counter" class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Counter</a>
                                <a href="#/examples/todomvc" class="block text-sm text-muted-foreground hover:text-foreground transition-colors">TodoMVC</a>
                                <a href="#/examples/hackernews" class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Hacker News</a>
                                <a href="#/examples/realworld" class="block text-sm text-muted-foreground hover:text-foreground transition-colors">RealWorld</a>
                            </div>
                        </div>
                        <div>
                            <h4 class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-3">Community</h4>
                            <div class="space-y-2">
                                <a href={GITHUB_LINK} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">GitHub</a>
                                <a href={CRATES_LINK} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Crates.io</a>
                            </div>
                        </div>
                    </div>
                    <div class="mt-10 pt-6 border-t border-border/50">
                        <p class="text-xs text-muted-foreground">"Built with Momenta. Open source under the MIT License."</p>
                    </div>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn Feature(props: &FeatureProps) -> Node {
    rsx! {
        <div class="card-link group rounded-lg p-4">
            <div class="mb-2.5 flex h-9 w-9 items-center justify-center rounded-md bg-primary/10 text-primary">
                <i class={format!("{} text-sm", props.icon)}></i>
            </div>
            <h3 class="mb-1 font-medium text-sm">{props.title}</h3>
            <p class="text-xs text-muted-foreground leading-relaxed">{props.description}</p>
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
        <article class="px-6 py-10 sm:px-8 lg:px-10 xl:pr-8 2xl:pr-10 fade-in">
            <DocPageHeader
                title="Computed Signals"
                summary="Use computed values when state can be described as a function of other state. Momenta keeps those relationships explicit so derived data stays cheap, predictable, and easy to reason about."
                chips={vec!["derived state", "automatic dependency tracking", "memoized reads"]}
                stats={vec![
                    ("Best for", "Values that should stay in sync with source signals"),
                    ("Avoid", "Storing duplicated state that can drift out of date"),
                    ("Mental model", "A live formula instead of another store"),
                ]}
            />

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="introduction">Introduction</h2>
                <p>
                    Computed signals are reactive values that automatically recalculate when their dependencies change.
                    They are perfect for derived state and expensive computations that should be cached.
                </p>

                <div class="doc-grid my-8">
                    <TheoryCard icon="fas fa-link" title="Computed values model relationships">
                        <p>
                            "Reach for a computed signal when the value should always be derivable from other signals. That keeps source-of-truth state small and removes synchronization bugs."
                        </p>
                    </TheoryCard>
                    <TheoryCard icon="fas fa-scale-balanced" title="Keep derivation pure">
                        <p>
                            "A computed closure should answer a question, not perform work with side effects. When you need logging, network calls, or DOM coordination, use an effect instead."
                        </p>
                    </TheoryCard>
                </div>

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
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <header class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">Dynamic Classes</h1>
                <p class="mt-3 text-lg text-muted-foreground leading-relaxed">
                    Learn how to work with dynamic CSS classes using the class! macro and classes() function.
                </p>
            </header>

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="introduction" class="font-bold tracking-tight">Introduction</h2>
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

                <h2 class="font-bold tracking-tight">Best Practices</h2>
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
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <header class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">List Rendering</h1>
                <p class="mt-3 text-lg text-muted-foreground leading-relaxed">
                    "Rendering lists of items efficiently using Rust's iterators."
                </p>
            </header>
            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
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
            <h2 class="font-bold tracking-tight">"Fruit List"</h2>
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
            <h2 class="font-bold tracking-tight">"Todo List"</h2>
            <ul class="space-y-2">
                {todos.map(|todo| {
                    let id = todo.id;
                    rsx! {
                        <li class={format!("flex items-center {}",
                            if todo.completed { "line-through text-muted-foreground" } else { "" }
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
                    on:input={move |e: web_sys::Event| {
                        if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                            new_todo_text.set(input.value());
                        }
                    }}
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
                    <li class="bg-muted p-2 rounded text-center">
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

                <h2 class="font-bold tracking-tight">Best Practices</h2>
                <ul>
                    <li>"Use .iter().map() pattern for list rendering"</li>
                    <li>"Extract complex item rendering into separate components"</li>
                    <li>"Use signals for list data that changes over time"</li>
                    <li>"Leverage Rust's powerful iterator methods for filtering, sorting, and transforming data"</li>
                    <li>"Consider memoizing expensive computations for list items"</li>
                </ul>

                <div class="mt-16 flex items-center justify-between border-t border-border pt-8">
                    <a href="#/when" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← Conditional Rendering"
                    </a>
                    <a href="#/performance" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Performance →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
fn ResourcesPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <header class="mb-10">
                <h1 title="" class="text-3xl font-bold tracking-tight sm:text-4xl">Resources</h1>
                <p class="mt-3 text-lg text-muted-foreground leading-relaxed">
                    "Resources provide a reactive wrapper for async loading with an explicit status signal and retry support."
                </p>
            </header>

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 class="font-bold tracking-tight">Introduction</h2>
                <p>
                    "Resources in Momenta are reactive primitives for async work such as API calls and background loading.
                    A resource gives you the latest resolved value, a status signal you can render from, and a retry() method to trigger the fetcher again."
                </p>

                <Note variant="info">
                    <p>
                        <strong>"Import path:"</strong> " Resource APIs currently live under momenta::signals, so examples here import create_resource and ResourceStatus from there alongside the prelude."
                    </p>
                </Note>

                <h2 id="basic-usage" class="font-bold tracking-tight">Basic Usage</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;
use momenta::signals::{create_resource, ResourceStatus};

#[component]
fn UserProfile() -> Node {
    let user_resource = create_resource(|| async {
        fetch_user_data().await
    });

    rsx! {
        <div>
            {match user_resource.status().get() {
                ResourceStatus::Idle | ResourceStatus::Pending | ResourceStatus::Loading => rsx! {
                    <div class="loading">"Loading user data..."</div>
                },
                ResourceStatus::Resolved => rsx! {
                    <h1>"User: " {user_resource.get().unwrap_or_default()}</h1>
                },
            }}
        </div>
    }
}

async fn fetch_user_data() -> String {
    "John Doe".to_string()
}"#}
                />

                <Note variant="info">
                    <p>
                        <strong>"Resource model:"</strong> " A resource tracks progress with ResourceStatus and stores the latest resolved value as Option&lt;T&gt;."
                    </p>
                </Note>

                <h2 id="creating-resources" class="font-bold tracking-tight">Creating Resources</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::signals::create_resource;

let user_data = create_resource(|| async {
    fetch_user().await
});

let user_posts = create_resource(move || {
    let user_id = user_id.get();
    async move {
        fetch_user_posts(user_id).await
    }
});

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

                <h2 id="resource-states" class="font-bold tracking-tight">Resource States</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::signals::{create_resource, ResourceStatus};

#[component]
fn ResourceStates() -> Node {
    let data_resource = create_resource(|| async {
        fetch_data().await
    });

    rsx! {
        <div>
            <p>"Status: " {format!("{:?}", data_resource.status().get())}</p>
            {when!(data_resource.status().get() == ResourceStatus::Resolved =>
                <div class="success">
                    "Data: " {data_resource.get().unwrap_or_default()}
                </div>
            else
                <div class="loading">"Waiting for data..."</div>
            )}
        </div>
    }
}

async fn fetch_data() -> String {
    "Successfully loaded data".to_string()
}"#}
                />

                <h2 id="retrying-resources" class="font-bold tracking-tight">Retrying Resources</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::signals::{create_resource, ResourceStatus};

#[component]
fn RetryableResource() -> Node {
    let api_resource = create_resource(|| async {
        fetch_api_data().await
    });

    rsx! {
        <div>
            {when!(api_resource.status().get() == ResourceStatus::Resolved =>
                <div>{api_resource.get().unwrap_or_default()}</div>
            else
                <div>
                    <p>"Fetch is not resolved yet"</p>
                    <button on:click={move |_| api_resource.retry()}>"Try Again"</button>
                </div>
            )}
        </div>
    }
}"#}
                />

                <h2 id="reactive-dependencies" class="font-bold tracking-tight">Reactive Dependencies</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::signals::{create_resource, ResourceStatus};

#[component]
fn ReactiveResource() -> Node {
    let user_id = create_signal(1);

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

            {when!(user_profile.status().get() == ResourceStatus::Resolved =>
                <div>{user_profile.get().unwrap_or_default()}</div>
            else
                <div>"Loading profile..."</div>
            )}
        </div>
    }
}"#}
                />

                <h2 id="reading-without-cloning" class="font-bold tracking-tight">Reading Without Cloning</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::signals::create_resource;

let stories = create_resource(|| async {
    fetch_stories().await
});

let story_count = stories.with(|items| items.len()).unwrap_or(0);"#}
                />

                <h2 id="combining-with-effects" class="font-bold tracking-tight">Combining with Effects</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::signals::{create_resource, ResourceStatus};

#[component]
fn ResourceWithEffects() -> Node {
    let data_resource = create_resource(|| async {
        fetch_important_data().await
    });

    create_effect(move || {
        match data_resource.status().get() {
            ResourceStatus::Resolved => {
                let data = data_resource.get().unwrap_or_default();
                log!("Data loaded successfully: {}", data);
            }
            ResourceStatus::Idle | ResourceStatus::Pending | ResourceStatus::Loading => {
                log!("Waiting for resource to resolve");
            }
        }
    });

    rsx! {
        <div>
            {when!(data_resource.status().get() == ResourceStatus::Resolved =>
                <div>{data_resource.get().unwrap_or_default()}</div>
            else
                <div>"Processing..."</div>
            )}
        </div>
    }
}"#}
                />

                <h2 id="best-practices" class="font-bold tracking-tight">Best Practices</h2>
                <ul>
                    <li>"Use create_resource for any asynchronous data loading"</li>
                    <li>"Render from status() when you need explicit loading state transitions"</li>
                    <li>"Use retry() to move a resource back to Pending and run the fetcher again"</li>
                    <li>"Resources automatically re-run when their dependencies change"</li>
                    <li>"Use with() when you want to read resolved data without cloning it"</li>
                    <li>"Combine resources with effects when status transitions need side effects"</li>
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
        <article class="px-6 py-10 sm:px-8 lg:px-10 xl:pr-8 2xl:pr-10 fade-in">
            <DocPageHeader
                title="Philosophy"
                summary="Momenta is built around explicit reactivity, direct DOM updates, and Rust-native ergonomics. The goal is not to imitate JavaScript frameworks in Rust, but to make reactive UI programming feel natural inside Rust’s constraints and strengths."
                chips={vec!["explicit primitives", "direct updates", "Rust-first design"]}
                stats={vec![
                    ("Rendering", "Update the exact DOM nodes that depend on changed data"),
                    ("State", "Represent change with signals and derived values"),
                    ("Tradeoff", "A little more explicitness for much clearer runtime behavior"),
                ]}
            />

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="mental-model" class="font-bold tracking-tight">Mental Model</h2>
                <p>
                    "Think of Momenta as a system that remembers which rendered values depend on which signals. A read creates a dependency, and a write only re-runs the bindings or computations attached to that dependency."
                </p>
                <ol>
                    <li>
                        <strong>Read.</strong>
                        " A signal is used inside RSX, a computed value, or an effect."
                    </li>
                    <li>
                        <strong>Track.</strong>
                        " Momenta records that dependency instead of waiting to diff an entire view later."
                    </li>
                    <li>
                        <strong>Update.</strong>
                        " When the signal changes, only the affected bindings are notified."
                    </li>
                </ol>

                <h2 id="core-principles" class="font-bold tracking-tight">Core Principles</h2>
                <p>
                    "The philosophy is straightforward: make updates precise, keep the reactive model explicit, and let Rust stay visible instead of wrapping it in framework magic."
                </p>
                <ol>
                    <li>
                        <strong>Element-level reactivity.</strong>
                        " Update the exact DOM bindings that depend on changed state instead of re-running unrelated view code."
                    </li>
                    <li>
                        <strong>Rust-first design.</strong>
                        " Lean on Rust’s ownership model, traits, and macros instead of hiding them behind a JavaScript-style abstraction layer."
                    </li>
                    <li>
                        <strong>Explicit primitives.</strong>
                        " Signals, computed values, effects, and resources are separate tools with clear jobs, which keeps data flow easier to reason about."
                    </li>
                    <li>
                        <strong>Composition over configuration.</strong>
                        " Build larger behavior from small primitives instead of depending on convention-heavy framework features."
                    </li>
                </ol>

                <h2 id="why-not-virtual-dom" class="font-bold tracking-tight">Why Not Virtual DOM?</h2>
                <p>
                    "Virtual DOM solves the problem of coordinating large re-renders. Momenta takes a different route: track dependencies as they are read, then update only the exact outputs that depend on changed data."
                </p>

                <div class="theory-panel not-prose my-8">
                    <div class="grid gap-3 md:grid-cols-2">
                        <div class="rounded-xl border border-border/60 bg-background/80 px-3.5 py-3">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">What Momenta gains</div>
                            <p class="mt-1.5 text-[13px] leading-5 text-muted-foreground">"No full component diff pass, less incidental work, and runtime behavior that is easier to predict when state changes."</p>
                        </div>
                        <div class="rounded-xl border border-border/60 bg-background/80 px-3.5 py-3">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">What Momenta asks from you</div>
                            <p class="mt-1.5 text-[13px] leading-5 text-muted-foreground">"You model state more deliberately and choose whether something should be source state, derived state, or an effect."</p>
                        </div>
                    </div>
                </div>

                <h2 id="framework-comparison" class="font-bold tracking-tight">Comparison with Other Frameworks</h2>
                <p>
                    "Momenta is closer to fine-grained reactive systems like Solid than to component re-render models like React. The key difference is that it applies that model directly in Rust, with macros and types that fit the language instead of mimicking JavaScript framework APIs."
                </p>

                <div class="theory-panel not-prose my-8">
                    <div class="grid gap-4 md:grid-cols-3">
                        <div class="rounded-xl border border-border/60 bg-background/80 px-3.5 py-3">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">React-style model</div>
                            <p class="mt-1.5 text-[13px] leading-5 text-muted-foreground">"Re-run component logic, produce a new tree, reconcile, then commit changes."</p>
                        </div>
                        <div class="rounded-xl border border-border/60 bg-background/80 px-3.5 py-3">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">Momenta model</div>
                            <p class="mt-1.5 text-[13px] leading-5 text-muted-foreground">"Track reads once, then send updates directly to the exact bindings that changed."</p>
                        </div>
                        <div class="rounded-xl border border-border/60 bg-background/80 px-3.5 py-3">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">Practical outcome</div>
                            <p class="mt-1.5 text-[13px] leading-5 text-muted-foreground">"Less incidental work, fewer rendering phases to reason about, and a tighter fit with Rust’s explicit style."</p>
                        </div>
                    </div>
                </div>

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
        <article class="px-6 py-10 sm:px-8 lg:px-10 xl:pr-8 2xl:pr-10 fade-in">
            <DocPageHeader
                title="rsx!"
                summary="The rsx! macro is Momenta’s declarative view language. It lets you describe structure, bind reactive values, and compose components while still staying firmly inside Rust’s type system and macro model."
                chips={vec!["HTML-like syntax", "typed attributes", "reactive children"]}
                stats={vec![
                    ("Purpose", "Describe UI structure declaratively"),
                    ("Compiles to", "A tree of lightweight Momenta nodes"),
                    ("Best used with", "Signals, components, and control-flow macros"),
                ]}
            />
            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "RSX allows you to write HTML-like syntax inside Rust code. It's a way to declaratively describe the structure of your UI."
                </p>

                <div class="doc-grid my-8">
                    <TheoryCard icon="fas fa-sitemap" title="Think in structure first">
                        <p>
                            "Use rsx! to express what the UI should look like when given the current state. Keep business rules outside the markup, then feed the result into a small, readable view tree."
                        </p>
                    </TheoryCard>
                    <TheoryCard icon="fas fa-arrow-right-arrow-left" title="Bindings stay local and explicit">
                        <p>
                            "Values become dynamic only where you interpolate them. That locality is important: it keeps reactive reads obvious and makes it easier to see why a specific text node or attribute updates."
                        </p>
                    </TheoryCard>
                </div>

                <div class="theory-panel not-prose my-8">
                    <div class="text-[11px] font-semibold uppercase tracking-[0.18em] text-primary">How to read RSX</div>
                    <h2 class="mt-3 text-2xl font-bold tracking-tight">A compact mental model</h2>
                    <div class="mt-4 grid gap-3 md:grid-cols-3">
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">Static parts</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"Literal tags, text, and attributes compile to fixed node structure."</p>
                        </div>
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">Dynamic parts</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"Interpolations, conditional blocks, and mapped lists establish reactive bindings."</p>
                        </div>
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">Composition</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"Components are just more nodes, so rsx! stays consistent as your UI grows."</p>
                        </div>
                    </div>
                </div>
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
            <p>"Welcome to Momenta."</p>
            <p>"😉"</p>
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
"#}
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
        {items.map(|item| <li>{item}</li>)}
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

                <h2 class="font-bold tracking-tight">Best Practices</h2>
                <ul>
                    <li>"Prefer quoted text nodes for predictable RSX output"</li>
                    <li>"Use underscores for hyphenated attribute names like data_id"</li>
                    <li>"Keep your components small and focused on a single responsibility"</li>
                    <li>"Use signals for state that changes over time"</li>
                    <li>"Extract repeated patterns into reusable components"</li>
                    <li>"Use the when! macro for conditional rendering"</li>
                    <li>"Use direct .map() closures for rendering lists"</li>
                </ul>

                <div class="mt-16 flex items-center justify-between border-t border-border pt-8">
                    <a href="#/philosophy" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← Philosophy"
                    </a>
                    <a href="#/signals" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
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
        <article class="px-6 py-10 sm:px-8 lg:px-10 xl:pr-8 2xl:pr-10 fade-in">
            <DocPageHeader
                title="Effects"
                summary="Effects connect Momenta’s reactive graph to the outside world. Use them to synchronize logging, timers, DOM APIs, and subscriptions with signal changes, while keeping pure derivation in computed values instead."
                chips={vec!["tracked dependencies", "cleanup", "side-effect boundaries"]}
                stats={vec![
                    ("Use for", "Imperative work that must follow reactive state"),
                    ("Avoid for", "Derived values that could stay pure and computed"),
                    ("Key rule", "Clean up anything you create outside the reactive graph"),
                ]}
            />
            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "Effects are functions that run when their dependencies change. They are the building blocks of reactivity in Momenta. Effects automatically track any signals accessed during their execution and re-run when those signals change."
                </p>
                <div class="doc-grid my-8">
                    <TheoryCard icon="fas fa-plug" title="Effects are integration points">
                        <p>
                            "An effect is where declarative state meets imperative APIs. That includes console logging, timers, browser listeners, subscriptions, and any bridge to code that Momenta does not own."
                        </p>
                    </TheoryCard>
                    <TheoryCard icon="fas fa-ban" title="Do not use effects as extra state containers">
                        <p>
                            "If you are writing an effect only to keep another value in sync, you likely want a computed signal instead. Effects are for performing work, not modeling relationships between values."
                        </p>
                    </TheoryCard>
                </div>
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
                <h2 class="font-bold tracking-tight">Advanced Patterns</h2>
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

                <h3>Effects with Cleanup</h3>
                <p>"Use create_effect_with_cleanup when your effect sets up resources that need teardown (intervals, listeners, subscriptions):"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

let is_polling = create_signal(true);

create_effect_with_cleanup(move || {
    if is_polling.get() {
        // Set up a polling interval
        let interval_id = set_interval(|| {
            log!("Polling for updates...");
        }, 5000);

        // Return a cleanup function — called before next run
        // or when the effect is destroyed
        move || {
            clear_interval(interval_id);
        }
    } else {
        // No cleanup needed
        || {}
    }
});"#}
                />

                <Note variant="warning">
                    <p>
                        <strong>"Important:"</strong> " Always use create_effect_with_cleanup when your effect creates timers, event listeners, or WebSocket connections. The cleanup function prevents resource leaks."
                    </p>
                </Note>

                <h3>Batch Updates</h3>
                <p>"When updating multiple signals at once, wrap them in batch() to trigger only a single re-render:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

let count = create_signal(0);
let name = create_signal("hello".to_string());
let items = create_signal(vec![1, 2, 3]);

// Without batch: each .set() triggers a re-render (3 renders)
count.set(1);
name.set("world".to_string());
items.push(4);

// With batch: all updates applied, then a single re-render
batch(|| {
    count.set(1);
    name.set("world".to_string());
    items.push(4);
}); // Single re-render happens here"#}
                />

                <Note variant="tip">
                    <p>
                        <strong>"Performance:"</strong> " Use batch() in event handlers that update multiple signals. This avoids intermediate renders and improves performance."
                    </p>
                </Note>

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
                <h2 class="font-bold tracking-tight">Best Practices</h2>
                <ul>
                    <li>"Keep effects as lightweight as possible"</li>
                    <li>"Avoid creating effects inside loops or other complex logic"</li>
                    <li>"Use " <code>"create_effect"</code> " for simple effects"</li>
                    <li>"Don't modify signals that you're tracking in the same effect to avoid infinite loops"</li>
                    <li>"Group related effects together for better code organization"</li>
                </ul>
                <div class="mt-16 flex items-center justify-between border-t border-border pt-8">
                    <a href="#/computed-signals" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← Computed Signals"
                    </a>
                    <a href="#/resources" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
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
        <article class="px-6 py-10 sm:px-8 lg:px-10 xl:pr-8 2xl:pr-10 fade-in">
            <DocPageHeader
                title="Signals"
                summary="Signals are Momenta’s source-of-truth state containers. Reading a signal creates a tracked dependency; writing to it invalidates only the computations and DOM bindings that actually depend on that value."
                chips={vec!["tracked reads", "targeted updates", "typed state"]}
                stats={vec![
                    ("Use for", "Mutable state that changes over time"),
                    ("Read path", "Reads register dependencies automatically"),
                    ("Write path", "Writes notify only interested dependents"),
                ]}
            />

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "When you create a signal, you get a getter and setter function. The getter tracks any scope it's called in,
                    and the setter triggers updates to any computations that depend on the signal's value."
                </p>

                <div class="doc-grid my-8">
                    <TheoryCard icon="fas fa-eye" title="Reads are meaningful">
                        <p>
                            "Calling .get() is not just data access. It tells the runtime that the current reactive scope depends on this signal. That is why dependency tracking stays precise."
                        </p>
                    </TheoryCard>
                    <TheoryCard icon="fas fa-pen-to-square" title="Writes should model real change">
                        <p>
                            "Use signals for values that genuinely evolve over time. If a value can be derived from other signals, keep it derived instead of storing a duplicate copy."
                        </p>
                    </TheoryCard>
                    <TheoryCard icon="fas fa-layer-group" title="Granularity is a design tool">
                        <p>
                            "Splitting state into smaller signals gives Momenta more opportunities to skip work. Prefer several focused signals over one large bucket of unrelated fields when the update patterns differ."
                        </p>
                    </TheoryCard>
                </div>

                <div class="theory-panel not-prose my-8">
                    <div class="text-[11px] font-semibold uppercase tracking-[0.18em] text-primary">Lifecycle</div>
                    <h2 class="mt-3 text-2xl font-bold tracking-tight">What happens when a signal changes?</h2>
                    <div class="mt-4 grid gap-3 md:grid-cols-4">
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">1. Create</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"A signal stores a typed value and starts with no dependents."</p>
                        </div>
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">2. Read</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"Reactive scopes that call .get() register themselves as dependents."</p>
                        </div>
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">3. Write</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"Calling .set() or using an update helper marks those dependents dirty."</p>
                        </div>
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">4. Flush</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"Momenta re-runs only the necessary computations and patches the exact affected DOM nodes."</p>
                        </div>
                    </div>
                </div>

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

                <h2 id="operator-overloads">Operator Overloads</h2>
                <p>"Signals support arithmetic assignment operators for concise updates:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let mut count = create_signal(10);

count += 5;   // count is now 15
count -= 3;   // count is now 12
count *= 2;   // count is now 24
count /= 4;   // count is now 6

// Comparison operators work directly
let is_big = count > 5;    // true
let is_exact = count == 6; // true"#}
                />

                <h2 id="with-method">The .with() Method</h2>
                <p>"Access the signal value by reference without cloning, useful for expensive-to-clone types:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let items = create_signal(vec![1, 2, 3, 4, 5]);

// Immutable access without cloning the whole Vec
let length = items.with(|v| v.len()); // Some(5)
let first = items.with(|v| v.first().cloned()); // Some(Some(1))"#}
                />

                <h2 id="boolean-signals">Boolean Signal Methods</h2>
                <p>"Signal&lt;bool&gt; has convenience methods for common operations:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let is_open = create_signal(false);

is_open.toggle();    // flips to true
is_open.toggle();    // flips back to false
is_open.turn_on();   // sets to true
is_open.turn_off();  // sets to false

rsx! {
    <button on:click={move |_| is_open.toggle()}>
        {when!(is_open => "Close" else "Open")}
    </button>
}"#}
                />

                <h2 id="then-method">The .then() Method</h2>
                <p>"Boolean-like signals can run a closure conditionally and return the result as an Option:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let is_logged_in = create_signal(true);

let greeting = is_logged_in.then(|| {
    "Welcome back".to_string()
});

assert_eq!(greeting, Some("Welcome back".to_string()));

is_logged_in.set(false);
assert_eq!(is_logged_in.then(|| "secret area"), None);"#}
                />

                <h2 id="vec-signals">Vector Signal Methods</h2>
                <p>"Signal&lt;Vec&lt;T&gt;&gt; provides familiar collection methods that automatically trigger updates:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let items = create_signal(vec!["apple", "banana"]);

// Mutating methods
items.push("cherry");          // append
items.insert(0, "avocado");    // insert at index
items.remove(1);               // remove by index
let last = items.pop();        // remove and return last

// Filtering
items.retain(|item| item.starts_with('a'));

// Querying
let count = items.len();        // length
let empty = items.is_empty();   // check empty
let first = items.get_at(0);    // get by index

// Updating
items.update_at(0, "apricot");  // update by index
items.clear();                  // remove all

// Sorting (requires T: Ord)
let numbers = create_signal(vec![3, 1, 2]);
numbers.sort();                  // [1, 2, 3]
numbers.reverse();               // [3, 2, 1]
numbers.sort_by_key(|n| -n);    // sort descending

// Iteration and mapping
let doubled = numbers.map(|n| n * 2);  // Vec<i32>

rsx! {
    <ul>
        {items.map(|item| rsx! { <li>{item}</li> })}
    </ul>
}"#}
                />

                <h2 id="signal-value">Custom Types with SignalValue</h2>
                <p>"To store custom types in signals, derive the SignalValue trait:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[derive(Clone, PartialEq, SignalValue)]
struct User {
    name: String,
    age: u32,
}

let user = create_signal(User {
    name: "Alice".to_string(),
    age: 30,
});

// Access fields via .with()
let name = user.with(|u| u.name.clone());

// Update the whole value
user.set(User {
    name: "Bob".to_string(),
    age: 25,
});"#}
                />
                <Note variant="info">
                    <p>
                        <strong>"Built-in support:"</strong> " SignalValue is already implemented for all numeric types, bool, char, String, &'static str, Vec&lt;T&gt;, and Option&lt;T&gt;."
                    </p>
                </Note>

                <h2 class="font-bold tracking-tight">Best Practices</h2>
                <ul>
                    <li>"Keep signals at the appropriate scope - not everything needs to be global state"</li>
                    <li>"Prefer fine-grained signals over large state objects for better performance"</li>
                    <li>"Group related signals into structs for better organization"</li>
                    <li>"Use derived values (closures that read signals) instead of creating redundant signals"</li>
                    <li>"Consider using custom signal types for domain-specific state management"</li>
                </ul>

                <div class="mt-16 flex items-center justify-between border-t border-border pt-8">
                    <a href="#/rsx" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← rsx!"
                    </a>
                    <a href="#/computed-signals" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Computed Signals →"
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
        <article class="px-6 py-10 sm:px-8 lg:px-10 xl:pr-8 2xl:pr-10 fade-in">
            <DocPageHeader
                title="Getting Started"
                summary="Set up the toolchain, mount your first component, and understand the three moving parts behind every Momenta app: RSX for structure, signals for state, and the runtime for precise DOM updates."
                chips={vec!["Trunk", "WASM target", "mount_to_body"]}
                stats={vec![
                    ("Build tool", "Trunk handles WASM, assets, and dev-server reloads"),
                    ("Core loop", "State changes flow from signals into RSX bindings"),
                    ("Outcome", "A small Rust app running directly in the browser"),
                ]}
            />

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <div class="doc-grid my-8">
                    <TheoryCard icon="fas fa-diagram-project" title="What you are wiring together">
                        <p>
                            "A Momenta app is just a Rust binary that renders a component tree into the browser. The rsx! macro defines the shape of that tree, signals hold changeable data, and the runtime keeps the DOM synchronized."
                        </p>
                    </TheoryCard>
                    <TheoryCard icon="fas fa-flag-checkered" title="What you should understand by the end">
                        <p>
                            "After this page, you should be able to bootstrap a project, identify where reactive state lives, and know where to go next for syntax, component structure, and reactive primitives."
                        </p>
                    </TheoryCard>
                </div>

                <div class="theory-panel not-prose my-8">
                    <div class="text-[11px] font-semibold uppercase tracking-[0.18em] text-primary">Before You Code</div>
                    <h2 class="mt-2.5 text-xl font-bold tracking-tight">The shortest useful mental model</h2>
                    <div class="mt-3 grid gap-2.5 md:grid-cols-3">
                        <div class="rounded-xl border border-border/60 bg-background/80 px-3.5 py-3">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">1. Build</div>
                            <p class="mt-1.5 text-[13px] leading-5 text-muted-foreground">"Trunk compiles your Rust crate to WebAssembly and serves the generated assets."</p>
                        </div>
                        <div class="rounded-xl border border-border/60 bg-background/80 px-3.5 py-3">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">2. Mount</div>
                            <p class="mt-1.5 text-[13px] leading-5 text-muted-foreground">"mount_to_body::<App>() boots the root component and attaches it to the document."</p>
                        </div>
                        <div class="rounded-xl border border-border/60 bg-background/80 px-3.5 py-3">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">3. React</div>
                            <p class="mt-1.5 text-[13px] leading-5 text-muted-foreground">"Signal writes trigger precise updates, so the browser only changes what the user can actually see."</p>
                        </div>
                    </div>
                </div>

                <h2 id="prerequisites" class="font-bold tracking-tight">Prerequisites</h2>
                <p>"Before getting started, make sure you have Rust and the WebAssembly target installed:"</p>
                <CodeBlock
                    filename="terminal"
                    highlight=""
                    language="bash"
                    code={r#"# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the WebAssembly compilation target
rustup target add wasm32-unknown-unknown

# Install Trunk (the recommended build tool)
cargo install trunk"#}
                />
                <Note variant="info">
                    <p>
                        <strong>"Trunk"</strong> " is the recommended build tool for Momenta. It handles WASM compilation, asset bundling, and live reloading during development."
                    </p>
                </Note>

                <h2 id="installation" class="font-bold tracking-tight">Installation</h2>
                <p>"Create a new Rust project and add Momenta to your " <code>"Cargo.toml"</code> ":"</p>
                <CodeBlock
                    filename="terminal"
                    highlight=""
                    language="bash"
                    code={r#"cargo new my-app
    cd my-app"#}
                />
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

                <h2 id="create-index-html" class="font-bold tracking-tight">Create index.html</h2>
                <p>"Trunk needs an index.html entry point. Create one at your project root:"</p>
                <CodeBlock
                    filename="index.html"
                    highlight=""
                    language="html"
                    code={r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>My Momenta App</title>
    <link data-trunk rel="rust" />
  </head>
  <body>
    <div id="app"></div>
  </body>
</html>"#}
                />

                <h2 id="first-component" class="font-bold tracking-tight">Your First Component</h2>
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

                <h2 id="project-structure" class="font-bold tracking-tight">Project Structure</h2>
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

                <h2 id="run-your-app" class="font-bold tracking-tight">Run Your App</h2>
                <p>"Start the development server with live reloading:"</p>
                <CodeBlock
                    filename="terminal"
                    highlight=""
                    language="bash"
                    code={r#"# Start the dev server with live reload
trunk serve

# Or build for production
trunk build --release"#}
                />
                <p>"Open " <code>"http://localhost:8080"</code> " in your browser. Trunk will automatically recompile and reload when you save changes."</p>

                <h2 id="next-steps" class="font-bold tracking-tight">Next Steps</h2>
                <div class="grid gap-3 sm:grid-cols-2 not-prose">
                    <a href="#/rsx" class="card-link group">
                        <h3 class="text-sm font-medium group-hover:text-primary transition-colors">RSX Syntax</h3>
                        <p class="text-xs text-muted-foreground mt-0.5">Learn the JSX-like template syntax.</p>
                    </a>
                    <a href="#/signals" class="card-link group">
                        <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Signals</h3>
                        <p class="text-xs text-muted-foreground mt-0.5">Understand reactive state management.</p>
                    </a>
                    <a href="#/components" class="card-link group">
                        <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Components</h3>
                        <p class="text-xs text-muted-foreground mt-0.5">Build reusable UI components.</p>
                    </a>
                    <a href="#/examples" class="card-link group">
                        <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Examples</h3>
                        <p class="text-xs text-muted-foreground mt-0.5">See complete example applications.</p>
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
fn ComponentsPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-6xl px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <DocPageHeader
                title="Components"
                summary="Components package structure, state, and behavior into reusable boundaries. In Momenta, a component is just a Rust function that returns a Node, so the abstraction stays lightweight and familiar."
                chips={vec!["#[component]", "props", "children"]}
                stats={vec![
                    ("Abstraction", "A function-based boundary around UI and reactive logic"),
                    ("Data flow", "Props come in, local signals stay inside, children compose downward"),
                    ("Goal", "Keep features understandable by splitting them into focused units"),
                ]}
            />

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="introduction" class="font-bold tracking-tight">Introduction</h2>
                <p>
                    "Components in Momenta are functions that return a Node. They can accept props and maintain
                    internal state using signals and other reactive primitives."
                </p>

                <div class="doc-grid my-8">
                    <TheoryCard icon="fas fa-border-all" title="A component is a boundary, not a mini framework">
                        <p>
                            "Use components to define clear ownership over a piece of UI. A good component owns one concern, receives a small prop surface, and exposes a predictable shape to the rest of the app."
                        </p>
                    </TheoryCard>
                    <TheoryCard icon="fas fa-right-left" title="Separate inputs from local behavior">
                        <p>
                            "Props describe what a parent can configure. Signals describe what the component itself needs to track while it runs. Keeping that line clean makes components easier to reuse and test."
                        </p>
                    </TheoryCard>
                </div>

                <div class="theory-panel not-prose my-8">
                    <div class="text-[11px] font-semibold uppercase tracking-[0.18em] text-primary">Heuristic</div>
                    <h2 class="mt-3 text-2xl font-bold tracking-tight">When should something become a component?</h2>
                    <div class="mt-4 grid gap-3 md:grid-cols-3">
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">Reuse</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"The same UI pattern appears in multiple places with different data."</p>
                        </div>
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">Complexity</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"The markup is long enough that it hides the intent of the parent view."</p>
                        </div>
                        <div class="rounded-2xl border border-border/60 bg-background/80 px-4 py-4">
                            <div class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">State ownership</div>
                            <p class="mt-2 text-sm leading-6 text-muted-foreground">"A feature needs local reactive state that should not leak into the rest of the tree."</p>
                        </div>
                    </div>
                </div>

                <h2 id="basic-component" class="font-bold tracking-tight">Basic Component</h2>
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

                <h2 id="components-with-props" class="font-bold tracking-tight">Components with Props</h2>
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

                <h2 id="components-with-state" class="font-bold tracking-tight">Components with State</h2>
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

                <h2 id="component-composition" class="font-bold tracking-tight">Component Composition</h2>
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

                <h2 id="best-practices" class="font-bold tracking-tight">Best Practices</h2>
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
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <header class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">Performance</h1>
                <p class="mt-3 text-lg text-muted-foreground leading-relaxed">
                    "Optimize your Momenta applications for maximum performance and efficiency."
                </p>
            </header>

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="introduction" class="font-bold tracking-tight">Introduction</h2>
                <p>
                    "Momenta is designed for performance from the ground up. Its fine-grained reactivity system
                    ensures that only the parts of your UI that actually need to update will re-render."
                </p>

                <h2 id="signal-optimization" class="font-bold tracking-tight">Signal Optimization</h2>
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

                <h2 id="component-optimization" class="font-bold tracking-tight">Component Optimization</h2>
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

                <h2 id="list-rendering-performance" class="font-bold tracking-tight">List Rendering Performance</h2>
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

                <h2 id="effect-optimization" class="font-bold tracking-tight">Effect Optimization</h2>
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

                <h2 id="memory-management" class="font-bold tracking-tight">Memory Management</h2>
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

                <h2 id="bundle-size-optimization" class="font-bold tracking-tight">Bundle Size Optimization</h2>
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

                <h2 id="best-practices" class="font-bold tracking-tight">Best Practices</h2>
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

                <h2 id="performance-monitoring" class="font-bold tracking-tight">Performance Monitoring</h2>
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

                <div class="mt-16 flex items-center justify-between border-t border-border pt-8">
                    <a href="#/lists" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← List Rendering"
                    </a>
                    <a href="#/deployment" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Deployment →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
fn DeploymentPage() -> Node {
    rsx! {
        <div class="px-6 py-10 sm:px-8 lg:px-10 space-y-8 fade-in">
            <div>
                <h1 class="text-4xl font-bold mb-4">Deployment</h1>
                <p class="text-lg text-muted-foreground mb-8">
                    Deploy your Momenta applications to production.
                </p>
            </div>

            <div>
                <h2 id="build-for-production" class="text-2xl font-bold mb-4">Build for Production</h2>
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
                <h2 id="static-hosting" class="text-2xl font-bold mb-4">Static Hosting</h2>
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
                <h2 id="best-practices" class="text-2xl font-bold mb-4">Best Practices</h2>
                <ul class="list-disc list-inside space-y-2 text-muted-foreground">
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
                <p class="text-lg text-muted-foreground mb-8">
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
                <ul class="list-disc list-inside space-y-2 text-muted-foreground">
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
                <p class="text-lg text-muted-foreground mb-8">
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
                <ul class="list-disc list-inside space-y-2 text-muted-foreground">
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
                <p class="text-lg text-muted-foreground mb-8">
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
                <ul class="list-disc list-inside space-y-2 text-muted-foreground">
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
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <header class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">Conditional Rendering</h1>
                <p class="mt-3 text-lg text-muted-foreground leading-relaxed">
                    "Use when! macro for conditional rendering based on reactive values."
                </p>
            </header>

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="introduction" class="font-bold tracking-tight">Introduction</h2>
                <p>
                    "The when! macro provides a clean way to conditionally render different UI based on
                    reactive values. It's similar to ternary operators but integrates seamlessly with Momenta's reactivity."
                </p>

                <h2 id="basic-usage" class="font-bold tracking-tight">Basic Usage</h2>
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

                <h2 id="complex-conditions" class="font-bold tracking-tight">Complex Conditions</h2>
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

                <h2 id="show-components" class="font-bold tracking-tight">Show Components</h2>
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

                <h2 id="advanced-patterns" class="font-bold tracking-tight">Advanced Patterns</h2>

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

                <h2 id="best-practices" class="font-bold tracking-tight">Best Practices</h2>
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
        <div class="flex items-center justify-center p-6 bg-card">
            <div class="text-center space-y-6">
                <h1 class="text-2xl font-bold">
                    "Momenta Counter"
                </h1>

                <div class="text-5xl font-bold tabular-nums">
                    {count}
                </div>

                <div class="flex gap-3 justify-center">
                    <button
                        class="px-5 py-2 bg-red-500 hover:bg-red-600 text-white font-medium rounded-lg transition-colors"
                        on:click={move |_| count -= 1}
                    >
                        "Decrease"
                    </button>

                    <button
                        class="px-5 py-2 bg-green-500 hover:bg-green-600 text-white font-medium rounded-lg transition-colors"
                        on:click={move |_| count += 1}
                    >
                        "Increase"
                    </button>
                </div>

                <button
                    class="px-4 py-1.5 border border-border rounded-lg text-sm hover:bg-muted transition-colors"
                    on:click={move |_| count.set(0)}
                >
                    "Reset"
                </button>
            </div>
        </div>
    }
}

// Routing Page
#[component]
fn RoutingPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <header class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">Routing</h1>
                <p class="mt-3 text-lg text-muted-foreground leading-relaxed">
                    "Client-side routing for single-page Momenta applications using momenta-router."
                </p>
            </header>

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "The momenta-router crate provides client-side routing with two modes: hash-based and pathname-based.
                    Routes are declared with the routes! macro and support dynamic parameters."
                </p>

                <h2 id="setup">Setup</h2>
                <p>"Add momenta-router to your Cargo.toml:"</p>
                <CodeBlock
                    language="toml"
                    filename="Cargo.toml"
                    highlight=""
                    code={r#"[dependencies]
momenta = "0.2"
momenta-router = "0.2""#}
                />

                <h2 id="router-context">RouterContext</h2>
                <p>"Create a RouterContext to manage navigation state. It provides a reactive signal that updates whenever the route changes."</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r##"use momenta::prelude::*;
use momenta_router::{RouterContext, RouterMode, routes};

#[component]
fn App() -> Node {
    // Hash-based routing (#/path)
    let router = RouterContext::new(RouterMode::Hash);
    let current_path = router.current_path();

    rsx! {
        <div>
            <nav>
                <a href="#/home">"Home"</a>
                <a href="#/about">"About"</a>
            </nav>
            {routes!(router, current_path, {
                "/home" => |_| rsx! { <HomePage /> },
                "/about" => |_| rsx! { <AboutPage /> },
            })}
        </div>
    }
}"##}
                />

                <h2 id="router-modes">Router Modes</h2>

                <h3>"Hash Mode"</h3>
                <p>"Uses the URL hash fragment. Works everywhere without server configuration:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r##"// URLs look like: https://myapp.com/#/about
    let router = RouterContext::new(RouterMode::Hash);"##}
                />

                <h3>"Pathname Mode"</h3>
                <p>"Uses the History API for clean URLs. Requires server-side fallback to index.html:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// URLs look like: https://myapp.com/about
    let router = RouterContext::new(RouterMode::Pathname);"#}
                />

                <Note variant="tip">
                    <p>
                        <strong>"Tip:"</strong> " Use Hash mode for static hosting (GitHub Pages, Netlify) and Pathname mode when you control the server configuration."
                    </p>
                </Note>

                <h2 id="dynamic-routes">Dynamic Route Parameters</h2>
                <p>"Define dynamic segments with :param syntax. Parameters are extracted into a RouteMatch:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta_router::{RouterContext, RouterMode, RouteMatch, routes};

#[component]
fn App() -> Node {
    let router = RouterContext::new(RouterMode::Hash);
    let path = router.current_path();

    rsx! {
        {routes!(router, path, {
            "/" => |_| rsx! { <Home /> },
            "/user/:id" => |m: RouteMatch| {
                let user_id = m.get("id").unwrap_or("unknown");
                rsx! { <UserProfile id={user_id} /> }
            },
            "/posts/:id/comments/:comment_id" => |m: RouteMatch| {
                let post_id = m.get("id").unwrap_or("0");
                let comment_id = m.get("comment_id").unwrap_or("0");
                rsx! {
                    <CommentView post_id={post_id} comment_id={comment_id} />
                }
            },
        })}
    }
}"#}
                />

                <h2 id="programmatic-navigation">Programmatic Navigation</h2>
                <p>"Use router.navigate() to change routes from code, for example after form submissions:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"#[component]
fn LoginForm() -> Node {
    let router = RouterContext::new(RouterMode::Hash);
    let username = create_signal(String::new());

    let handle_login = move |_| {
        // After successful login, navigate to dashboard
        router.navigate("/dashboard");
    };

    rsx! {
        <form>
            <input
                type="text"
                value={username}
                placeholder="Username"
            />
            <button type="button" on:click={handle_login}>
                "Login"
            </button>
        </form>
    }
}"#}
                />

                <h2 id="active-links">Active Link Styling</h2>
                <p>"Use the current_path signal to conditionally style the active navigation link:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r##"let router = RouterContext::new(RouterMode::Hash);
let current_path = router.current_path();

let nav_link = move |path: &'static str, label: &'static str| {
    let is_active = current_path.get() == path;
    let class = if is_active {
        "nav-link text-primary font-bold border-b-2 border-primary"
    } else {
        "nav-link text-muted-foreground hover:text-foreground"
    };

    rsx! {
        <a href={format!("#{path}")} class={class}>
            {label}
        </a>
    }
};

rsx! {
    <nav class="flex gap-4">
        {nav_link("/", "Home")}
        {nav_link("/about", "About")}
        {nav_link("/contact", "Contact")}
    </nav>
}"##}
                />

                <h2 id="route-match">RouteMatch API</h2>
                <p>"The RouteMatch struct provides parameter extraction for dynamic routes:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"pub struct RouteMatch {
    pub params: Vec<(String, String)>,
}

impl RouteMatch {
    // Get a parameter by name
    pub fn get(&self, key: &str) -> Option<&str>;
}

// Example usage in route handler
"/blog/:slug" => |m: RouteMatch| {
    let slug = m.get("slug").unwrap_or("not-found");
    rsx! { <BlogPost slug={slug} /> }
}"#}
                />

                <h2 id="best-practices">Best Practices</h2>
                <ul>
                    <li>"Create the RouterContext once at the top level and pass it down via props"</li>
                    <li>"Use Hash mode for simple static deployments"</li>
                    <li>"Use Pathname mode when you have server-side URL rewriting"</li>
                    <li>"Keep route patterns simple and readable"</li>
                    <li>"Extract route handlers into separate page components"</li>
                    <li>"Use programmatic navigation for redirects and post-action flows"</li>
                </ul>

                <div class="mt-16 flex items-center justify-between border-t border-border pt-8">
                    <a href="#/deployment" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← Deployment"
                    </a>
                    <a href="#/examples" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Examples →"
                    </a>
                </div>
            </section>
        </article>
    }
}

// Examples Page
#[component]
fn ExamplesPage() -> Node {
    rsx! {
        <div class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <header class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">Examples</h1>
                <p class="mt-3 text-lg text-muted-foreground leading-relaxed">
                    "Explore example applications built with Momenta."
                </p>
            </header>

            <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                <a href="#/examples/counter" class="card-link group">
                    <div class="mb-3 flex h-10 w-10 items-center justify-center rounded-lg bg-blue-500/10 text-blue-500">
                        <i class="fas fa-plus-minus text-lg"></i>
                    </div>
                    <h3 class="font-medium mb-1 group-hover:text-primary transition-colors">Counter</h3>
                    <p class="text-sm text-muted-foreground leading-relaxed">
                        "A simple counter demonstrating signals, events, and reactive updates."
                    </p>
                </a>

                <a href="#/examples/todomvc" class="card-link group">
                    <div class="mb-3 flex h-10 w-10 items-center justify-center rounded-lg bg-green-500/10 text-green-500">
                        <i class="fas fa-check-square text-lg"></i>
                    </div>
                    <h3 class="font-medium mb-1 group-hover:text-primary transition-colors">TodoMVC</h3>
                    <p class="text-sm text-muted-foreground leading-relaxed">
                        "Complete TodoMVC implementation with filtering, editing, and persistence."
                    </p>
                </a>

                <a href="#/examples/hackernews" class="card-link group">
                    <div class="mb-3 flex h-10 w-10 items-center justify-center rounded-lg bg-orange-500/10 text-orange-500">
                        <i class="fab fa-hacker-news text-lg"></i>
                    </div>
                    <h3 class="font-medium mb-1 group-hover:text-primary transition-colors">Hacker News</h3>
                    <p class="text-sm text-muted-foreground leading-relaxed">
                        "HN client with async data fetching, pagination, and comments."
                    </p>
                </a>

                <a href="#/examples/realworld" class="card-link group">
                    <div class="mb-3 flex h-10 w-10 items-center justify-center rounded-lg bg-purple-500/10 text-purple-500">
                        <i class="fas fa-globe text-lg"></i>
                    </div>
                    <h3 class="font-medium mb-1 group-hover:text-primary transition-colors">RealWorld</h3>
                    <p class="text-sm text-muted-foreground leading-relaxed">
                        "Full-stack blog platform with auth, articles, comments, and profiles."
                    </p>
                </a>

                <div class="card-link group border-dashed opacity-60">
                    <div class="mb-3 flex h-10 w-10 items-center justify-center rounded-lg bg-muted text-muted-foreground">
                        <i class="fas fa-plus text-lg"></i>
                    </div>
                    <h3 class="font-medium mb-1">More Coming Soon</h3>
                    <p class="text-sm text-muted-foreground leading-relaxed">
                        "Additional examples are being added. Contributions welcome!"
                    </p>
                </div>
            </div>
        </div>
    }
}

fn main() {
    render_root::<App>("#app");
}
