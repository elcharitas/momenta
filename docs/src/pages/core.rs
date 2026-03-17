#![allow(unused_imports)]

use crate::components::*;
use alloc::{format, vec};
use momenta::prelude::*;

#[component]
pub fn RsxPage() -> Node {
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
                    <a href="/philosophy" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← Philosophy"
                    </a>
                    <a href="/signals" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
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
                    <a href="/computed-signals" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← Computed Signals"
                    </a>
                    <a href="/resources" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Resources →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
pub fn SignalsPage() -> Node {
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
                <p>
                    <code>"Signal<bool>"</code>
                    " has convenience methods for common operations:"
                </p>
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
                <p>
                    <code>"Signal<Vec<T>>"</code>
                    " provides familiar collection methods that automatically trigger updates:"
                </p>
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
                        <strong>"Built-in support:"</strong>
                        " SignalValue is already implemented for all numeric types, bool, char, String, &'static str, "
                        <code>"Vec<T>"</code>
                        ", and "
                        <code>"Option<T>"</code>
                        "."
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
                    <a href="/rsx" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← rsx!"
                    </a>
                    <a href="/computed-signals" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Computed Signals →"
                    </a>
                </div>
            </section>
        </article>
    }
}

// Add more page implementations...
#[component]
pub fn GettingStartedPage() -> Node {
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
                    <a href="/rsx" class="card-link group">
                        <h3 class="text-sm font-medium group-hover:text-primary transition-colors">RSX Syntax</h3>
                        <p class="text-xs text-muted-foreground mt-0.5">Learn the JSX-like template syntax.</p>
                    </a>
                    <a href="/signals" class="card-link group">
                        <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Signals</h3>
                        <p class="text-xs text-muted-foreground mt-0.5">Understand reactive state management.</p>
                    </a>
                    <a href="/components" class="card-link group">
                        <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Components</h3>
                        <p class="text-xs text-muted-foreground mt-0.5">Build reusable UI components.</p>
                    </a>
                    <a href="/examples" class="card-link group">
                        <h3 class="text-sm font-medium group-hover:text-primary transition-colors">Examples</h3>
                        <p class="text-xs text-muted-foreground mt-0.5">See complete example applications.</p>
                    </a>
                </div>
            </section>
        </article>
    }
}
