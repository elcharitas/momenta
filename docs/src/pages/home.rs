#![allow(unused_imports)]

use crate::components::*;
use alloc::{format, vec};
use momenta::prelude::*;

#[component]
pub fn HomePage(_props: &NavigationProps) -> Node {
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
                    <a href={docs_href("/getting-started")} class="inline-flex items-center gap-2 rounded-lg bg-primary px-5 py-2.5 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
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
                    <a href={docs_href("/getting-started")} class="card-link group">
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
                    <a href={docs_href("/signals")} class="card-link group">
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
                    <a href={docs_href("/components")} class="card-link group">
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
                    <a href={docs_href("/examples")} class="card-link group">
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
pub fn CounterExample() -> Node {
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
                                <a href={docs_href("/getting-started")} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Getting Started</a>
                                <a href={docs_href("/signals")} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Signals</a>
                                <a href={docs_href("/components")} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Components</a>
                                <a href={docs_href("/rsx")} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">RSX Syntax</a>
                            </div>
                        </div>
                        <div>
                            <h4 class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-3">Examples</h4>
                            <div class="space-y-2">
                                <a href={docs_href("/examples/counter")} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Counter</a>
                                <a href={docs_href("/examples/todomvc")} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">TodoMVC</a>
                                <a href={docs_href("/examples/hackernews")} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">Hacker News</a>
                                <a href={docs_href("/examples/realworld")} class="block text-sm text-muted-foreground hover:text-foreground transition-colors">RealWorld</a>
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
pub fn CounterExample() -> Node {
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
