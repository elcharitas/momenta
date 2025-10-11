use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_simple_element_to_string(c: &mut Criterion) {
    use momenta::prelude::*;

    c.bench_function("simple_element_to_string", |b| {
        b.iter(|| {
            let node = rsx! { <div>"Hello World"</div> };
            black_box(node.to_string());
        });
    });
}

fn bench_nested_elements_to_string(c: &mut Criterion) {
    use momenta::prelude::*;

    c.bench_function("nested_elements_to_string", |b| {
        b.iter(|| {
            let node = rsx! {
                <div class="container">
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
            };
            black_box(node.to_string());
        });
    });
}

fn bench_list_rendering_small(c: &mut Criterion) {
    use momenta::prelude::*;

    c.bench_function("list_rendering_small", |b| {
        b.iter(|| {
            let items = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];
            let node = rsx! {
                <ul>
                    {items.iter().map(|item| rsx!(<li>{item}</li>))}
                </ul>
            };
            black_box(node.to_string());
        });
    });
}

fn bench_list_rendering_large(c: &mut Criterion) {
    use momenta::prelude::*;

    c.bench_function("list_rendering_large", |b| {
        b.iter(|| {
            let items: Vec<i32> = (0..100).collect();
            let node = rsx! {
                <ul>
                    {items.iter().map(|item| rsx!(<li>"Item " {*item}</li>))}
                </ul>
            };
            black_box(node.to_string());
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3));
    targets = bench_simple_element_to_string,
        bench_nested_elements_to_string,
        bench_list_rendering_small,
        bench_list_rendering_large
}
criterion_main!(benches);
