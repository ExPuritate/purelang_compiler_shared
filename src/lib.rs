use std::collections::HashMap;

pub trait Compiler {
    /// `p` must be an absolute path.
    fn add_file(&mut self, p: &str) -> global::Result<()>;
    fn compile(&self, map: &HashMap<String, String>) -> global::Result<()>;
    fn paths(&self) -> Vec<String>;
}
