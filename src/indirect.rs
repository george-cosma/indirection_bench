use crate::common::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, HashMap};

#[derive(size_of::SizeOf)]
pub struct IndirectRuntime {
    // module_name -> (module, function_name -> function_index)
    pub module_linker: BTreeMap<String, (Module, BTreeMap<String, FunctionIndex>)>,
}

impl IndirectRuntime {
    pub fn new() -> Self {
        IndirectRuntime {
            module_linker: BTreeMap::new(),
        }
    }

    pub fn add_module(&mut self, module: Module) {
        let mut map = BTreeMap::new();
        for (i, function) in module.funcs.iter().enumerate() {
            match function {
                Function::Local { name, .. } => {
                    map.insert(name.clone(), i);
                }
                Function::Imported { function_name, .. } => {
                    map.insert(function_name.clone(), i);
                }
            }
        }
        self.module_linker
            .insert(module.name.clone(), (module, map));
    }

    pub fn run<'a>(&'a self, fuel: u32, mut module_name: &'a str, mut function_name: &'a str) {
        for _ in 0..fuel {
            let (module, map) = &self.module_linker[module_name];
            let function_index = map[function_name];
            let function = &module.funcs[function_index];

            match function {
                Function::Local { function_body, .. } => {
                    // Simulate calling another function by index in its own module. This next
                    // function will be an imported function.
                    match &module.funcs[*function_body] {
                        Function::Imported {
                            module_name: module,
                            function_name: func,
                        } => {
                            module_name = &module;
                            function_name = &func;
                        }
                        _ => panic!("Expected imported function"),
                    }
                }
                _ => panic!("Cannot run imported functions"),
            }
        }
    }
}
