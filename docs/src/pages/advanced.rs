#![allow(unused_imports)]

use crate::components::*;
use alloc::{format, vec};
use momenta::prelude::*;

#[component]
pub fn ComponentsPage() -> Node {
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
pub fn PerformancePage() -> Node {
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
version = "0.2"
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
                    <a href="/lists" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← List Rendering"
                    </a>
                    <a href="/ssr" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "SSR & Hydration →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
pub fn SsrPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 xl:pr-8 2xl:pr-10 fade-in">
            <DocPageHeader
                title="SSR and Hydration"
                summary="Server rendering in Momenta stays intentionally small: render HTML on the server, stream when transport benefits from chunks, and hydrate only when the browser needs to resume from server markup."
                chips={vec!["momenta-ssr", "streaming", "hydration"]}
                stats={vec![
                    ("Buffered output", "render_to_string for complete HTML responses"),
                    ("Streamed output", "render_to_chunks for chunked transports"),
                    ("Resume support", "render_to_hydration_string plus hydrate_root on the client"),
                ]}
            />

            <section class="prose prose-neutral dark:prose-invert max-w-none prose-headings:tracking-tight prose-p:leading-relaxed prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                <h2 id="overview">Overview</h2>
                <p>
                    Momenta keeps SSR in the <code>momenta-ssr</code> crate so the core UI model stays focused. The server-facing surface is just a few concepts:
                </p>
                <ul>
                    <li><code>render_to_string</code> for buffered HTML</li>
                    <li><code>render_to_chunks</code> for chunked or streamed HTML</li>
                    <li><code>render_to_hydration_string</code> when the browser should resume from server markup</li>
                    <li>thin response adapters for Axum, Actix, and Hyper</li>
                </ul>

                <Note variant="info">
                    <p>
                        <strong>Rule of thumb:</strong> use plain SSR when you only need HTML output. Add hydration only when the browser must attach event handlers and continue from the server-rendered DOM.
                    </p>
                </Note>

                <h2 id="install-server-crate">Install the Server Crate</h2>
                <CodeBlock
                    language="toml"
                    filename="Cargo.toml"
                    highlight=""
                    code={r#"[dependencies]
momenta = { version = "0.2", default-features = false }
momenta-ssr = "0.2"

# Enable only the adapter you need
momenta-ssr = { version = "0.2", features = ["axum"] }"#}
                />

                <h2 id="buffered-ssr">Buffered SSR</h2>
                <p>
                    This is the simplest path. Render a component or closure to a complete HTML string and return it from your server handler.
                </p>
                <CodeBlock
                    language="rust"
                    filename="src/server.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;
use momenta_ssr::render_to_string;

fn page() -> String {
    render_to_string(|| {
        rsx! {
            <main>
                <h1>"Hello from Momenta SSR"</h1>
                <p>"This response was rendered on the server."</p>
            </main>
        }
    })
}"#}
                />

                <h2 id="streaming-html">Streaming HTML</h2>
                <p>
                    If your framework prefers a byte stream, render the same UI into chunks and hand those chunks to the transport layer.
                </p>
                <CodeBlock
                    language="rust"
                    filename="src/server.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;
use momenta_ssr::{render_to_chunks, RenderOptions};

fn stream_chunks() -> Vec<String> {
    render_to_chunks(
        || {
            rsx! {
                <main>
                    <h1>"Streaming Momenta"</h1>
                    <p>"Each chunk can be forwarded by Axum, Actix, or Hyper."</p>
                </main>
            }
        },
        RenderOptions { chunk_size: 1024 },
    )
}"#}
                />

                <h2 id="hydratable-ssr">Hydratable SSR</h2>
                <p>
                    Hydration adds stable <code>data-momenta-*</code> markers to the HTML and can embed request-scoped JSON for the client to read during startup.
                </p>
                <CodeBlock
                    language="rust"
                    filename="src/server.rs"
                    highlight=""
                    code={r##"use momenta::prelude::*;
use momenta_ssr::{render_to_hydration_string, HydrationOptions};

fn hydratable_page() -> String {
    render_to_hydration_string(
        || {
            rsx! {
                <div id="app-shell">
                    <h1>"Hello"</h1>
                    <p>"Rendered on the server, resumed in the browser."</p>
                </div>
            }
        },
        HydrationOptions {
            state_json: Some(r#"{"user":"jon","theme":"light"}"#.into()),
            ..HydrationOptions::default()
        },
    )
}"##}
                />

                <h2 id="client-resume-example">Client Resume Example</h2>
                <p>
                    On the client, call <code>hydrate_root</code> instead of <code>render_root</code>. This reuses the server DOM instead of replacing it.
                </p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r##"use momenta::prelude::*;

#[component]
fn App() -> Node {
    rsx! {
        <div id="app-shell">
            <h1>"Hello"</h1>
            <p>"Rendered on the server, resumed in the browser."</p>
        </div>
    }
}

fn main() {
    let request_data = read_default_hydration_data();
    let _ = request_data;

    hydrate_root::<App>("#app");
}"##}
                />

                <h2 id="framework-adapters">Framework Adapters</h2>
                <p>
                    The adapter helpers stay thin so each framework keeps its own response types:
                </p>
                <ul>
                    <li><code>axum_html</code> and <code>axum_stream</code></li>
                    <li><code>actix_html</code> and <code>actix_stream</code></li>
                    <li><code>hyper_html</code> and <code>hyper_stream</code></li>
                </ul>
                <CodeBlock
                    language="rust"
                    filename="src/axum.rs"
                    highlight=""
                    code={r#"use axum::{routing::get, Router};
use momenta::prelude::*;
use momenta_ssr::axum_html;

async fn home() -> axum::response::Html<String> {
    axum_html(|| rsx!(<main><h1>"Hello from Axum"</h1></main>))
}

fn app() -> Router {
    Router::new().route("/", get(home))
}"#}
                />

                <Note variant="tip">
                    <p>
                        <strong>Use the smallest mode that fits:</strong> buffered HTML is simplest to reason about, chunked rendering helps transport large responses, and hydration should be added only when the client must continue from server-rendered markup.
                    </p>
                </Note>

                <div class="mt-16 flex items-center justify-between border-t border-border pt-8">
                    <a href="/performance" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← Performance"
                    </a>
                    <a href="/deployment" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Deployment →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
pub fn DeploymentPage() -> Node {
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
pub fn TodoMVCPage() -> Node {
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
pub fn HackerNewsPage() -> Node {
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
pub fn RealWorldPage() -> Node {
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
pub fn ShowPage() -> Node {
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
