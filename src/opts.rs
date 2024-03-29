
//! The options passed in many functions.

use std::path::PathBuf;

#[derive(Debug)]
pub struct Opts {
    pub inpath:  PathBuf,
    pub outpath: PathBuf,
    pub prefix: String,
    pub is_file: bool,
    pub extension: String,
    pub is_recurse: bool,
    pub is_test: bool,
    pub is_jobs: bool,
    pub is_nested: bool,
    pub is_dir: bool,
    pub min_size: u64,
    pub is_verbose: bool,
    pub is_quiet: bool,
    pub sizes: Vec<u32>,
    pub quality: u8,
    pub sigma: f32,
    pub thresh: i32,
    pub is_tagfile: bool,
    pub use_largest: bool

}




#[derive(Debug)]
pub struct Metrics {
    pub count: u32,
    pub resized: u32,
    pub traversed: u32,
    pub skipped: u32
}
