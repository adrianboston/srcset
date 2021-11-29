//! A few file and path utilities

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Creates a std::path::Path from destination, root, parent, filename, and extension strs
#[inline]
pub fn path_from_strs_dest(dest: &str, root: &str, parent: &str, filename: &str, ext: &str) -> PathBuf {
    let mut pb = PathBuf::new();
    pb.push(dest);
    pb.push(root);
    pb.push(parent);

    let f = filename.to_owned() + "." + ext;
    pb.push(f);
    pb
}

/// Creates a std::path::Path from root, parent, filename, and extension strs
#[inline]
pub fn path_from_strs(root: &str, parent: &str, filename: &str, ext: &str) -> PathBuf {
    let mut pb = PathBuf::new();
    pb.push(root);
    pb.push(parent);

    let f = filename.to_owned() + "." + ext;
    pb.push(f);
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

// Determine whether to use the filename extension or the provided str extension.
#[inline]
pub fn use_fileext<'a>(path: &'a Path, extension: &'a str) -> &'a str
{
    match extension
    {
        "" => path.extension().and_then(OsStr::to_str).unwrap(),
        _ => extension,
    }
}
