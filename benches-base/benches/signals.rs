use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_signal_creation(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope;

    c.bench_function("signal_creation", |b| {
        b.iter(|| {
            run_scope(
                || {
                    let signal = create_signal(black_box(0));
                    black_box(signal);
                    rsx!(<div />)
                },
                |_| {},
            );
        });
    });
}

fn bench_signal_updates(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope;

    c.bench_function("signal_updates", |b| {
        b.iter(|| {
            run_scope(
                || {
                    let signal = create_signal(0);
                    for i in 0..100 {
                        signal.set(black_box(i));
                    }
                    black_box(signal.get());
                    rsx!(<div />)
                },
                |_| {},
            );
        });
    });
}

fn bench_signal_reads(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope;

    c.bench_function("signal_reads", |b| {
        b.iter(|| {
            run_scope(
                || {
                    let signal = create_signal(42);
                    for _ in 0..100 {
                        black_box(signal.get());
                    }
                    rsx!(<div />)
                },
                |_| {},
            );
        });
    });
}

fn bench_computed_signals(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope;

    c.bench_function("computed_signals", |b| {
        b.iter(|| {
            run_scope(
                || {
                    let signal = create_signal(10);
                    // let computed = create_computed(move || signal.get() * 2);

                    // create_effect(move || {
                    //     for i in 0..50 {
                    //         signal.set(black_box(i));
                    //     }
                    // });

                    // black_box(computed.get());
                    rsx!(<div />)
                },
                |_| {},
            );
        });
    });
}

fn bench_effects(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope;

    c.bench_function("effects", |b| {
        b.iter(|| {
            run_scope(
                || {
                    let signal = create_signal(0);
                    let effect_runs = create_signal(0);

                    create_effect(move || {
                        signal.get();
                        // effect_runs.set(effect_runs.get() + 1);
                    });

                    // black_box(effect_runs.get());
                    rsx!(<div />)
                },
                |_| {},
            );
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3));
    targets = bench_signal_creation,
        bench_signal_updates,
        bench_signal_reads,
        bench_computed_signals,
        bench_effects
}
criterion_main!(benches);
