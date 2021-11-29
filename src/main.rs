//! \details The srcset utility generates multiple (eight) scaled versions of an image at particular breakpoints
//!  those of 320,480,640,768,960,1024,1280,1440 pixels wide that match common Mobile and widescreen
//! viewports using  convert utility and outputs the needed <img> tag.

mod utils;
mod opts;
mod img;
mod walk;

use std::path::{Path, PathBuf};

use crate::opts::Opts;
use crate::img::process_image;
use crate::walk::walk_path;


fn main() {
   
   // The defaults!
    let mut inpath_str = "".to_string();
    let mut outpath_str = "/tmp/srcset/".to_string();
    
    let mut is_file = false;
    let mut extension = "".to_string();
    let mut sizes = "(min-width: 768px) 50vw, 100vw".to_string();
    let mut is_recurse = false;
    let mut is_jobs = false;
    let mut is_nested = false;
    let mut is_test = false;
    let mut is_verbose = false;

    
    let mut min_kb = 100;

    {
        let mut args = argparse::ArgumentParser::new();

        args.set_description("srcset command-line utility");

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

        args.refer(&mut min_kb)
                .add_option(&["-m", "--min"], argparse::Store,
                "Minimum size of image to process in kb, otherwise skip");

        args.refer(&mut is_jobs)
                .add_option(&["-j", "--job"], argparse::StoreTrue,
                "Use parallel jobs");

        args.refer(&mut is_nested)
                .add_option(&["-n", "--nested"], argparse::StoreTrue,
                "Images are saved in a nested (not flat) hierarchy");

        args.refer(&mut is_test)
                .add_option(&["-z", "--test"], argparse::StoreTrue,
                "Test run. Images are found but not created");

        args.refer(&mut is_verbose)
                .add_option(&["-v", "--verbose"], argparse::StoreTrue,
                "Verbose output");

        args.refer(&mut inpath_str)
                .add_argument("file", argparse::Store,
                "Path (Filename or directory) of image");


        args.parse_args_or_exit();
    }
    
    // Output must end in /
    if !outpath_str.ends_with("/") {  outpath_str.push_str("/"); }

    if inpath_str == "" {
        println!("File or directory argument is required.");
        std::process::exit(1);
    }
    let inpath = PathBuf::from(&inpath_str);
    
    if inpath.is_file() {
        is_nested = false;
        is_recurse = false;
        is_file = true;
    }
    
    let outpath = PathBuf::from(&outpath_str);
    if outpath.is_file() {
        println!("Selected outpath cannot be a file.");
        std::process::exit(1);
    }


    let opts = Opts{inpath: inpath, outpath: outpath, is_file: is_file, extension: extension, sizes: sizes, min_size: min_kb * 1024, is_recurse: is_recurse, is_jobs: is_jobs, is_nested: is_nested, is_test: is_test, is_dir: true, is_verbose: is_verbose};

    let inpath = Path::new(&inpath_str);

    let _result =
        match inpath.is_dir()
        {
            true => walk_path(&inpath, &opts),
            _ => process_image(&inpath, &opts),
        };
    
    //println!("{:?}", result);
}

