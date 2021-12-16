# NAME

**srcset** -- generate multiple responsive images for web and mobile.

## SYNOPSIS

`./srset [-rjnvqzh] [—t type] [-s sizes] [-o outpath] filename`

`./srset [-rjnvqzh] [—t type] [-s sizes] [-o outpath] file hierarchy`

## DESCRIPTION

The srcset utility generates multiple (eight) scaled versions of an image at particular breakpoints -- 480,640,768,960,1024,1366,1600 and 1920 pixels wide -- that match common Mobile and widescreen viewports and outputs the needed `<img>` tag.

A file path, whether filename or file hierarcy is required. Specify the path (file or file hierarchy) to generate images. The type of file path, whether file or file hierarchy is determined by **srcset**.

The options are as follows:

-r  --recurse  **recurse** the provided directory. ignored for single file.

-o  --out       the **output** directory for the resized image. defaults to /`tmp/srcset/`

-t  --type      the **type** of image conversion (png, jpg, ... ); defaults to the same type as the original image found in the input path.

-m  --min     set the **minimum** size of image that will be processed; otherwise an image will be skipped. Ignored for single files. Specifed in Kilobytes. The default is `100` (aka  a min of `102400` bytes). 

-s  --size      the **sizes** tag used in the srcset image tag. defaults to `(min-width: 768px) 50vw, 100vw`.

-j  --jobs      whether to use parallel threaded **jobs** on image conversion.

-n  --nest     use a **nested** directory hierarchy on the output, otherwise it is flat. ignored for single file.

-p --prefix   add a string prefix to the filenames within the <img/> tag, such as `/var/www/html/pics`

-z  --test      run a test or **null** run. File paths are traversed but no images are generated and no new file path is created. The `<img>` markup will be generated to the console.

-v   --verbose   use **verbose** output.

-q  --quiet     **quiet** the errors. much the same as piping error to null, `2>/dev/null` 

-h --help        display the **help**.

## BUILD COMPILE

