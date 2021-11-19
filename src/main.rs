/*

The srcset utility generates multiple (eight) scaled versions of an image at particular breakpoints -- 320,480,640,768,960,1024,1280,1440 pixels wide -- that match common Mobile and widescreen viewports using Imagemagick's convert utility and outputs the needed <img> tag.

    str='<img src="'$outprefix$srcsuffix.$type'" srcset="'$outprefix-320w.$type' 320w, '$outprefix-480w.$type' 480w, '$outprefix-640w.$type' 640w, '$outprefix-768w.$type' 768w, '$outprefix-960w.$type' 960w, '$outprefix-1024w.$type' 1024w, '$outprefix-1440w.$type' 1440w" sizes="'$sizes'" alt="An image named '$filename'"/>'


*/

//extern crate argparse;
//extern crate image;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::ffi::OsStr;


use argparse::{ArgumentParser, StoreTrue, Store};
use image::{GenericImageView};

// ./srset.sh [-hmz] [-q quality] [—t type] [-l legacysize] [-s sizes] [-o out path] [-f filename] filename

// ./srset.sh [-hmz] [—n findname] [-q quality] [—t type] [-l legacysize] [-s sizes] [-o out path] [-f file hierarchy] file hierarchy

//



fn main() {

    let mut inpath_str = "filename".to_string();
    let mut outpath_str = ".".to_string();
    let mut extension = "".to_string();

    let mut is_recurse = false;
    let mut is_test = false;

    {
        let mut args = ArgumentParser::new();

        args.set_description("srcset command-line utility");

        args.refer(&mut inpath_str)
                .required()
                .add_option(&["-p", "--path"], Store,
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

        args.refer(&mut is_test)
                .add_option(&["-t", "--test"], StoreTrue,
                "Test run. Images are found but not created");

        args.parse_args_or_exit();
    }

    let outpath = Path::new(&outpath_str);
    if outpath.is_file() {
        println!("Selected outpath is a file");
        std::process::exit(1);
    }
    // Make sure outpath has suffix '/'
    if outpath.ends_with("/") {
        println!("Remove trailing /");
    }

    //println!("Outpath parent {}, filename {}, ", outpath.parent().unwrap().to_str().unwrap(), outpath.file_stem().and_then(OsStr::to_str).unwrap(),  );
        

    let inpath = Path::new(&inpath_str);

    let options = Options {inpath, outpath, extension, is_recurse, is_test};

    
    if inpath.is_dir() {
        let _noop = loop_path(&inpath, &outpath, is_recurse, is_test, &options);
    }
    else
    {
        process_image(&inpath, &outpath, is_test);
    }

}

/// \fn             visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry))
/// \brief          Recurse through a directory tree
///
/// \param dir          The path of the directory
/// \param outpath      The path of the output directory
/// \param is_recurse   Whether to recurse directory
fn loop_path(dir: &Path, outpath: &Path, is_recurse: bool, is_test: bool)  -> io::Result<()>
{
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if is_recurse && path.is_dir() {
                loop_path(&path, &outpath, is_recurse, is_test, &options)?;
            } else {
                match path.extension().and_then(OsStr::to_str)
                {
                    None => (),
                    Some("jpg") | Some("JPG") | Some("png") | Some("PNG") => process_image(&path, &outpath, is_test),
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
/// \param dir          The path of the directory
/// \param is_recurse   Whether to recurse directory
fn process_image(path: &Path, outpath: &Path, is_test: bool)
{
    let file_path = path.to_str().unwrap();

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(file_path).unwrap();
    
    let wh = img.dimensions();
    let (w,h) = wh;

    let aspect =  w as f32 / h as f32;
    
    println!("Processing image {}: {:?} {:.2} {:?}", file_path, wh, aspect, img.color());

    // 320,480,640,768,960,1024,1280,1440 pixels wide
    let sizes = [320, 480, 640, 768, 960, 1024, 1280, 1440];
    for n in sizes {
        scale_and_save(&path, &outpath, &img, n as u32, (n as f32 / aspect) as u32, is_test);
    }
    
}

/// \fn             scale_and_save(path: &Path, img: &image::DynamicImage, nwidth: u32, nheight: u32)
/// \brief          Resize provided image and save the resulting new image onto disk
///
/// \param path     The path of the directory
/// \param img      The initial image
/// \param nwidth   Width of the new image
/// \param nheight  Height of the new image

fn scale_and_save(path: &Path, outpath: &Path, img: &image::DynamicImage, nwidth: u32, nheight: u32, is_test: bool)
{
    let file_parent = path.parent().unwrap().to_str().unwrap();
//    println!("  Parent {:?}", file_parent);
    
//    println!("  Outpath {:?}", outpath.parent().unwrap().to_str().unwrap());

//    if outpath == path.parent().unwrap() {
//        println!("  Root is the same");
//    }
    
//    let file_name = path.file_name();
     let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();
//     println!(" Filename {:?}", file_name);

    let file_ext = path.extension().and_then(OsStr::to_str).unwrap();
//    println!("  Ext {:?}", file_ext);


    // The new path from names, sizes and file ext
//    let new_name = format!("{}/{}-{}x{}.{}",file_parent, file_name, nwidth, nheight, file_ext);
    let new_name = format!("{}{}-{}x{}.{}",outpath.to_str().unwrap(), file_name, nwidth, nheight, file_ext);
    println!("New image {}", new_name);

    if !is_test {
        let scaled = img.resize_to_fill(nwidth, nheight, image::imageops::FilterType::Lanczos3);
        // Write the contents of this image to the Writer in format found in extension.
        scaled.save(new_name).unwrap();
    }



}



#[allow(dead_code)]
fn extension_from_path(path: &Path) -> Option<&str>
{
    path.extension().and_then(OsStr::to_str)
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
