pub type FunctionIndex = usize;

#[derive(Debug, Clone, size_of::SizeOf)]
pub enum Function {
    Local {
        name: String,
        // This would be the span, but in our case we want to simulate it calling really fast to
        // another function by index.
        function_body: FunctionIndex,
    },
    Imported {
        module_name: String,
        function_name: String,
    },
}

#[derive(Debug, size_of::SizeOf)]
pub struct Module {
    pub name: String,
    pub funcs: Vec<Function>,
}
