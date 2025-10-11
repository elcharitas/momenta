use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_signal_creation(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope_once;

    c.bench_function("signal_creation_current", |b| {
        b.iter(|| {
            run_scope_once(|| {
                let signal = create_signal(black_box(0));
                black_box(signal);
                rsx!(<div />)
            });
        });
    });
}

fn bench_signal_updates(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope_once;

    c.bench_function("signal_updates_current", |b| {
        b.iter(|| {
            run_scope_once(|| {
                let signal = create_signal(0);
                for i in 0..100 {
                    signal.set(black_box(i));
                }
                black_box(signal.get());
                rsx!(<div />)
            });
        });
    });
}

fn bench_signal_reads(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope_once;

    c.bench_function("signal_reads_current", |b| {
        run_scope_once(|| {
            let signal = create_signal(42);
            b.iter(|| {
                for _ in 0..100 {
                    black_box(signal.get());
                }
            });
            rsx!(<div />)
        });
    });
}

fn bench_computed_signals(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope_once;

    c.bench_function("computed_signals_current", |b| {
        b.iter(|| {
            run_scope_once(|| {
                let signal = create_signal(10);
                // let computed = create_computed(move || signal.get() * 2);

                for i in 0..50 {
                    // signal.set(black_box(i));
                    // black_box(computed.get());
                }
                rsx!(<div />)
            });
        });
    });
}

fn bench_effects(c: &mut Criterion) {
    use momenta::prelude::*;
    use momenta::signals::run_scope_once;

    c.bench_function("effects_current", |b| {
        b.iter(|| {
            run_scope_once(|| {
                let signal = create_signal(0);
                let effect_runs = create_signal(0);

                create_effect({
                    let signal = signal.clone();
                    let effect_runs = effect_runs.clone();
                    move || {
                        let _ = signal.get();
                        effect_runs.set(effect_runs.get() + 1);
                    }
                });

                // for i in 0..20 {
                //     signal.set(black_box(i));
                // }
                // black_box(effect_runs.get());
                rsx!(<div />)
            });
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
