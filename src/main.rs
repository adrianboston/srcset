/*

The srcset utility generates multiple (eight) scaled versions of an image at particular breakpoints -- 320,480,640,768,960,1024,1280,1440 pixels wide -- that match common Mobile and widescreen viewports using Imagemagick's convert utility and outputs the needed <img> tag.

*/

//extern crate argparse;
//extern crate image;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::ffi::OsStr;


use argparse::{ArgumentParser, StoreTrue, Store};
use image::{GenericImageView};


struct Options<'a> {
    outpath: &'a Path,
    extension: String,
    sizes: String,
    is_recurse: bool,
    is_test: bool,
}


fn main() {

    let mut inpath_str = "".to_string();
    let mut outpath_str = "".to_string();
    let mut extension = "".to_string();
    let mut sizes = "(min-width: 768px) 50vw, 100vw".to_string();
    let mut is_recurse = false;
    let mut is_test = false;

    {
        let mut args = ArgumentParser::new();

        args.set_description("srcset command-line utility");

        args.refer(&mut inpath_str)
                .required()
                .add_option(&["-f", "--file"], Store,
                "Path (Filename or directory) of image");
 
        args.refer(&mut outpath_str)
                .add_option(&["-o", "--out"], Store,
                "Output directory)");

        args.refer(&mut is_recurse)
                .add_option(&["-r", "--recurse"], StoreTrue,
                "Recurse directories");

        args.refer(&mut extension)
                .add_option(&["-t", "--type"], Store,
                "Output filetype (jpg, png, etc)");

        args.refer(&mut sizes)
                .add_option(&["-s", "--sizes"], Store,
                "The src viewport sizes tag as string");

        args.refer(&mut is_test)
                .add_option(&["-z", "--test"], StoreTrue,
                "Test run. Images are found but not created");

        args.parse_args_or_exit();
    }
    
    // Output must end in /
    if !outpath_str.ends_with("/") {  outpath_str.push_str("/"); }

    let outpath = Path::new(&outpath_str);
    if outpath.is_file() {
        println!("Selected outpath is a file");
        std::process::exit(1);
    }

    let inpath = Path::new(&inpath_str);

    let options = Options {outpath, extension, sizes, is_recurse, is_test};

    let result =
    match inpath.is_dir()
    {
        true => loop_path(&inpath, &options),
        _ => process_image(&inpath, &options),
    };
    println!("{:?}", result);

}

/// \fn             visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry))
/// \brief          Recurse through a directory tree
///
/// \param dir          The path of the directory
/// \param outpath      The path of the output directory
/// \param is_recurse   Whether to recurse directory
fn loop_path(dir: &Path, options: &Options)  -> io::Result<()>
{
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if options.is_recurse && path.is_dir() {
                loop_path(&path, &options)?;
            } else {
                // Directories dont have extensions?! so will simply continue
                match path.extension().and_then(OsStr::to_str)
                {
                    None => (),
                    Some("jpg") | Some("JPG") | Some("png") | Some("PNG")
                        | Some("tiff") | Some("TIFF")
                            => process_image(&path, &options)?,
                    _ => (),
                }


            }
        }
    }
    
    Ok(())

}

/// \fn             process_image(path: &Path)
/// \brief          Process one image
///
/// \param dir          The path of the current file
/// \param is_recurse   Whether to recurse directory
fn process_image(path: &Path, options: &Options) -> io::Result<()>
{
    let file_path = path.to_str().unwrap();

    let sizes = [320, 480, 640, 768, 960, 1024, 1280, 1440];

    if !options.is_test
    {
        // This is slow. No point opening in a test???
        // Use the open function to load an image from a Path.
        // `open` returns a `DynamicImage` on success.
        let img = image::open(file_path).unwrap();
    
        let wh = img.dimensions();
        let (w,h) = wh;

        let aspect =  w as f32 / h as f32;
    
        println!("\nProcessing image {}: {:?} {:.2} {:?}", file_path, wh, aspect, img.color());
    
        // Outpath must exist. Then create directory based on image name (not incuding extension)
        let j = options.outpath.join(path.file_stem().and_then(OsStr::to_str).unwrap());
        let p = j.as_path();
        if !p.exists() {
            std::fs::create_dir(p)?;
        }
    
        // Saving the legacy image
        let legacy_name = format!("{}{}/legacy.{}",options.outpath.to_str().unwrap(), path.file_stem().and_then(OsStr::to_str).unwrap(), get_file_ext(&path, &options) );
        img.save(legacy_name).unwrap();

        // 320,480,640,768,960,1024,1280,1440 pixels wide
        // Iterate through the sizes and create a scaled image for each
        for n in sizes {
            scale_and_save(&path, &options.outpath, &img, n as u32, (n as f32 / aspect) as u32, &options);
        }
    }
    else
    {
        println!("\nProcessing image {}", file_path);
    }
    
    
    // End of iteration
    let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();
    let file_ext: &str;
    if options.extension == "" {
        file_ext = path.extension().and_then(OsStr::to_str).unwrap();
    } else {
        file_ext = options.extension.as_str();
    }
    
    // Produce the output path of new files
    println!("Images placed in {0}{1}/320w.{2}, and /480w.{2}, /640w.{2}, /768w.{2}, /960w.{2}, /1024w.{2}, /1280w.{2}, 1440w.{2}", options.outpath.to_str().unwrap(), file_name, file_ext);

    // Now output the srcset tag
    let srcset_tag = format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/320w.{1} 320w, {0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w, {0}/960w.{1} 960w, {0}/1024w.{1} 1024w, {0}/1280w.{1} 1280w, {0}/1440w.{1} 1440w\" sizes=\"{2}\" alt=\"A file named {0}\">", file_name, file_ext, options.sizes);

    println!("Output srcset tag\n{}", srcset_tag);
    
    Ok(())
}

/// \fn             scale_and_save(path: &Path, img: &image::DynamicImage, nwidth: u32, nheight: u32)
/// \brief          Resize provided image and save the resulting new image onto disk
///
/// \param path     The path of the directory
/// \param img      The initial image
/// \param nwidth   Width of the new image
/// \param nheight  Height of the new image

fn scale_and_save(path: &Path, outpath: &Path, img: &image::DynamicImage, nwidth: u32, nheight: u32, options: &Options)
{
    
    let file_name = path.file_stem().and_then(OsStr::to_str).unwrap(); //?

    // The new path from names, sizes and file ext
    let file_ext: &str;
    if options.extension == "" {
        file_ext = path.extension().and_then(OsStr::to_str).unwrap(); //?
    } else {
        file_ext = options.extension.as_str();
    }

    let new_name = format!("{}{}/{}w.{}",outpath.to_str().unwrap(), file_name, nwidth, file_ext);
    
    let scaled = img.resize_to_fill(nwidth, nheight, image::imageops::FilterType::Lanczos3);

    // Write the contents of this image in format found in file_ext.
    scaled.save(&new_name).unwrap();

    println!("Created {}", new_name);
}



// The new path from names, sizes and file ext
fn get_file_ext<'life>(path: &'life Path, options: &'life Options) -> &'life str
{
    match options.extension.as_str()
    {
        "" => path.extension().and_then(OsStr::to_str).unwrap(),
        _ => options.extension.as_str(),
    }
}


#[allow(dead_code)]
fn loop_dir(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()>
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                loop_dir(&path, cb)?;
            } else {

                //println!("File {:?}", entry);

                cb(&entry);
            }
        }
    }
    Ok(())
}
