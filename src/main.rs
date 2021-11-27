/*

The srcset utility generates multiple (eight) scaled versions of an image at particular breakpoints -- 320,480,640,768,960,1024,1280,1440 pixels wide -- that match common Mobile and widescreen viewports using Imagemagick's convert utility and outputs the needed <img> tag.

*/

use std::ffi::OsStr;
use image::{GenericImageView};
use rayon::prelude::*;
use std::path::{Path, PathBuf};


#[derive(Debug)]
struct ProgOptions {
    inpath:  PathBuf,
    outpath: PathBuf,
    extension: String,
    sizes: String,
    is_recurse: bool,
    is_test: bool,
    is_jobs: bool,
    is_nested: bool
}




fn main() {
   
   // The defaults!
    let mut inpath_str = ".".to_string();
    let mut outpath_str = "/tmp/srcset/".to_string();
    let mut extension = "".to_string();
    let mut sizes = "(min-width: 768px) 50vw, 100vw".to_string();
    let mut is_recurse = false;
    let mut is_jobs = false;
    let mut is_nested = false;
    let mut is_test = false;

    {
        let mut args = argparse::ArgumentParser::new();

        args.set_description("srcset command-line utility");

        args.refer(&mut inpath_str)
                .required()
                .add_option(&["-f", "--file"], argparse::Store,
                "Path (Filename or directory) of image");
 
        args.refer(&mut outpath_str)
                .add_option(&["-o", "--out"], argparse::Store,
                "Output directory)");

        args.refer(&mut is_recurse)
                .add_option(&["-r", "--recurse"], argparse::StoreTrue,
                "Recurse directories");

        args.refer(&mut extension)
                .add_option(&["-t", "--type"], argparse::Store,
                "Output filetype (jpg, png, etc)");

        args.refer(&mut sizes)
                .add_option(&["-s", "--sizes"], argparse::Store,
                "The src viewport sizes tag as string");

        args.refer(&mut is_jobs)
                .add_option(&["-j", "--job"], argparse::StoreTrue,
                "Use parallel jobs");

        args.refer(&mut is_nested)
                .add_option(&["-n", "--nested"], argparse::StoreTrue,
                "Images are saved in a nested (not flat) hierarchy");

        args.refer(&mut is_test)
                .add_option(&["-z", "--test"], argparse::StoreTrue,
                "Test run. Images are found but not created");

        args.parse_args_or_exit();
    }
    
    // Output must end in /
    if !outpath_str.ends_with("/") {  outpath_str.push_str("/"); }

    let inpath = PathBuf::from(&inpath_str);
    
    let outpath = PathBuf::from(&outpath_str);
    if outpath.is_file() {
        println!("Selected outpath is a file");
        std::process::exit(1);
    }

    let prog_options = ProgOptions{inpath, outpath, extension, sizes, is_recurse, is_jobs, is_nested, is_test};

    let inpath = Path::new(&inpath_str);

    let result =
        match inpath.is_dir()
        {
            true => loop_path(&inpath, &prog_options),
            _ => process_image(&inpath, &prog_options),
        };
    
    
    println!("{:?}", result);

}

/// \fn             loop_path(dir: &Path, prog_options: &ProgOptions)  -> std::io::Result<()>
/// \brief          Recurse a directory tree, hunting for jpg, png and tiff extensions
///
/// \param path     The path of the current file
/// \param options  The program options

fn loop_path(dir: &Path, prog_options: &ProgOptions)  -> anyhow::Result<()>
{
    
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if prog_options.is_recurse && path.is_dir() {
                loop_path(&path, &prog_options)?;
            } else {
                // Directories dont have extensions?! so will simply continue
                match path.extension().and_then(OsStr::to_str)
                {
                    None => (),
                    Some("jpg") | Some("JPG") | Some("png") | Some("PNG")
                        | Some("tiff") | Some("TIFF")
                            => match process_image(&path, &prog_options) {
                                    Err(e) => println!("\nERROR: processing image {:?} {:?}\n", path, e),
                                    _ => (),
                                }
                                ,
                    _ => (),
                }


            }
        }
    }
    
    Ok(())

}

/// \fn             process_image(path: &Path, prog_options: &ProgOptions) -> std::io::Result<()>
/// \brief          Process one image
///
/// \param path     The path of the current file
/// \param options  The program options

