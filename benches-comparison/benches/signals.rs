//! Reactive Signals Comparison: momenta vs leptos
//!
//! Benchmarks reactive primitive operations. Only momenta and leptos are
//! compared here because they expose signal APIs that can be exercised
//! outside a full component/renderer context. Dioxus and Yew require a
//! VirtualDom / component runtime for their reactive hooks, so they are
//! compared in the SSR benchmarks instead.

use criterion::{Criterion, criterion_group, criterion_main};

mod momenta_signals {
    use criterion::black_box;
    use momenta::prelude::*;
    use momenta::signals::run_scope_transient;

    pub fn signal_creation() {
        run_scope_transient(
            || {
                let sig = create_signal(black_box(0i32));
                black_box(sig);
                rsx!(<div />)
            },
            |_| {},
        );
    }

    pub fn signal_updates() {
        run_scope_transient(
            || {
                let sig = create_signal(0i32);
                for i in 0..100 {
                    sig.set(black_box(i));
                }
                black_box(sig.get());
                rsx!(<div />)
            },
            |_| {},
        );
    }

    pub fn signal_reads() {
        run_scope_transient(
            || {
                let sig = create_signal(42i32);
                for _ in 0..100 {
                    black_box(sig.get());
                }
                rsx!(<div />)
            },
            |_| {},
        );
    }

    pub fn effect_creation() {
        run_scope_transient(
            || {
                let sig = create_signal(0i32);
                create_effect(move || {
                    let _ = sig.get();
                });
                rsx!(<div />)
            },
            |_| {},
        );
    }
}

mod leptos_signals {
    use criterion::black_box;
    use leptos::prelude::*;

    pub fn signal_creation() {
        let owner = Owner::new();
        owner.with(|| {
            let sig = RwSignal::new(black_box(0i32));
            black_box(sig);
        });
    }

    pub fn signal_updates() {
        let owner = Owner::new();
        owner.with(|| {
            let sig = RwSignal::new(0i32);
            for i in 0..100 {
                sig.set(black_box(i));
            }
            black_box(sig.get());
        });
    }

    pub fn signal_reads() {
        let owner = Owner::new();
        owner.with(|| {
            let sig = RwSignal::new(42i32);
            for _ in 0..100 {
                black_box(sig.get());
            }
        });
    }

    pub fn effect_creation() {
        let owner = Owner::new();
        owner.with(|| {
            let sig = RwSignal::new(0i32);
            Effect::new(move |_| {
                let _ = sig.get();
            });
        });
    }
}

fn bench_signal_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_creation");
    group.bench_function("momenta", |b| b.iter(|| momenta_signals::signal_creation()));
    group.bench_function("leptos", |b| b.iter(|| leptos_signals::signal_creation()));
    group.finish();
}

fn bench_signal_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_updates_100x");
    group.bench_function("momenta", |b| b.iter(|| momenta_signals::signal_updates()));
    group.bench_function("leptos", |b| b.iter(|| leptos_signals::signal_updates()));
    group.finish();
}

fn bench_signal_reads(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_reads_100x");
    group.bench_function("momenta", |b| b.iter(|| momenta_signals::signal_reads()));
    group.bench_function("leptos", |b| b.iter(|| leptos_signals::signal_reads()));
    group.finish();
}

fn bench_effect_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("effect_creation");
    group.bench_function("momenta", |b| b.iter(|| momenta_signals::effect_creation()));
    group.bench_function("leptos", |b| b.iter(|| leptos_signals::effect_creation()));
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3));
    targets = bench_signal_creation,
        bench_signal_updates,
        bench_signal_reads,
        bench_effect_creation
}
criterion_main!(benches);
