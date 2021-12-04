
//! Takes a filepath, opens an image and the saves the image in the format specified
//! by either the original file extension or that provided in options.

use std::ffi::OsStr;

use image::{DynamicImage, GenericImageView};
use std::path::Path;
use rayon::prelude::*;
use anyhow::Result;

use crate::opts::{Opts, Metrics};
use crate::utils::{use_fileext,mk_dir, path_from_array};



/// Process the image provided in the path.
/// Iterate through the sizes and create a scaled image for each
pub fn process_image(path: &Path, opts: &Opts, m: &mut Metrics) -> Result<()>
{

    let sizes = [320, 480, 640, 768, 960, 1024, 1280, 1440];

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img:DynamicImage = image::open(path)?;
    
    if opts.is_file {
        println!("<< {:?}", path);
    } else {
        println!("<< {:?}", path.strip_prefix(opts.inpath.as_path()).unwrap());
    }

    let wh = img.dimensions();
    let (w,h) = wh;
    let aspect =  w as f32 / h as f32;

    let ext = use_fileext(&path, &opts.extension);
    let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();

    let np = match opts.is_nested {
        true => path_from_array(&[
                        &opts.outpath.to_str().unwrap(),
                        &path.strip_prefix(opts.inpath.as_path()).unwrap().parent().unwrap().to_str().unwrap(),
                        &file_name,
                        &("legacy.".to_owned()+ext)]),
        _ =>    path_from_array(&[
                    &opts.outpath.to_str().unwrap(),
                    &file_name,
                    &("legacy.".to_owned()+ext)]
                    ),
    };

    if opts.is_verbose { println!(">> {:?}", np);}

    if !opts.is_test {
        mk_dir(&np);
        img.save(&np)?;
    }

    // 320,480,640,768,960,1024,1280,1440 pixels wide
    // Iterate through the sizes and create a scaled image for each
    match opts.is_jobs {
    
        // The following uses rayon parallel processes
        true => {
                let _r: Result<Vec<_>, _> = sizes.par_iter().map( |sz|
                        scale_and_save(&path, &opts.outpath, &img, *sz, (*sz as f32 / aspect) as i32, &opts.extension, &opts))
                        .collect();
                },

        false =>
            for n in sizes
            {
                scale_and_save(&path, &opts.outpath, &img, n, (n as f32 / aspect) as i32, &opts.extension, &opts)?;
            }
            ,
     };
    

    // THE SRCSET TAG
    let sp = match opts.is_nested {
        true =>
                path_from_array(&[
                    &opts.prefix,
                    &path.strip_prefix(opts.inpath.as_path()).unwrap().parent().unwrap().to_str().unwrap(),
                    &file_name]
                    ),
        _ =>    path_from_array(&[&opts.prefix,&file_name]),
    };

    let tag = format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/320w.{1} 320w, {0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w, {0}/960w.{1} 960w, {0}/1024w.{1} 1024w, {0}/1280w.{1} 1280w, {0}/1440w.{1} 1440w\" sizes=\"{2}\" alt=\"A file named {3}\">", sp.to_str().unwrap(), ext, opts.sizes, file_name);

    // THE SRCSET.TXT DESINATION
    let f = match opts.is_nested {
        true =>
                path_from_array(&[
                        &opts.outpath.to_str().unwrap(),
                        &path.strip_prefix(opts.inpath.as_path()).unwrap().parent().unwrap().to_str().unwrap(),
                        &file_name,
                        "srcset.txt"]
                        ),

        _ =>    path_from_array(&[
                    &opts.outpath.to_str().unwrap(),
                    &file_name,
                    "srcset.txt"]),
    };

    if opts.is_verbose { println!(">> {:?}", f);}

    println!("\n{}\n\n", tag);

    if !opts.is_test {
        std::fs::write(f, &tag)?;
    }
    
    // Increment the counter
    m.count = m.count + 1;
    m.resized = m.resized+(sizes.len() as u32);

    Ok(())
}



///  Resize the image provided by path and save the resulting new image onto outpath
pub fn scale_and_save(path: &Path, outpath: &Path,
        img: &DynamicImage, nwidth: i32, nheight: i32,
        ext: &str, opts: &Opts ) -> Result<()>
{
    // Filename only with no extension
    let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();

    // The filename extension. jpg, png etc A valid image extension
    let ext = use_fileext(&path, &ext);

    // The new path from names, sizes and file ext
    let img_path = match opts.is_nested {
        true =>
                path_from_array(&[
                        &outpath.to_str().unwrap(),
                        &path.strip_prefix(opts.inpath.as_path()).unwrap().parent().unwrap().to_str().unwrap(),
                        &file_name,
                        &(nwidth.to_string().to_owned() + "w." + ext)]),
        _ =>    path_from_array(&[
                        &outpath.to_str().unwrap(),
                        &file_name,
                        &(nwidth.to_string().to_owned() + "w." + ext)]),
    };

     if opts.is_verbose { println!(">> {:?}", img_path);}

    if !opts.is_test {
        let scaled = img.resize_to_fill(nwidth as u32, nheight as u32, image::imageops::FilterType::Lanczos3);
        scaled.save(&img_path)?;
    }

    Ok(())
}

