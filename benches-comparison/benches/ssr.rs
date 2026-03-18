//! SSR Rendering Comparison: momenta vs leptos vs dioxus vs yew
//!
//! Benchmarks the time to render equivalent HTML structures to strings
//! across all four frameworks.

use criterion::{Criterion, black_box, criterion_group, criterion_main};

mod momenta_bench {
    use momenta::prelude::*;

    pub fn simple_element() -> String {
        momenta_ssr::render_to_string(|| rsx!(<div>"Hello World"</div>))
    }

    pub fn nested_elements() -> String {
        momenta_ssr::render_to_string(|| {
            rsx!(
                <div>
                    <header>
                        <h1>"Title"</h1>
                        <nav>
                            <a href="/">"Home"</a>
                            <a href="/about">"About"</a>
                        </nav>
                    </header>
                    <main>
                        <p>"Content goes here"</p>
                    </main>
                </div>
            )
        })
    }

    pub fn list_render() -> String {
        momenta_ssr::render_to_string(|| {
            let items: Vec<i32> = (0..100).collect();
            rsx!(
                <ul>
                    {items.iter().map(|i| rsx!(<li>"Item " {*i}</li>))}
                </ul>
            )
        })
    }
}

mod leptos_bench {
    use leptos::prelude::*;

    pub fn simple_element() -> String {
        view! { <div>"Hello World"</div> }.to_html()
    }

    pub fn nested_elements() -> String {
        view! {
            <div>
                <header>
                    <h1>"Title"</h1>
                    <nav>
                        <a href="/">"Home"</a>
                        <a href="/about">"About"</a>
                    </nav>
                </header>
                <main>
                    <p>"Content goes here"</p>
                </main>
            </div>
        }
        .to_html()
    }

    pub fn list_render() -> String {
        let items: Vec<i32> = (0..100).collect();
        let owner = Owner::new();
        owner.with(|| {
            view! {
                <ul>
                    {items
                        .into_iter()
                        .map(|i| view! { <li>{format!("Item {i}")}</li> })
                        .collect::<Vec<_>>()}
                </ul>
            }
            .to_html()
        })
    }
}

mod dioxus_bench {
    use dioxus::prelude::*;

    fn simple_element_app() -> Element {
        rsx! { div { "Hello World" } }
    }

    fn nested_elements_app() -> Element {
        rsx! {
            div {
                header {
                    h1 { "Title" }
                    nav {
                        a { href: "/", "Home" }
                        a { href: "/about", "About" }
                    }
                }
                main {
                    p { "Content goes here" }
                }
            }
        }
    }

    fn list_render_app() -> Element {
        let items: Vec<i32> = (0..100).collect();
        rsx! {
            ul {
                for item in items {
                    li { "Item {item}" }
                }
            }
        }
    }

    pub fn simple_element() -> String {
        let mut dom = VirtualDom::new(simple_element_app);
        dom.rebuild_in_place();
        dioxus::ssr::render(&dom)
    }

    pub fn nested_elements() -> String {
        let mut dom = VirtualDom::new(nested_elements_app);
        dom.rebuild_in_place();
        dioxus::ssr::render(&dom)
    }

    pub fn list_render() -> String {
        let mut dom = VirtualDom::new(list_render_app);
        dom.rebuild_in_place();
        dioxus::ssr::render(&dom)
    }
}

mod yew_bench {
    use yew::ServerRenderer;
    use yew::prelude::*;

    #[function_component]
    fn SimpleElement() -> Html {
        html! { <div>{"Hello World"}</div> }
    }

    #[function_component]
    fn NestedElements() -> Html {
        html! {
            <div>
                <header>
                    <h1>{"Title"}</h1>
                    <nav>
                        <a href="/">{"Home"}</a>
                        <a href="/about">{"About"}</a>
                    </nav>
                </header>
                <main>
                    <p>{"Content goes here"}</p>
                </main>
            </div>
        }
    }

    #[function_component]
    fn ListRender() -> Html {
        let items: Vec<i32> = (0..100).collect();
        html! {
            <ul>
                { for items.iter().map(|i| html! { <li>{format!("Item {i}")}</li> }) }
            </ul>
        }
    }

    pub fn simple_element(rt: &tokio::runtime::Runtime) -> String {
        rt.block_on(async { ServerRenderer::<SimpleElement>::new().render().await })
    }

    pub fn nested_elements(rt: &tokio::runtime::Runtime) -> String {
        rt.block_on(async { ServerRenderer::<NestedElements>::new().render().await })
    }

    pub fn list_render(rt: &tokio::runtime::Runtime) -> String {
        rt.block_on(async { ServerRenderer::<ListRender>::new().render().await })
    }
}

fn bench_ssr_simple(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("ssr_simple_element");
    group.bench_function("momenta", |b| {
        b.iter(|| black_box(momenta_bench::simple_element()))
    });
    group.bench_function("leptos", |b| {
        b.iter(|| black_box(leptos_bench::simple_element()))
    });
    group.bench_function("dioxus", |b| {
        b.iter(|| black_box(dioxus_bench::simple_element()))
    });
    group.bench_function("yew", |b| {
        b.iter(|| black_box(yew_bench::simple_element(&rt)))
    });
    group.finish();
}

fn bench_ssr_nested(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("ssr_nested_elements");
    group.bench_function("momenta", |b| {
        b.iter(|| black_box(momenta_bench::nested_elements()))
    });
    group.bench_function("leptos", |b| {
        b.iter(|| black_box(leptos_bench::nested_elements()))
    });
    group.bench_function("dioxus", |b| {
        b.iter(|| black_box(dioxus_bench::nested_elements()))
    });
    group.bench_function("yew", |b| {
        b.iter(|| black_box(yew_bench::nested_elements(&rt)))
    });
    group.finish();
}

fn bench_ssr_list(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("ssr_list_100_items");
    group.bench_function("momenta", |b| {
        b.iter(|| black_box(momenta_bench::list_render()))
    });
    group.bench_function("leptos", |b| {
        b.iter(|| black_box(leptos_bench::list_render()))
    });
    group.bench_function("dioxus", |b| {
        b.iter(|| black_box(dioxus_bench::list_render()))
    });
    group.bench_function("yew", |b| b.iter(|| black_box(yew_bench::list_render(&rt))));
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3));
    targets = bench_ssr_simple,
        bench_ssr_nested,
        bench_ssr_list
}
criterion_main!(benches);