Consider using the Mac OSX DMG for those of you on OSX and without the rust compiler. See the [release section](https://github.com/adrianboston/srcset/releases). Open the DMG and copy the executable to the `/urs/local/bin` directory if desired. 

Otherwise, I've found Rust considerably easier to build from source than C/C++ projects. Dependencies are pulled in very smoothly; it seems to work without any hiccups. 

- Get the Rust compiler. Follow the instructions on https://www.rust-lang.org/tools/install

- Git this project. `git clone https://github.com/adrianboston/srcset.git`

- Open terminal and nagivate to the root directory. `cd srcset`

- Then issue the Rust compiler build command: `cargo build --release`

** Make sure to build using the `--release` option; otherwise, it will be very slow. Consider it similar to gcc optimization level.


## THE PROBLEM

Generating multiple responsive images using Photoshop, Lightroom or other GUI application is an irksome task for opposable-thumbed humans. Further, the needed `<img>` tag referencing multiple images in the `srcset` attribute is long and tedious to generate. On the other hand, the sweet RUSTy *srcset* is a generator that can be be easily added into a automated build workflow. And that long `<img>` tag with the full set of `srcset` images is the standard output which can then be dropped into the target html file(s).

## BACKGROUND

Images are important UI/UX aspects but usually the largest payload of a web site or page. As it turns out, speed is User Experience too. Google suggests that a web page load in under 3 seconds or users will abandon the site. Amazon correctly measures this in amount of dollars lost per second. With Mobile the situation is aggravated: the connection is slower and expensive; users are even more likely to abandon the site.

In comes the HTML5 `srcset` attribute to help, whether Mobile or desktop Web. The html `<img>` tag takes an optional set of images that should be scaled versions of the original. The Mobile or Web browser selects an image given its current width and resolution capabilities. 'srcset' recommends images that don't waste expensive Mobile bandwidth yet provide a image suitable for the device's resolution. In desktops the browser will select an image based on its browser current width (opposed to the device's full width since they are always full screen).

In summary, the `srcset` attribute permits the use of an image that is not too big yet not too small. The `srcset` attribute is ignored and `src` is used in legacy browsers.

## AUDIENCE

`srcset` is designed for power Web designers, DevOps and Sysops that want to ensure the fastest response time out of ther wesites for their audience. If you read [Google's recommendations](https://developers.google.com/speed/) and [Test, Optimize. Repeat](https://www.webpagetest.org/) for Website request/response time then this utility is for you.  

Of course, it can be used on single files and small directories but it's built to quickly burn through tens, hundreds if not thousands of web images. 


## USE

`srcset` is built using Rust known for its speed plus it leverages modern multi-core architectures. Use the `-j` directive to turn on parallel jobs or tasks. 

`srcset` requires a file path, whether filename or file hierarcy. If a filename, that single file will resized. If a file hierarchy, the files within that directory will be resized. Specifying the option `r` it will walk the file hierarchy resizing any found images.

The utility resizes each image using the same compression as the original image; however, specify the desination type using the `-t` directive. *srcset* permits the use of an image in TIFF format -- that is often the second step after Nikon, Canon and other 'raw' native formats -- from which `srcset` can generate the final HTML-ready images. Or you can stick with the tried JPEG, PNG and GIF.

## FILE STRUCTURE

Due to the large number of resized images, they are organized into a file structure. The name of the directory matches the original filename. The name of each resized image contains the width of the image and placed into the directory from `480w` to `1920w`. The original file is resized to the max size (1900w or smaller depending on original width), placed into the directory and renamed to `legacy`. Therefore, `srcset` will skip over any files named `legacy`, `480w`,.... `1920w` to avoid duplicate work. 

For clarification, given an image named `my_image` the following directory will be constructed.

```
$ srcset my_image.jpg

- my_image/
        legacy.jpg
        480w.jpg
        640w.jpg
        768w.jpg
        960w.jpg
        1024w.jpg
        1366w.jpg
        1600w.jpg
        1920w.jpg
```

The resulting tag is:

```
<img src="my_image/legacy.JPG" srcset="my_image/480w.JPG 480w, my_image/640w.JPG 640w, my_image/768w.JPG 768w, my_image/960w.JPG 960w, my_image/1024w.JPG 1024w, my_image/1366w.JPG 1366w, my_image/1600w.JPG 1600w. my_image/1920w.JPG 1920w" sizes="(min-width: 768px) 50vw, 100vw" alt="A file named my_image">
```

## ERRORS AND WARNINGS

Note that warnings / errors can be piped into a file. The most common warning is skipping a file due to its small size less than the `-m` directive that is simply a warning. `-q --quiet` will suppress all these warnings.

Again, this feature is most useful for ripping through a full directory opposed to burning a few images.

`srcset -r /root 2>srcset.err`


### Useful Resources

##### The srcset tag and responsive design

- https://developer.mozilla.org/en-US/docs/Learn/HTML/Multimedia_and_embedding/Responsive_images
- https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img
- https://css-tricks.com/responsive-images-youre-just-changing-resolutions-use-srcset/
- https://www.smashingmagazine.com/2015/06/efficient-image-resizing-with-imagemagick/

##### Common screen sizes

- http://mediag.com/news/popular-screen-resolutions-designing-for-all/
- https://mydevice.io/devices/
- https://deviceatlas.com/blog/most-used-smartphone-screen-resolutions-in-2016

The following one recommends 640,788,1024,1366,1600 and 1920 widths. I added in 960 to fill the gap.
- https://medium.com/hceverything/applying-srcset-choosing-the-right-sizes-for-responsive-images-at-different-breakpoints-a0433450a4a3

##### A word about Speed

- https://developers.google.com/speed/
- Use the following performance tool for measuring your web page speed https://www.webpagetest.org

### NOTES

- Make sure to `chmod u+x srcset.sh` for executable permissions
- Move to a known path such as `/usr/local/bin`
