use criterion::*;
use indirection_bench::common::*;
use indirection_bench::indirect::*;
use indirection_bench::direct::*;

pub const MODULES: u32 = 10;
pub const FUEL: u32 = 1_000;

fn bench_indirect(c: &mut Criterion) {
    let mut runtime = IndirectRuntime::new();
    for i in 0..black_box(MODULES) {
        runtime.add_module(Module {
            name: format!("module_{}", i),
            funcs: vec![
                Function::Imported {
                    module_name: format!("module_{}", (i + 1) % black_box(MODULES)),
                    function_name: format!("local_function_{}", (i + 1) % black_box(MODULES)),
                },
                Function::Local {
                    name: format!("local_function_{}", i),
                    function_body: black_box(0),
                },
            ],
        });
    }

    c.bench_function("indirect runtime", |b| {
        b.iter(|| {
            runtime.run(
                black_box(FUEL),
                black_box("module_0"),
                black_box("local_function_0"),
            );
        })
    });
}

fn bench_direct(c: &mut Criterion) {
    let mut funcs = vec![];
    for i in 0..black_box(MODULES) {
        funcs.push(Function::Local {
            name: format!("local_function_{}", i),
            function_body: black_box((i + 1) % MODULES) as usize,
        });
    }

    let runtime = DirectRuntime::new(Module {
        name: "module_0".to_string(),
        funcs,
    });

    c.bench_function("direct runtime", |b| {
        b.iter(|| {
            runtime.run(
                black_box(FUEL),
                black_box("local_function_0"),
            );
        })
    });
}

criterion_group!(benches, bench_indirect, bench_direct);
criterion_main!(benches);
