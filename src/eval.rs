#[derive(Debug)]
pub enum Ast {
    /// Atomic number (integer or floating-point)
    Number { whole: String, fraction: String },
    /// Atomic symbol
    Symbol(String),
    /// List of sub-lists or atomics
    List(Vec<(Ast, /*evaluate now*/ bool)>),
}
