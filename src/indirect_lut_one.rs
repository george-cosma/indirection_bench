use crate::common::*;

#[derive(size_of::SizeOf)]
pub struct LUT {
    // module_idx -> offset in func_lut where the bodies of imported functies are
    pub mod_lut: Vec<usize>,
    // module_idx_offset + imported_function_idx -> (module_idx, function_idx)
    pub func_lut: Vec<(usize, usize)>,
}

impl LUT {
    pub fn new(modules: &[Module]) -> Self {
        let mut mod_lut = Vec::new();
        let mut func_lut = Vec::new();

        for module in modules {
            mod_lut.push(func_lut.len());
            for function in &module.funcs {
                if let Function::Imported {
                    module_name: searched_module,
                    function_name: searched_function,
                } = function
                {
                    func_lut.push(manual_lookup(modules, &searched_module, &searched_function));
                }
            }
        }
        LUT { mod_lut, func_lut }
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
        self.func_lut[self.mod_lut[module_idx] + func_idx]
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
