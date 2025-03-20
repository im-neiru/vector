use std::path::Path;

pub struct Config<'a> {
    pub input_dir: &'a Path,
    pub exclude_dirs: &'a [&'a Path],
    pub output_dir: &'a Path,
}
