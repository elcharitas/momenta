#![allow(unused_imports)]

use crate::components::*;
use alloc::{format, vec};
use momenta::prelude::*;

#[component]
pub fn RoutingPage() -> Node {
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
    // Hash-based routing (/path)
    let router = RouterContext::new(RouterMode::Hash);
    let current_path = router.current_path();

    rsx! {
        <div>
            <nav>
                <a href="/home">"Home"</a>
                <a href="/about">"About"</a>
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
                    code={r##"// URLs look like: https://myapp.com//about
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
                    <a href="/deployment" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "← Deployment"
                    </a>
                    <a href="/examples" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Examples →"
                    </a>
                </div>
            </section>
        </article>
    }
}

// Examples Page
#[component]
pub fn ExamplesPage() -> Node {
    rsx! {
        <div class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <header class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">Examples</h1>
                <p class="mt-3 text-lg text-muted-foreground leading-relaxed">
                    "Explore example applications built with Momenta."
                </p>
            </header>

            <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                <a href="/examples/counter" class="card-link group">
                    <div class="mb-3 flex h-10 w-10 items-center justify-center rounded-lg bg-blue-500/10 text-blue-500">
                        <i class="fas fa-plus-minus text-lg"></i>
                    </div>
                    <h3 class="font-medium mb-1 group-hover:text-primary transition-colors">Counter</h3>
                    <p class="text-sm text-muted-foreground leading-relaxed">
                        "A simple counter demonstrating signals, events, and reactive updates."
                    </p>
                </a>

                <a href="/examples/todomvc" class="card-link group">
                    <div class="mb-3 flex h-10 w-10 items-center justify-center rounded-lg bg-green-500/10 text-green-500">
                        <i class="fas fa-check-square text-lg"></i>
                    </div>
                    <h3 class="font-medium mb-1 group-hover:text-primary transition-colors">TodoMVC</h3>
                    <p class="text-sm text-muted-foreground leading-relaxed">
                        "Complete TodoMVC implementation with filtering, editing, and persistence."
                    </p>
                </a>

                <a href="/examples/hackernews" class="card-link group">
                    <div class="mb-3 flex h-10 w-10 items-center justify-center rounded-lg bg-orange-500/10 text-orange-500">
                        <i class="fab fa-hacker-news text-lg"></i>
                    </div>
                    <h3 class="font-medium mb-1 group-hover:text-primary transition-colors">Hacker News</h3>
                    <p class="text-sm text-muted-foreground leading-relaxed">
                        "HN client with async data fetching, pagination, and comments."
                    </p>
                </a>

                <a href="/examples/realworld" class="card-link group">
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
