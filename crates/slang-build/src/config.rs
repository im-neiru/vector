use std::path::Path;

pub struct Config<'a> {
    pub input_dir: &'a Path,
    pub output_dir: &'a Path,
    pub optimization: OptimizationLevel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptimizationLevel {
    Default,
    High,
    Maximal,
}
