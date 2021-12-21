
//! A few utilities for creating paths and directories. 

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Creates a std::path::Path from an array of strings
#[inline]
pub fn path_from_array(array: &[&str]) -> PathBuf {
    let mut pb = PathBuf::new();
    for s in array {
        pb.push(s);
    }
    pb
}

/// Creates a full directory for the provided path, but drops the filename
#[inline]
pub fn mk_dir(p: &Path) {
    match std::fs::create_dir_all(&p.parent().unwrap() ) {
        Err(_) => (),
        _ => (),
    }
}

/// Determine whether to use the filename extension or the provided str extension.
#[inline]
pub fn use_fileext<'a>(path: &'a Path, ext: &'a str) -> &'a str {
    match ext
    {
        "" => path.extension().and_then(OsStr::to_str).unwrap(),
        _ => ext,
    }
}
