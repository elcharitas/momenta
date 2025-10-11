use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_simple_component(c: &mut Criterion) {
    use momenta::prelude::*;

    c.bench_function("simple_component", |b| {
        b.iter(|| {
            #[component]
            fn SimpleComponent() -> Node {
                rsx! { <div>"Hello World"</div> }
            }

            black_box(SimpleComponent::render(&Default::default()));
        });
    });
}

fn bench_component_with_props(c: &mut Criterion) {
    use momenta::prelude::*;

    c.bench_function("component_with_props", |b| {
        b.iter(|| {
            struct ButtonProps {
                label: &'static str,
                count: i32,
            }

            #[component]
            fn Button(props: &ButtonProps) -> Node {
                rsx! {
                    <button>
                        {props.label} " - " {props.count}
                    </button>
                }
            }

            let props = ButtonProps {
                label: "Click me",
                count: black_box(42),
            };
            black_box(Button::render(&props));
        });
    });
}

fn bench_nested_components(c: &mut Criterion) {
    use momenta::prelude::*;

    c.bench_function("nested_components", |b| {
        b.iter(|| {
            #[component]
            fn Child() -> Node {
                rsx! { <span>"Child"</span> }
            }

            #[component]
            fn Parent() -> Node {
                rsx! {
                    <div>
                        {Child::render(&Default::default())}
                        {Child::render(&Default::default())}
                        {Child::render(&Default::default())}
                    </div>
                }
            }

            black_box(Parent::render(&Default::default()));
        });
    });
}

fn bench_component_with_state(c: &mut Criterion) {
    use momenta::prelude::*;

    c.bench_function("component_with_state", |b| {
        b.iter(|| {
            #[component]
            fn Counter() -> Node {
                let count = create_signal(black_box(0));

                rsx! {
                    <div>
                        <h1>"Count: " {count}</h1>
                        <button>"Increment"</button>
                    </div>
                }
            }

            black_box(momenta::signals::run_scope(
                || Counter::render(&Default::default()),
                |_| {},
            ));
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3));
    targets = bench_simple_component,
        bench_component_with_props,
        bench_nested_components,
        bench_component_with_state
}
criterion_main!(benches);
