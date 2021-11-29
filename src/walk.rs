//! Walk a directory tree, hunting for jpg, png and tiff extensions


use std::path::Path;
use std::ffi::OsStr;

use crate::opts::Opts;
use crate::img::process_image;


/// Walk a directory tree, hunting for jpg, png and tiff extensions
pub fn walk_path(dir: &Path, opts: &Opts)  -> anyhow::Result<()>
{
    lazy_static::lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new("320w|480w|640w|768w|960w|1024w|1280w|1440w|legacy").unwrap();
    }

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if opts.is_recurse && path.is_dir() {
                walk_path(&path, &opts)?;
            } else {
                // Directories dont have extensions?! so will simply continue
                match path.extension().and_then(OsStr::to_str)
                {
                    None => (),
                    Some("jpg") | Some("JPG") | Some("png") | Some("PNG")
                        | Some("tiff") | Some("TIFF") | Some("tif") | Some("TIF")
                            => {
                                    if path.metadata()?.len() > opts.min_size
                                    {
                                        let nm = path.file_stem().and_then(OsStr::to_str).unwrap();

                                        if !RE.is_match(nm)
                                        {
                                            match process_image(&path, &opts) {
                                                Err(e) => eprintln!("WARNING: Processing error {:?} {:?}", path, e),
                                                _ => (),
                                        }
                                        }
                                    }
                                    else
                                    {
                                        eprintln!("WARNING: Skipping {:?}", path);
                                    }
                                }
                                ,
                    _ => (),
                }


            }
        }
    }

    Ok(())
}
