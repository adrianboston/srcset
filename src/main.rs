
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

-r  --recurse   **recurse** the provided directory. ignored for single file.

-o  --out       The **output** directory for the resized image. defaults to `/tmp/srcset/`; windows its `srcset`

-t  --type      The **type** of image conversion (png, jpg, webp, ... ); defaults to the same type as the original image found in the input path.

-s  --size      The **sizes** for responsive images in comma,separated,value form; defaults to `480, 640, 768, 960, 1024, 1366, 1600, 1920`.

-q --quality    Quality with a value in the range 1-100 where 100 is the best; default is `82`. Only for jpegs.

-u  --unsharpen Unsharpen with a sigma float and threshold int; default is `0.25,8`.

-m  --min       Set the **minimum** size of image that will be processed; otherwise an image will be skipped. Ignored for single files. Specifed in Kilobytes. The default is `100` (aka  a min of `102400` bytes). 

-p --prefix     String prefix to the filenames within the <img/> tag, such as `/var/www/html/pics`.

-j  --jobs      Whether to use parallel threaded **jobs** on image conversion.

-n  --nest      Use a **nested** directory hierarchy on the output, otherwise it is flat. ignored for single file.

-z  --test      Run a test or **null** run. File paths are traversed but no images are generated and no new file path is created. The `<img>` markup will be generated to the console.

-v   --verbose  Use **verbose** output.

-e  --quiet     **quiet** the errors; functionaly the same as piping error to null, `2>/dev/null` 

--version      Display the **versio**.

-h --help       Display the **help**.

## USE

`srcset` is built using Rust known for its speed plus it leverages modern multi-core architectures. Use the `-j` directive to turn on parallel jobs.

`srcset` requires a file path, whether filename or file hierarcy. If a filename, that single file will resized. If a file hierarchy, the files within that directory will be resized. Specifying the option `r` it will walk the file hierarchy resizing any found images.

The utility resizes each image using the same compression as the original image; however, specify the desination type using the `-t` directive. *srcset* permits the use of an image in TIFF format -- that is often the second step after Nikon, Canon and other 'raw' native formats -- from which `convert` can generate the final HTML-ready images. Or you can stick with the tried JPEG, PNG and GIF.

The newly added Webp format is recommended since it offers both lossless and lossy compression in one convenient format. Google claims that its lossless images are 26% smaller that PNGs while its lossy images are 25-34% smaller than JPEGS at the same quality.

 ##  FILE STRUCTURE 
 
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
                "Output directory)");

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
                "Minimum size of image to process in kb, otherwise skip");

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
                "Unsharpen with a sigma float and threshold int; default is 0.25,8");

        args.refer(&mut is_tagfile)
                .add_option(&["-d", "--notag"], argparse::StoreTrue,
                "Dont create a tag file");

        args.refer(&mut use_largest)
                .add_option(&["-l", "--largest"], argparse::StoreTrue,
                "Scale to the largest size");

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

