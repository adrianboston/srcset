
/*!

# NAME

**srcset** -- generate multiple responsive images for web and mobile.

## SUMMARY

The srcset utility generates multiple (eight) scaled versions of an image at particular breakpoints
those of 320,480,640,768,960,1024,1280,1440 pixels wide, the widths match common Mobile and widescreen viewports.

It convert images from jpg, png, tiff formats; and outputs the needed <img> tag.

## SYNOPSIS

`./srset [-rjnvzh] [—t type] [-s sizes] [-o outpath] filename`

`./srset [-rjnvzh] [—t type] [-s sizes] [-o outpath] file hierarchy`

## DESCRIPTION

A file path, whether filename or file hierarcy is required. Specify the path (file or file hierarchy) to generate images. The type of file path, whether file or file hierarchy is determined by srcset.

The options are as follows:

-r --recurse **recurse** the provided path. ignored for single file.

-o --out an **output** directory for the resized image. defaults to /`tmp/srcset/`

-t --type the **type** of image conversion (png, jpg, ... ); defaults to the same type as the original image found in the input path.

-m --min the **minimum** size of image that will be processed; otherwise an image will be skipped. Ignored for single files. Specifed in Kb. The default is `100`

-s --size the sizes tag used in the **srcset** image tag. defaults to `480,`

-j --jobs whether to use parallel or threaded **jobs** on image conversion.

-n --nest use a **nested** directory hierarchy on the output, otherwise it is flat. ignored for single file.

-p --prefix add a string prefix to the filenames within the <srcset/> tag.

-z --test run a test or **null** run. File paths are traversed but no images are generated and no new file path is created. The `<img>` markup will be generated to the console.

-v --verbose use verbose output.

-q  --quiet **quiet** the errors. much the same as piping error to null, `2>/dev/null`

-h   display the help.

## USE

`srcset` is built using Rust known for its speed plus it leverages modern multi-core architectures. Use the `-j` directive to turn on parallel jobs.

`srcset` requires a file path, whether filename or file hierarcy. If a filename, that single file will resized. If a file hierarchy, the files within that directory will be resized. Specifying the option `r` it will walk the file hierarchy resizing any found images.

The utility resizes each image using the same compression as the original image; however, specify the desination type using the `-t` directive. *srcset* permits the use of an image in TIFF format -- that is often the second step after Nikon, Canon and other 'raw' native formats -- from which `convert` can generate the final HTML-ready images. Or you can stick with the tried JPEG, PNG and GIF.

Due to the large number of resized images, they are organized into a file structure. The name of the directory matches the original filename. The name of each resized image contains the width of the image and placed into the directory from `320w` to `1440w`. The original file is copied, placed into the directory and renamed to `legacy`. Therefore, `srcset` will skip over any files named `legacy`, `320w`, `480w`,.... `1440w` to avoid duplicate work.

For example, given an image named `my_image` the following directory will be constructed.

```
srcset my_image.jpg

- my_image/
        legacy.jpg
        320w.jpg
        480w.jpg
        640w.jpg
        768w.jpg
        960w.jpg
        1024w.jpg
        1280w.jpg
        1440w.jpg
```

The resulting tag is:

```
<img src="my_image/legacy.jpg" srcset="my_image/320w.jpg 320w, my_image/480w.jpg 480w, my_image/640w.jpg 640w, my_image/768w.jpg 768w, my_image/960w.jpg 960w, my_image/1024w.jpg 1024w, my_image/1280w.jpg 1280w, my_image/1440w.png 1440w" sizes="(min-width: 768px) 50vw, 100vw" alt="A file named my_image">
```

*/

mod utils;
mod opts;
mod img;
mod walk;

use std::path::{Path, PathBuf};
use std::time::Instant;
use yansi::Paint;

use crate::opts::{Opts, Metrics};
use crate::img::process_image;
use crate::walk::walk_path;


fn main() {
   
   // The defaults!
    let mut inpath_str = "".to_string();
    let mut outpath_str = "/tmp/srcset/".to_string();
    
    let mut is_file = false;
    let mut extension = "".to_string();
    let mut prefix = "".to_string();

    let mut srcsizes = "480, 640, 768, 960, 1024, 1366, 1600, 1920".to_string();

    let mut is_recurse = false;
    let mut is_jobs = false;
    let mut is_nested = false;
    let mut is_test = false;
    let mut is_verbose = false;
    let mut is_quiet = false;
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

        args.refer(&mut prefix)
                .add_option(&["-p", "--prefix"], argparse::Store,
                "Prefix added to the srcset tag, such as webroot");

        args.refer(&mut srcsizes)
                .add_option(&["-s", "--sizes"], argparse::Store,
                "The responsive sizes as csv: defaults to \"480, 640, 768, 960, 1024, 1366, 1600, 1920\"");

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

        args.refer(&mut is_quiet)
                .add_option(&["-q", "--quiet"], argparse::StoreTrue,
                "Quiet errors");

        args.refer(&mut inpath_str)
                .add_argument("file", argparse::Store,
                "Path (Filename or directory) of image");


        args.parse_args_or_exit();
    }
    
    // Output must end in `/` so simply append one.
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

    // Convert size string by comma
    let split = srcsizes.split(",");
    let mut vec: Vec<u32> = vec![];
    for s in split {
        let x = s.trim_matches(' ');
        vec.push(x.parse::<u32>().unwrap() );
    }

    let opts = Opts{inpath: inpath, outpath: outpath, 
                    is_file: is_file, extension: extension, 
                    prefix: prefix, min_size: min_kb * 1024, 
                    is_recurse: is_recurse, is_jobs: is_jobs, is_nested: is_nested, 
                    is_test: is_test, is_dir: true, is_verbose: is_verbose, 
                    is_quiet: is_quiet, srcsizes: vec};
    
    let mut m = Metrics{count: 0, resized: 0, traversed: 0, skipped: 0 };

    let inpath = Path::new(&inpath_str);

    let start = Instant::now();
    let _result =
        match inpath.is_dir()
        {
            true => walk_path(&inpath, &opts, &mut m),
            _ => process_image(&inpath, &opts, &mut m),
        };
    let duration = start.elapsed();
    
    println!("Count: {}, Resized: {}, Traversed: {}, Skipped {} ", Paint::green(m.count), Paint::yellow(m.resized), Paint::blue(m.traversed), Paint::red(m.skipped));
    println!("{:?}", Paint::green(duration));
}

