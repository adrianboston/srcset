
//! Walk a directory tree, hunting for jpg, png and tiff and image extensions.

use std::path::Path;
use std::ffi::OsStr;
use anyhow::Result;

use crate::opts::{Opts, Metrics};
use crate::img::process_image;

/// Walk or traverse a provided directory. Calls recursively if a directory is found within
/// the provided path, and the options specify todoso.
pub fn walk_path(dir: &Path,  opts: &Opts, m: &mut Metrics) -> Result<()>
{
    if dir.is_dir() {
         // An error here (permission denied) will bail the walk. Dont bail the walk. Instead continue back to the parent
        let rd = match std::fs::read_dir(dir) {
            Ok(t) => t,
            Err(e) => { if !opts.is_quiet{eprintln!("WARNING: Processing error {:?}", e);} return Ok(())},
        };
        
        for entry in rd {

            m.traversed = m.traversed + 1;

            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => { if !opts.is_quiet{eprintln!("WARNING: Processing error {:?}", e)}; continue;},
            };
            let path = entry.path();
            if opts.is_recurse && path.is_dir() {
                walk_path(&path, &opts, m)?;
            } else {
                digest_path(&path, &opts, m)?;
            }
        }
    }
    Ok(())
}

/// Digest or consume a path. Check extension for image type (jpg, png, tif or others specified). In addition,
/// Skips any filename matching `^\\d{3}w$|^\\d{4}w$|^legacy$`
/// If matching the above concerns, then process the iamge.
/// Moves on without error if there is no match.
pub fn digest_path(path: &Path, opts: &Opts, m: &mut Metrics) -> Result<()>
{
    // match any filename with 3 or 4 digits ending in a w; and `legacy`
    lazy_static::lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new("^\\d{3}w$|^\\d{4}w$|^legacy$").unwrap();
    }

    // Directories dont have extensions?! so will simply continue
    match path.extension().and_then(OsStr::to_str)
    {
        // No extension. Move on
        None => (),
        
        Some(s)
                => {
                    if check_extension(s) {
                        if path.metadata().unwrap().len() > opts.min_size
                        {
                            let nm = path.file_stem().and_then(OsStr::to_str).unwrap();

                            // Make sure were not converting a previously converted image. Matching the filename
                            if !RE.is_match(nm)
                            {
                                match process_image(&path, &opts, m) {
                                    Err(e) => { if !opts.is_quiet{eprintln!("WARNING: Processing error {:?} {:?}", path, e)}},
                                    _ => (),
                            }
                            }
                        }
                        else
                        {
                            m.skipped = m.skipped + 1;
                            if !opts.is_quiet{eprintln!("WARNING: Skipping {:?}", path)};
                        }
                    }
                    }
                    ,
    }

    Ok(())
}


fn check_extension(ext: &str) -> bool
{
    match ext.to_lowercase().as_str() {
        #[cfg(feature = "bmp")]
        "bmp" => true,
        #[cfg(feature = "gif")]
        "gif" => true,
        #[cfg(feature = "hdr")]
        "hdr" => true,
        #[cfg(feature = "ico")]
        "ico" => true,
        #[cfg(feature = "jpeg")]
        "jpg" | "jpeg" => true,
        #[cfg(feature = "png")]
        "png" => true,
        #[cfg(feature = "pnm")]
        "pnm" | "pgm" | "ppm" => true,
        #[cfg(feature = "tga")]
        "tga" => true,
        #[cfg(feature = "dds")]
        "dds" => true,
        #[cfg(feature = "tiff")]
        "tif" | "tiff" => true,
        #[cfg(feature = "webp")]
        "webp" => true,
        #[cfg(feature = "farbfeld")]
        "ff" => true,
        #[cfg(any(feature = "avif-encoder", feature = "avif-decoder"))]
        "avif" => true,
        _  => false,
    }
}