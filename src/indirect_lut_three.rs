use crate::common::*;

#[derive(size_of::SizeOf)]
pub struct LUT {
    pub func_lut: Vec<(usize, usize)>,
    pub max_len: usize,
}

impl LUT {
    pub fn new(modules: &[Module]) -> Self {
        let mut module_lut = Vec::new();

        for module in modules {
            let mut func_lut = Vec::new();
            for function in &module.funcs {
                if let Function::Imported {
                    module_name: searched_module,
                    function_name: searched_function,
                } = function
                {
                    func_lut.push(manual_lookup(modules, &searched_module, &searched_function));
                }
            }
            module_lut.push(func_lut);
        }

        let max_len = module_lut.iter().map(|x| x.len()).max().unwrap_or(0);
        let mut func_lut: Vec<(usize, usize)> = Vec::new();
        for module in &module_lut {
            for (module_idx, function_idx) in module {
                func_lut.push((module_idx.clone(), function_idx.clone()));
            }
            for _ in 0..(max_len - module.len()) {
                func_lut.push((usize::MAX, usize::MAX));
            }
        }

        LUT { func_lut, max_len }
    }

    /// # Arguments
    /// - module_idx: index of the module which imports the searched function
    /// - func_idx: index of the imported function in the importing module's index space
    ///
    /// # Returns
    /// - (module_idx, function_idx)
    ///     - module_idx: index of the module which exports the searched function
    ///    - function_idx: index of the exported function in the exporting module's index space
    pub fn lookup(&self, module_idx: usize, func_idx: usize) -> (usize, usize) {
        self.func_lut[module_idx * self.max_len + func_idx]
    }
}

fn manual_lookup(modules: &[Module], module_name: &str, func_name: &str) -> (usize, usize) {
    for (module_idx, module) in modules.iter().enumerate() {
        if module.name == module_name {
            for (function_idx, function) in module.funcs.iter().enumerate() {
                if let Function::Local { name, .. } = function {
                    if name == func_name {
                        return (module_idx, function_idx);
                    }
                }
            }
        }
    }

    panic!(
        "Function not found, module_name: {}, func_name: {}",
        module_name, func_name
    );
}

#[derive(size_of::SizeOf)]
pub struct IndirectRuntimeLut {
    pub lut: LUT,
    pub modules: Vec<Module>,
}

impl IndirectRuntimeLut {
    pub fn new() -> Self {
        IndirectRuntimeLut {
            lut: LUT::new(&[]),
            modules: Vec::new(),
        }
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }

    pub fn freeze(&mut self) {
        self.lut = LUT::new(&self.modules);
    }

    pub fn run<'a>(&'a self, fuel: u32, module_name: &'a str, function_name: &'a str) {
        // Find start point
        let (mut mod_idx, mut func_idx) = manual_lookup(&self.modules, module_name, &function_name);

        // Loooooooooooooooooop
        for _ in 0..fuel {
            let function = &self.modules[mod_idx].funcs[func_idx];

            match function {
                Function::Local { function_body, .. } => {
                    // Simulate calling another function by index in its own module. This next
                    // function will be an imported function.
                    (mod_idx, func_idx) = self.lut.lookup(mod_idx, *function_body);
                }
                _ => panic!("Cannot run imported functions"),
            }
        }
    }
}
