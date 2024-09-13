use crate::common::*;
use std::collections::{BTreeMap, HashMap};

pub struct DirectRuntime {
    pub master_module: Module,
    pub function_map: BTreeMap<String, FunctionIndex>,
}

impl DirectRuntime {
    pub fn new(master_module: Module) -> Self {
        let mut function_map = BTreeMap::new();
        for (i, function) in master_module.funcs.iter().enumerate() {
            match function {
                Function::Local { name, .. } => {
                    function_map.insert(name.clone(), i);
                }
                Function::Imported { function_name, .. } => {
                    function_map.insert(function_name.clone(), i);
                }
            }
        }

        DirectRuntime {
            master_module,
            function_map,
        }
    }

    pub fn run<'a>(&'a self, fuel: u32, function_name: &'a str) {
        let mut function_index = self.function_map[function_name];
        for _ in 0..fuel {
            match self.master_module.funcs[function_index] {
                Function::Local { function_body, .. } => {
                    function_index = function_body;
                }
                _ => panic!("Cannot run imported functions"),
            }
        }
    }
}
