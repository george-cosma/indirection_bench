use criterion::*;
use indirection_bench::common::*;
use indirection_bench::direct::*;
use indirection_bench::indirect::*;

pub const MODULES: u32 = 15;
pub const FUEL: u32 = 1_000_000;
pub const JINX_MODULE_SIZE: usize = 300;

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

    // Massive stand-alone module
    runtime.add_module(Module {
        name: "jinx".to_string(),
        funcs: vec![
            Function::Local {
                name: "jinx".to_string(),
                function_body: 0,
            };
            JINX_MODULE_SIZE
        ],
    });

    c.bench_function("indirect runtime", |b| {
        b.iter(|| {
            runtime.run(
                black_box(FUEL),
                black_box("module_0"),
                black_box("local_function_0"),
            );
        })
    });

    println!(
        "Memory usage: {} bytes",
        size_of::size_of_values([&runtime as &dyn size_of::SizeOf]).total_bytes()
    );
}

fn bench_direct(c: &mut Criterion) {
    let mut funcs = vec![];
    for i in 0..black_box(MODULES) {
        funcs.push(Function::Local {
            name: format!("local_function_{}", i),
            function_body: black_box((i + 1) % MODULES) as usize,
        });
    }

    // Massive stand-alone module
    //
    // Note: since the runtime here is created once, we will just add them directly to the `funcs`
    // vector.
    funcs.append(&mut vec![
        Function::Local {
            name: "jinx".to_string(),
            function_body: 0,
        };
        JINX_MODULE_SIZE
    ]);

    let runtime = DirectRuntime::new(Module {
        name: "module_0".to_string(),
        funcs,
    });

    c.bench_function("direct runtime", |b| {
        b.iter(|| {
            runtime.run(black_box(FUEL), black_box("local_function_0"));
        })
    });

    println!(
        "Memory usage: {} bytes",
        size_of::size_of_values([&runtime as &dyn size_of::SizeOf]).total_bytes()
    );
}

fn lut_one(c: &mut Criterion) {
    let mut runtime = indirection_bench::indirect_lut_one::IndirectRuntimeLut::new();
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

    // Massive stand-alone module
    runtime.add_module(Module {
        name: "jinx".to_string(),
        funcs: vec![
            Function::Local {
                name: "jinx".to_string(),
                function_body: 0,
            };
            JINX_MODULE_SIZE
        ],
    });

    runtime.freeze();

    c.bench_function("LUT 1", |b| {
        b.iter(|| {
            runtime.run(
                black_box(FUEL),
                black_box("module_0"),
                black_box("local_function_0"),
            );
        })
    });

    println!(
        "Memory usage: {} bytes",
        size_of::size_of_values([&runtime as &dyn size_of::SizeOf]).total_bytes()
    );
}

fn lut_two(c: &mut Criterion) {
    let mut runtime = indirection_bench::indirect_lut_two::IndirectRuntimeLut::new();
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

    // Massive stand-alone module
    runtime.add_module(Module {
        name: "jinx".to_string(),
        funcs: vec![
            Function::Local {
                name: "jinx".to_string(),
                function_body: 0,
            };
            JINX_MODULE_SIZE
        ],
    });

    runtime.freeze();

    c.bench_function("LUT 2", |b| {
        b.iter(|| {
            runtime.run(
                black_box(FUEL),
                black_box("module_0"),
                black_box("local_function_0"),
            );
        })
    });

    println!(
        "Memory usage: {} bytes",
        size_of::size_of_values([&runtime as &dyn size_of::SizeOf]).total_bytes()
    );
}

fn lut_three(c: &mut Criterion) {
    let mut runtime = indirection_bench::indirect_lut_three::IndirectRuntimeLut::new();
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

    // Massive stand-alone module
    runtime.add_module(Module {
        name: "jinx".to_string(),
        funcs: vec![
            Function::Local {
                name: "jinx".to_string(),
                function_body: 0,
            };
            JINX_MODULE_SIZE
        ],
    });

    runtime.freeze();

    c.bench_function("LUT 3", |b| {
        b.iter(|| {
            runtime.run(
                black_box(FUEL),
                black_box("module_0"),
                black_box("local_function_0"),
            );
        })
    });

    println!(
        "Memory usage: {} bytes",
        size_of::size_of_values([&runtime as &dyn size_of::SizeOf]).total_bytes()
    );
}

criterion_group!(
    benches,
    bench_indirect,
    bench_direct,
    lut_one,
    lut_two,
    lut_three
);
criterion_main!(benches);
