#![allow(unused_imports)]

use crate::components::*;
use alloc::{format, vec};
use momenta::prelude::*;

#[component]
pub fn ComputedSignalsPage() -> Node {
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
pub fn ClassesPage() -> Node {
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
pub fn ForPage() -> Node {
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
                    <a href={docs_href("/when")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← Conditional Rendering"
                    </a>
                    <a href={docs_href("/performance")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Performance →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
pub fn ResourcesPage() -> Node {
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
                        <strong>"Resource model:"</strong>
                        " A resource tracks progress with ResourceStatus and stores the latest resolved value as "
                        <code>"Option<T>"</code>
                        "."
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
pub fn PhilosophyPage() -> Node {
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
