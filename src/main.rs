/*! Command-line entry point for srcset.

Usage: `srcset [OPTIONS] <FILE_OR_DIRECTORY>`.
Run `srcset --help` for the available options; see README.md for defaults,
examples, and notes on options whose current behavior differs from their intent.
*/

use std::path::{Path, PathBuf};
use std::time::Instant;
use yansi::Paint;

mod utils;
mod opts;
mod img;
mod walk;
mod img_ext;

use crate::opts::{Opts, Metrics};
use crate::img::process_image;
use crate::walk::walk_path;


fn main() {
   
   // The defaults!
    let mut inpath_str = "".to_string();

    let mut outpath_str =
    if cfg!(target_family = "windows") {
        "srcset".to_string()
    } else {
        "/tmp/srcset/".to_string()
    };

    
    
    let mut is_file = false;
    let mut extension = "".to_string();
    let mut prefix = "".to_string();

    let mut sizes = "320, 480, 640, 768, 960, 1024, 1366, 1600, 1920".to_string();

    let mut is_recurse = false;
    let mut is_jobs = false;
    let mut is_nested = false;
    let mut is_test = false;
    let mut is_verbose = false;
    let mut is_quiet = false;
    let mut min_kb = 100;
    let mut quality = 82;
    let mut unsharpen = "0.25,8".to_string();

    let mut is_tagfile = true;
    let mut use_largest = true;

    let mut is_version: bool = false;

    {
        let mut args = argparse::ArgumentParser::new();

        const DESCRIPTION_STRING: &'static str =
        concat!("srcset command-line utility v", env!("CARGO_PKG_VERSION"));        

        args.set_description(DESCRIPTION_STRING);

        args.refer(&mut outpath_str)
                .add_option(&["-o", "--out"], argparse::Store,
                "Output directory (default: /tmp/srcset/ on Unix, srcset on Windows)");

        args.refer(&mut is_recurse)
                .add_option(&["-r", "--recurse"], argparse::StoreTrue,
                "Recurse directories");

        args.refer(&mut extension)
                .add_option(&["-t", "--type"], argparse::Store,
                "Output filetype (jpg, png, etc)");

        args.refer(&mut prefix)
                .add_option(&["-p", "--prefix"], argparse::Store,
                "Prefix added to the srcset tag");

        args.refer(&mut sizes)
                .add_option(&["-s", "--sizes"], argparse::Store,
                "The sizes for responsive images: defaults to \"320, 480, 640, 768, 960, 1024, 1366, 1600, 1920\"");

        args.refer(&mut min_kb)
                .add_option(&["-m", "--min"], argparse::Store,
                "Minimum file size in KiB for directory processing (default: 100)");

        args.refer(&mut is_jobs)
                .add_option(&["-j", "--job"], argparse::StoreTrue,
                "Use parallel jobs");

        args.refer(&mut is_nested)
                .add_option(&["-n", "--nested"], argparse::StoreTrue,
                "Images are saved in a nested hierarchy");

        args.refer(&mut is_test)
                .add_option(&["-z", "--test"], argparse::StoreTrue,
                "Test run. Images are found but not created");

        args.refer(&mut is_verbose)
                .add_option(&["-v", "--verbose"], argparse::StoreTrue,
                "Verbose output");

        args.refer(&mut is_quiet)
                .add_option(&["-e", "--quiet"], argparse::StoreTrue,
                "Quiet errors");

        args.refer(&mut inpath_str)
                .add_argument("file", argparse::Store,
                "Path (Filename or directory) of image");

        args.refer(&mut quality)
                .add_option(&["-q", "--quality"], argparse::Store,
                "Quality with a value in the range 1-100 where 100 is the best; default is 82");

        args.refer(&mut unsharpen)
                .add_option(&["-u", "--unsharpen"], argparse::Store,
                "Unsharpen parameters (default: 0.25,8); currently has no effect");

        args.refer(&mut is_tagfile)
                .add_option(&["-d", "--notag"], argparse::StoreTrue,
                "Intended to disable tag files; currently has no effect");

        args.refer(&mut use_largest)
                .add_option(&["-l", "--largest"], argparse::StoreTrue,
                "Scale to the largest configured size (already the default)");

        args.refer(&mut is_version)
                .add_option(&["--version"], argparse::StoreTrue,
                "Print version and exit");

        args.parse_args_or_exit();
    }
    

    if is_version {
        println!("srcset {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(1);        
    }

    // Output must end in `/` so simply append one.
    if !outpath_str.ends_with("/") {  outpath_str.push_str("/"); }

    // Seems like a whitespace creeps in?
    extension = extension.trim().to_string();

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

    // Convert size string by comma
    let split = sizes.split(",");
    let mut vec: Vec<u32> = vec![];
    for s in split {
        let x = s.trim_matches(' ');
        vec.push(x.parse::<u32>().unwrap() );
    }

    let sigmathresh: Vec<&str> = unsharpen.split(",").collect();
    let sigma = sigmathresh[0].parse::<f32>().unwrap();
    let thresh = sigmathresh[1].parse::<i32>().unwrap();


    let opts = Opts{inpath: inpath, outpath: outpath, 
                    is_file: is_file, extension: extension, 
                    prefix: prefix, min_size: min_kb * 1024, 
                    is_recurse: is_recurse, is_jobs: is_jobs, is_nested: is_nested, 
                    is_test: is_test, is_dir: true, is_verbose: is_verbose, 
                    is_quiet: is_quiet, sizes: vec, quality: quality,
                    sigma: sigma, thresh: thresh, is_tagfile: is_tagfile,
                    use_largest: use_largest};
    
    let mut m = Metrics{count: 0, resized: 0, traversed: 0, skipped: 0 };

    let inpath = Path::new(&inpath_str);

    let start = Instant::now();
    let _result =
        match inpath.is_dir()
        {
            true => walk_path(&inpath, &opts, &mut m),
            _ => {
                match process_image(&inpath, &opts, &mut m) {
                    Err(e) => { if !opts.is_quiet{eprintln!("{} {:?}, {:?}", Paint::red("WARNING: Processing error: "), inpath, e)}},
                    _ => (),           
                }
                Ok(())
                },                              
           
//            process_image(&inpath, &opts, &mut m),
            
        };
    let duration = start.elapsed();
    
    println!("Count: {}, Resized: {}, Traversed: {}, Skipped {} ", Paint::green(m.count), Paint::yellow(m.resized), Paint::blue(m.traversed), Paint::red(m.skipped));
    println!("{:?}", Paint::green(duration));
}