fn process_image(path: &Path, prog_options: &ProgOptions) -> anyhow::Result<()>
{

    let sizes = [320, 480, 640, 768, 960, 1024, 1280, 1440];

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(path)?;

    let wh = img.dimensions();
    let (w,h) = wh;

    let aspect =  w as f32 / h as f32;

    let x = path.strip_prefix(prog_options.inpath.as_path());
    let y = path.file_stem().unwrap();

    println!("Final {:?} {:?}", x, y);

    let ext = use_fileext(&path, &prog_options.extension);

    let np = match prog_options.is_nested {
        true =>
                path_from_strs_dest(
                        prog_options.outpath.to_str().unwrap(),
                        &path.strip_prefix(prog_options.inpath.as_path()).unwrap().parent().unwrap().to_str().unwrap(),
                        &path.file_stem().and_then(OsStr::to_str).unwrap(),
                        &"legacy",
                        ext
                        ),
        
        _ =>    path_from_strs(
                    &prog_options.outpath.to_str().unwrap(),
                    &path.file_stem().and_then(OsStr::to_str).unwrap(),
                    &"legacy",
                    &ext
                    ),
    };

    if !prog_options.is_test {
        mk_dir(&np);
        img.save(&np)?;
    }

    // 320,480,640,768,960,1024,1280,1440 pixels wide
    // Iterate through the sizes and create a scaled image for each

    match prog_options.is_jobs {
        // The following uses rayon parallel processes
        true =>
            sizes.par_iter().for_each( |sz| scale_and_save(&path, &prog_options.outpath, &img, *sz, (*sz as f32 / aspect) as i32, &prog_options.extension, &prog_options) ),

        false =>
            for n in sizes {
                scale_and_save(&path, &prog_options.outpath, &img, n, (n as f32 / aspect) as i32, &prog_options.extension, &prog_options);
            },
     }


    let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();
    let ext = use_fileext(&path, &prog_options.extension);
    
    // Now output the srcset tag
    let tag = format!("<img src=\"{0}/legacy.{1}\" srcset=\"{0}/320w.{1} 320w, {0}/480w.{1} 480w, {0}/640w.{1} 640w, {0}/768w.{1} 768w, {0}/960w.{1} 960w, {0}/1024w.{1} 1024w, {0}/1280w.{1} 1280w, {0}/1440w.{1} 1440w\" sizes=\"{2}\" alt=\"A file named {0}\">", file_name, ext, prog_options.sizes);


    let f = match prog_options.is_nested {
        true =>
                path_from_strs_dest(
                        prog_options.outpath.to_str().unwrap(),
                        &path.strip_prefix(prog_options.inpath.as_path()).unwrap().parent().unwrap().to_str().unwrap(),
                        &file_name,
                        "srcset",
                        "txt"
                        ),
        
        _ =>    path_from_strs(
                    &prog_options.outpath.to_str().unwrap(),
                    &file_name,
                    "srcset",
                    "txt"
                    ),
    };
    
    
    if !prog_options.is_test {
        std::fs::write(f, &tag)?;
    }

    println!("Srcset tag: {}", tag);
    
    Ok(())
}


/// \fn             scale_and_save(path: &Path, img: &image::DynamicImage, nwidth: u32, nheight: u32)
/// \brief          Resize provided image and save the resulting new image onto disk
///
/// \param path     The original path of the image
/// \param outpath  The path of the outpath directory
/// \param img      The initial image
/// \param nwidth   Width of the new image
/// \param nheight  Height of the new image
/// \param nheight  Extension of the new image


fn scale_and_save(path: &Path, outpath: &Path,
        img: &image::DynamicImage, nwidth: i32, nheight: i32, ext: &str, prog_options: &ProgOptions )
{
    // Filename only with no extension
    let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();
    
    // The filename extension. jpg, png etc A valid image extension
    let ext = use_fileext(&path, &ext);
    
    // The new path from names, sizes and file ext
    let img_path = match prog_options.is_nested {
        true =>
                path_from_strs_dest(
                        &outpath.to_str().unwrap(),
                        &path.strip_prefix(prog_options.inpath.as_path()).unwrap().parent().unwrap().to_str().unwrap(),
                        &file_name,
                        &(nwidth.to_string().to_owned() + "w"),
                        &ext
                        ),
        
        _ =>    path_from_strs(
                        &outpath.to_str().unwrap(),
                        &file_name,
                        &(nwidth.to_string().to_owned() + "w"),
                        ext
                        ),
    };
 
    if !prog_options.is_test {
        let scaled = img.resize_to_fill(nwidth as u32, nheight as u32, image::imageops::FilterType::Lanczos3);
        scaled.save(&img_path).expect("\nERROR Image failed to write\n");
    }
}

fn path_from_strs_dest(dest: &str, root: &str, parent: &str, filename: &str, ext: &str) -> PathBuf {
    let mut pb = PathBuf::new();
    pb.push(dest);
    pb.push(root);
    pb.push(parent);
    
    let f = filename.to_owned() + "." + ext;
    pb.push(f);
    
    println!("Creating Path: {:?}", pb);
    pb
}

fn path_from_strs(root: &str, parent: &str, filename: &str, ext: &str) -> PathBuf {
    let mut pb = PathBuf::new();
    pb.push(root);
    pb.push(parent);
    
    let f = filename.to_owned() + "." + ext;
    pb.push(f);
    
    println!("Creating Path: {:?}", pb);
    pb
}

fn path_from_paths(root: &Path, parent: &Path, filename: &str, ext: &str) -> PathBuf {
    let mut pb = PathBuf::new();
    pb.push(root);
    pb.push(parent);
    
    let f = filename.to_owned() + "." + ext;
    pb.push(f);

    println!("Creating Path: {:?}", pb);
    pb
}

fn mk_dir(p: &Path) {
        match std::fs::create_dir_all(&p.parent().unwrap() ) {
            Err(_) => (),
            _ => (),
        }
}

// The new path from names, sizes and file ext
fn use_fileext<'a>(path: &'a Path, extension: &'a str) -> &'a str
{
    match extension
    {
        "" => path.extension().and_then(OsStr::to_str).unwrap(),
        _ => extension,
    }
}





#[allow(dead_code)]
fn loop_dir(dir: &Path, cb: &dyn Fn(&std::fs::DirEntry)) -> std::io::Result<()>
{
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
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

