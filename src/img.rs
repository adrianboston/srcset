
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
//    let _all_sizes = [480, 640, 768, 960, 1024, 1366, 1600, 1920];
    
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
    
    // Pick maximum array slice based on width of provided image
    let sizes = strip_sizes(w);
    let max = *sizes.last().unwrap();
    let sz = path.metadata().unwrap().len();
    
    println!("Image: Width={}: Height={}; Size={}; Color={:?}", w, h, human_bytes::human_bytes(sz as f64), img.color());

    let ext = use_fileext(&path, &opts.extension);
    let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();
    
    
    // Legacy should use the largest size not necessarily the initial size. Could be far too big
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

    // For the legacy image, do not just copy the original resize to max size of 1440 or less
    if !opts.is_test {
        //let w = sizes.last().unwrap();
        let legacy_img = img.resize_to_fill(max as u32, (max as f32/aspect) as u32, image::imageops::FilterType::Lanczos3);

        mk_dir(&np);
        legacy_img.save(&np)?;
    }
    

    // 320,480,640,768,960,1024,1280,1440 pixels wide
    // Iterate through the sizes and create a scaled image for each
    match opts.is_jobs {
    
        // The following uses rayon parallel processes
        true => {
                let _r: Result<Vec<_>, _> = sizes.par_iter().map( |sz|
                        scale_and_save(&path, &opts.outpath, &img, *sz, (*sz as f32 / aspect) as u32, &opts.extension, &opts))
                        .collect();
                },

        false =>
            for n in sizes
            {
                scale_and_save(&path, &opts.outpath, &img, *n, (*n as f32 / aspect) as u32, &opts.extension, &opts)?;
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
    
//    let tag = format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/320w.{1} 320w, {0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w, {0}/960w.{1} 960w, {0}/1024w.{1} 1024w, {0}/1280w.{1} 1280w, {0}/1440w.{1} 1440w\" sizes=\"{2}\" alt=\"A file named {3}\">", sp.to_str().unwrap(), ext, opts.sizes, file_name);
    let tag = create_tag(max, sp.to_str().unwrap(), ext, file_name);

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
        img: &DynamicImage, nwidth: u32, nheight: u32,
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

/// Provide an array that is suitable for large and small images based on the width
fn strip_sizes(max: u32) -> &'static [u32]
{
    match max {
        d if d < 480 => &[480],
        d if d < 640 => &[480,640],
        d if d < 768 => &[480,640,768],
        d if d < 960 => &[480,640,768,960],
        d if d < 1024 => &[480,640,768,960,1024],
        d if d < 1366 => &[480,640,768,960,1024,1366],
        d if d < 1600 => &[480,640,768,960,1024,1366,1600],
        _ => &[480,640,768,960,1024,1366,1600,1920],
    }
}

/// Provide an <img srcset=""> tag with the image names, smaller images get smaller sets of images
fn create_tag<'a>(max: u32, f: &'a str, ext: &'a str, n: &'a str) -> String
{
    let tag =
    match max {
        d if d < 480 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/480w.{1}\" sizes=\"(max-width:480px) 100vw, min-width:481px) 25vw\" alt=\"A file named {2}\">",f, ext, n),
        d if d < 640 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/480w.{1} 480w, {0}/640w.{1} 640w\" sizes=\"(max-width:640px) 100vw, min-width:641px) 33vw\" alt=\"A file named {2}\">",f, ext, n),
        d if d < 768 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w\" sizes=\"(max-width:320px) 50vw, (max-width:768px) 100vw, min-width:769px) 50vw\" alt=\"A file named {2}\">",f, ext, n),
        d if d < 960 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w, {0}/960w.{1} 960w\" sizes=\"(max-width:320px) 50vw, (max-width:960px) 75vw, min-width:961px) 95vw\" alt=\"A file named {2}\">",f, ext, n),
        d if d < 1024 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w, {0}/960w.{1} 960w, {0}/1024w.{1} 1024w\" sizes=\"(max-width:320px) 50vw, (max-width:960px) 75vw, min-width:961px) 95vw\" alt=\"A file named {2}\">",f, ext, n),
        d if d < 1366 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w, {0}/960w.{1} 960w, {0}/1024w.{1} 1024w, {0}/1366w.{1} 1366w\" sizes=\"(max-width:320px) 50vw, (max-width:960px) 75vw, (min-width:961px) 95vw\" alt=\"A file named {2}\">",f, ext, n),
        d if d < 1660 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w, {0}/960w.{1} 960w, {0}/1024w.{1} 1024w, {0}/1366w.{1} 1366w, {0}/1600w.{1} 1600w\" sizes=\"(max-width:320px) 25vw, (min-width: 960px) 75vw, 100vw\" alt=\"A file named {2}\">",f, ext, n),
        _ =>             format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w, {0}/960w.{1} 960w, {0}/1024w.{1} 1024w, {0}/1366w.{1} 1366w, {0}/1600w.{1} 1600w, {0}/1920w.{1} 1920w\" sizes=\"(min-width: 1024px) 50vw, 100vw\" alt=\"A file named {2}\">",f, ext, n),
    };

    println!("{}",tag);
    tag
}
