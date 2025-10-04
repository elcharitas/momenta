# Momenta

**Simple and performant reactivity for building user interfaces**

A fine-grained reactive framework for Rust that makes it simple to build high-performance, reactive user interfaces using Rust's type system and ownership model.

[![Crates.io](https://img.shields.io/crates/v/momenta.svg)](https://crates.io/crates/momenta)
[![Documentation](https://docs.rs/momenta/badge.svg)](https://docs.rs/momenta)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/elcharitas/momenta/badge.svg?branch=main)](https://coveralls.io/github/elcharitas/momenta?branch=main)

```rust
use momenta::prelude::*;

#[component]
fn Counter() -> Node {
    let count = create_signal(0);
    
    rsx! {
        <div class="counter">
            <h1>"Count: " {count}</h1>
            <button on:click={move |_| count += 1}>"Increment"</button>
        </div>
    }
}
```

## Features

### Element-Level Reactivity
Fine-grained reactivity that automatically tracks dependencies and updates only what has changed.

### Type-Safe Components
Leverage Rust's type system for compile-time guarantees and better developer experience.

### Lightweight & Fast
Small runtime with minimal overhead. Your apps stay fast and bundle sizes stay small.

### Familiar API
Inspired by React with a Rust-first approach to reactive programming.

### SSR Ready
Server-side rendering support out of the box with a simple API for better performance and SEO.

### Composable Primitives
Build complex UIs from simple, reusable reactive primitives:
- **Signals** - Fine-grained reactive state
- **Computed Signals** - Derived values with automatic memoization
- **Effects** - Side effect management
- **Resources** - Async data loading with built-in loading/error states

## Quick Start

Add Momenta to your `Cargo.toml`:

```toml
[dependencies]
momenta = "0.2"
```

Create your first component:

```rust
use momenta::prelude::*;

#[component]
fn App() -> Node {
    let count = create_signal(0);
    
    rsx! {
        <div>
            <h1>"Counter: " {count}</h1>
            <button on:click={move |_| count += 1}>
                "Increment"
            </button>
            <button on:click={move |_| count -= 1}>
                "Decrement"
            </button>
        </div>
    }
}

fn main() {
    momenta::dom::render_root::<App>("#app");
}
```

## Why Momenta?

I started this project while attempting to transit my [portfolio](https://elcharitas.wtf) from Next.js to Rust. I tried using dioxus, yew, and hypertext, but I found them to be too complex and verbose for my needs. I wanted a simple and intuitive way to write HTML-like templates in Rust, while still leveraging the full power of Rust's type system.

Momenta aims to provide:
- **Simplicity** - Easy to learn, easy to use
- **Performance** - Fine-grained reactivity means minimal updates
- **Type Safety** - Leverage Rust's type system for compile-time guarantees
- **Familiarity** - React-like API that's easy to pick up

## Core Concepts

### Signals - Reactive State
```rust
let count = create_signal(0);
count.set(5);           // Set value
let value = count.get(); // Get value
count += 1;             // Update with operators
```

### Computed Signals - Derived Values
```rust
let count = create_signal(0);
let doubled = create_computed(move || count.get() * 2);
// or use the derive method
let tripled = count.derive(|&n| n * 3);
```

### Effects - Side Effects
```rust
create_effect(|| {
    console::log!("Count changed to: {}", count.get());
});
```

### Resources - Async Data
```rust
let user = create_resource(|| async {
    fetch_user_data().await
});

rsx! {
    <div>
        {when!(user.loading() => <p>"Loading..."</p>
        else when!(user.error().is_some() => <p>"Error loading user"</p>)
        else <p>"User: " {user.get().unwrap()}</p>)}
    </div>
}
```

### Components - Reusable UI
```rust
pub struct ButtonProps {
    pub label: &'static str,
    pub on_click: Box<dyn Fn()>,
}

#[component]
fn Button(props: &ButtonProps) -> Node {
    rsx! {
        <button on:click={move |_| (props.on_click)()}>
            {props.label}
        </button>
    }
}
```

### Control Flow - Conditionals & Lists
```rust
// Conditional rendering
{when!(show => <p>"Visible"</p> else <p>"Hidden"</p>)}

// List rendering
let items = create_signal(vec!["Apple", "Banana", "Cherry"]);
rsx! {
    <ul>
        {items.map(|item| rsx!(<li>{item}</li>))}
    </ul>
}
```

<!-- ## Examples

Check out these example applications:

- **[Counter](examples/counter)** - Simple counter demonstrating signals
- **[TodoMVC](examples/todomvc)** - Classic TodoMVC implementation
- **[Hacker News](examples/hackernews)** - Hacker News clone with async data loading
- **[RealWorld](examples/realworld)** - Full-stack RealWorld example -->

## Documentation

- **[Getting Started Guide](https://elcharitas.github.io/momenta)** - Learn the basics
- **[API Documentation](https://docs.rs/momenta)** - Complete API reference

## Feature Flags

Enable optional features in your `Cargo.toml`:

```toml
[dependencies]
momenta = { version = "0.2", features = ["full-reactivity"] }
```

Available features:
- `dom` - All HTML elements with DOM rendering (default)
- `wasm` - WebAssembly support for browser rendering (default)
- `computed` - Computed signals support
- `memoization` - Memoization utilities
- `full-reactivity` - All reactive features (includes computed + memoization, default)

For server-side rendering without DOM, use only `momenta-core`:
```toml
[dependencies]
momenta-core = "0.2"
```

## Community & Support

- **[GitHub Discussions](https://github.com/elcharitas/momenta/discussions)** - Ask questions, share ideas
- **[Issues](https://github.com/elcharitas/momenta/issues)** - Report bugs, request features
- **[Contributing Guide](CONTRIBUTING.md)** - Learn how to contribute

## Contributing

We welcome contributions! Here's how you can help:

1. **Report bugs** - Open an issue with a minimal reproduction
2. **Suggest features** - Start a discussion about your idea
3. **Improve docs** - Help us make the docs better
4. **Submit PRs** - Fix bugs or implement features

Please read our [Contributing Guide](CONTRIBUTING.md) before submitting PRs.

## Comparison with Other Frameworks

| Feature | Momenta | Yew | Dioxus |
|---------|---------|-----|--------|
| Fine-grained reactivity | Yes | No | Yes |
| JSX-like syntax | Yes | Yes | Yes |
| SSR Support | Yes | Yes | Yes |
| Learning curve | Low | Medium | Medium |
| Bundle size | Small | Large | Medium |

## License

MIT License - see [LICENSE](LICENSE) for details
