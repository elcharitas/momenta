# momenta-ssr

Server-side rendering utilities for Momenta.

`momenta-ssr` keeps the server API intentionally small:
- `render_to_string` for buffered HTML
- `render_to_chunks` for streamed or chunked HTML
- `render_to_hydration_string` for hydratable output
- optional Axum, Actix, and Hyper adapters

## Installation

```toml
[dependencies]
momenta = { version = "0.2", default-features = false }
momenta-ssr = "0.2"
```

Enable framework adapters only when needed:

```toml
[dependencies]
momenta-ssr = { version = "0.2", features = ["axum"] }
```

Available adapter features:
- `axum`
- `actix`
- `hyper`

## Buffered Rendering

```rust
use momenta::prelude::*;
use momenta_ssr::render_to_string;

let html = render_to_string(|| {
    rsx!(<main><h1>"Hello from Momenta SSR"</h1></main>)
});
```

## Chunked Rendering

```rust
use momenta::prelude::*;
use momenta_ssr::render_to_chunks;

let chunks = render_to_chunks(
    || rsx!(<main><h1>"Hello from Momenta SSR"</h1></main>),
    1024,
);
```

Use the returned chunks directly or wrap them with the adapter helpers for your server framework.

## Hydration

Hydratable rendering emits stable `data-momenta-hid` markers so the browser can attach to existing DOM instead of replacing it. You can also embed request-scoped JSON for the client to read during startup.

```rust
use momenta::prelude::*;
use momenta_ssr::{render_to_hydration_string, HydrationOptions};

let html = render_to_hydration_string(
    || rsx!(<div id="app-shell"><h1>"Hello"</h1></div>),
    HydrationOptions {
        state_json: Some(r#"{"user":"jon"}"#.into()),
        ..HydrationOptions::default()
    },
);
```

On the client:

```rust
use momenta::prelude::*;

#[component]
fn App() -> Node {
    rsx!(<div id="app-shell"><h1>"Hello"</h1></div>)
}

fn main() {
    let request_data = read_default_hydration_data();
    let _ = request_data;
    hydrate_root::<App>("#app");
}
```

If you do not need resume support, use `render_to_string` on the server and `render_root` on the client instead.

## Adapter Helpers

`momenta-ssr` provides thin helpers so each framework keeps its native response types:
- Axum: `axum_html`, `axum_stream`
- Actix: `actix_html`, `actix_stream`
- Hyper: `hyper_html`, `hyper_stream`

The rendering model stays the same regardless of the transport:
1. Build a `Node`
2. Render it to a string, chunks, or hydratable HTML
3. Return it with the framework adapter that matches your server
