
//! Takes a filepath, opens an image and the saves the image in the format specified
//! by either the original file extension or that provided in options.


use std::path::Path;
use std::ffi::OsStr;
use rayon::prelude::*;
use anyhow::Result;
use image::{DynamicImage};
use image::GenericImageView;
use yansi::Paint;

use crate::opts::{Opts, Metrics};
use crate::utils::{use_fileext,mk_dir, path_from_array};
use crate::img_ext::ImgExt;


/// Process the image provided in the path.
/// Iterate through the sizes and create a scaled image for each
pub fn process_image(path: &Path, opts: &Opts, m: &mut Metrics) -> Result<()>
{
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img:DynamicImage = image::open(path)?;

    if opts.is_file {
        println!("{:?}", Paint::green(&path));
    } else {
        println!("{:?}", Paint::green(path.strip_prefix(opts.inpath.as_path()).unwrap()));
    }

    let (w,h) = img.dimensions();
    let aspect =  w as f32 / h as f32;

    // Pick maximum array slice based on width of image
    let sizes = match strip_sizes(w, &opts.sizes) {
        None => return Ok(()),
        Some(v) => v, 
    };

    // The largest size is the legacy one
    let max = sizes.last().unwrap();
    
    if opts.is_verbose { print_image_details(&img, &path)};

    let ext = use_fileext(&path, &opts.extension);
    let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();
    
    // Legacy should use the largest size of the provided range not the initial size. Could be too large
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

    // For the legacy image, do not just copy the original resize to max size of 1440 or less
    if !opts.is_test {
        //let w = sizes.last().unwrap();
        let legacy_img = img.resize_to_fill(*max, (*max as f32/aspect) as u32, image::imageops::FilterType::Lanczos3);
        legacy_img.unsharpen(opts.sigma, opts.thresh);

        mk_dir(&np);
        legacy_img.save(&np)?;
        //legacy_img.save_with_quality(&np, opts.quality)?;

        if opts.is_verbose {print_image_details(&legacy_img, &np)}

    }
    m.resized = m.resized + 1;  // One resize for legacy    
    

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
            for n in &sizes
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
    
    let tag = create_tag(*max, sp.to_str().unwrap(), ext, file_name, &opts.sizes);

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

    if opts.is_verbose { println!("{:?}", f);}

    println!("\n{}\n\n", Paint::blue(&tag) );

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


    if !opts.is_test {
        let scaled =    img.resize_to_fill(nwidth as u32, nheight as u32, image::imageops::FilterType::Lanczos3);
        scaled.unsharpen(opts.sigma, opts.thresh);
        
        //scaled.save(&img_path)?;
        scaled.save_with_quality(&img_path, opts.quality)?;

        if opts.is_verbose {print_image_details(&img, &img_path)}
    } else {
        if opts.is_verbose { println!(">> {:?}", img_path);}
    }

    Ok(())
}

/// Provide an array that is suitable for large and small images based on the width
fn strip_sizes(max: u32, sizes: &Vec<u32>) -> Option<Vec<u32>>
{
   let mut vec = vec![];
   for x in 0 .. sizes.len() {
       if max >= sizes[x] {
           vec.push(sizes[x])
       }
   }
   // return None if empty vec
   if vec.len() == 0 {
       return None;}
   else {
       Some(vec)}
}

/// Provide an <img srcset=""> tag with the image names, smaller images get smaller sets of images
fn create_tag<'a>(max: u32, f: &'a str, ext: &'a str, n: &'a str, sizes: &Vec<u32>) -> String
{

    let mut string = String::new();
    for x in 0 ..sizes.len() {
        if max >= sizes[x] {
            let s =
            match x {
                0 => format!("{0}/{1}w.{2} {1}w",f, sizes[x], ext),
                _ => format!(",{0}/{1}w.{2} {1}w",f, sizes[x], ext),
            };
            string.push_str(&s);
        }
    }
 
    let tag =
    match max {
        d if d < 480 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{3}\" sizes=\"(max-width:480px) 100vw, min-width:481px) 25vw\" alt=\"A file named {2}\">",f, ext, n, string),
        d if d < 640 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{3}\" sizes=\"(max-width:640px) 100vw, min-width:641px) 33vw\" alt=\"A file named {2}\">",f, ext, n, string),
        d if d < 768 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{3}\" sizes=\"(max-width:320px) 50vw, (max-width:768px) 100vw, min-width:769px) 50vw\" alt=\"A file named {2}\">",f, ext, n, string),
        d if d < 960 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{3}\" sizes=\"(max-width:320px) 50vw, (max-width:960px) 75vw, min-width:961px) 95vw\" alt=\"A file named {2}\">",f, ext, n, string),
        d if d < 1024 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{3}\" sizes=\"(max-width:320px) 50vw, (max-width:960px) 75vw, min-width:961px) 95vw\" alt=\"A file named {2}\">",f, ext, n, string),
        d if d < 1366 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{3}\" sizes=\"(max-width:320px) 50vw, (max-width:960px) 75vw, (min-width:961px) 95vw\" alt=\"A file named {2}\">",f, ext, n, string),
        d if d < 1660 => format!("<img src=\"{0}/legacy.{1}\" srcset=\"{3}\" sizes=\"(max-width:320px) 25vw, (min-width: 960px) 75vw, 100vw\" alt=\"A file named {2}\">",f, ext, n, string),
        _ =>             format!("<img src=\"{0}/legacy.{1}\" srcset=\"{3}\" sizes=\"(min-width: 1024px) 50vw, 100vw\" alt=\"A file named {2}\">",f, ext, n, string),
    };

    tag
}


fn print_image_details(img: &DynamicImage, path: &Path) {
    let (w,h) = img.dimensions();
    let sz = path.metadata().unwrap().len();
    println!("{:?} Width={}: Height={}; Size={}; Color={:?}", path, Paint::red(w), h, Paint::red(human_bytes::human_bytes(sz as f64)), img.color());
}



/*
fn encode_image_jpeg(path: &Path, img: &DynamicImage, nwidth: u32, nheight: u32) -> Result<()> 
{

    let fin = File::open(path)?;
    let buf = BufReader::new(fin);


    let mut encoder = image::jpeg::JpegEncoder::new_with_quality(&mut buf, 80);

//    let img = image::ImageBuffer::new(512, 512);

    let bytes = img.as_bytes();

    encoder.encode(&bytes, nwidth, nheight, image::ColorType::Rgba8)?;

    buffer.write_all(bytes)?;

    Ok(())
}
*/
