# NAME

**srcset** -- generate multiple responsive images for web and mobile.

## SYNOPSIS

`srcset [OPTIONS] <FILE_OR_DIRECTORY>`

## DESCRIPTION

Pass an image file or directory. Directory traversal processes supported images larger than the minimum size; use `-r` to include subdirectories. The default output directory is `/tmp/srcset/` on Unix and `srcset` on Windows.

| Option | Description |
| --- | --- |
| `-o`, `--out PATH` | Output directory. |
| `-r`, `--recurse` | Recurse into subdirectories; ignored for a single file. |
| `-t`, `--type TYPE` | Output format, such as `jpg`, `png`, or `webp`. Defaults to the input format. |
| `-p`, `--prefix PREFIX` | Prefix for image paths in the generated `<img>` tag. |
| `-s`, `--sizes SIZES` | Comma-separated widths; default `320,480,640,768,960,1024,1366,1600,1920`. |
| `-m`, `--min KB` | Minimum file size for directory processing, in KiB; default `100`. Files must be larger than this value. Ignored for a single file. |
| `-j`, `--job` | Process image sizes in parallel. |
| `-n`, `--nested` | Preserve the input directory hierarchy under the output directory; ignored for a single file. |
| `-z`, `--test` | Dry run: report paths and generate the tag without writing images or files. |
| `-v`, `--verbose` | Print additional details. |
| `-e`, `--quiet` | Suppress processing warnings. |
| `-q`, `--quality QUALITY` | JPEG quality, 1–100; default `82`. |
| `-u`, `--unsharpen SIGMA,THRESHOLD` | Set unsharpen parameters; default `0.25,8`. The current implementation discards the sharpened result, so this option has no effect on saved images. |
| `-d`, `--notag` | Intended to disable the tag file, but currently has no effect: tags are printed and written by default. |
| `-l`, `--largest` | Select the largest configured size. This is already the default, so the flag currently has no effect. Images smaller than that width are enlarged. |
| `--version` | Print the version. Currently exits with status 1. |
| `-h`, `--help` | Print help. |

Quote values containing spaces, for example `--sizes "320, 640, 1024"`.

## THE PROBLEM

Generating multiple responsive images using Photoshop, Lightroom or other GUI application is an irksome task even for opposable-thumbed humans. Further, the needed `<img>` tag referencing multiple images in the `srcset` attribute is long and tedious to generate. On the other hand, the *srcset* tool is a generator that can be be easily added into a automated build workflow. And that long `<img>` tag with the full set of `srcset` images is the standard output which can then be dropped into the target html file(s).

## BACKGROUND

Images are important UI/UX aspects but usually the largest payload of a web site or page. As it turns out, speed is User Experience too. Google suggests that a web page load in under 3 seconds or users will abandon the site. Amazon correctly measures this in amount of dollars lost per second. With Mobile the situation is aggravated: the connection is slower and expensive; users are even more likely to abandon the site. As an extreme benchmark, Google.com comes in under 1.5 secs using [Web Page Test](https://www.webpagetest.org/) and costs **$** to view on Mobile. Anything above 10 secs is bad news. Boston Globe's Big Picture takes from 16 to 20 seconds and the cost is **$$$$$** for a Mobile user. ( Tested fom a Virigina EC2 machine). 

In comes the HTML5 [srcset](https://developer.mozilla.org/en-US/docs/Learn/HTML/Multimedia_and_embedding/Responsive_images) attribute to help, whether Mobile or desktop Web. The html `<img>` tag takes an optional set of images that are [scaled versions](https://css-tricks.com/responsive-images-youre-just-changing-resolutions-use-srcset/) of the original. The Mobile or Web browser selects an image given its current width and resolution capabilities. 'srcset' recommends images that don't waste expensive Mobile bandwidth yet provide a image suitable for the device's resolution. In desktops the browser will select an image based on its browser current width (opposed to the device's width).

In summary, the `srcset` attribute permits the use of an image that is not too big yet not too small. The `srcset` attribute is ignored and `src` is used in legacy browsers. Keep in mind that disk - for storing the extra images - is cheap and atemporal while both user's viewing period and network bandwith is measured in units of time.

## AUDIENCE

`srcset` is designed for Web designers, DevOps and Sysops that want to ensure the fastest response time out of ther wesites for their audience. If you read [Google's recommendations](https://developers.google.com/speed/) and [Test, Optimize. Repeat](https://www.webpagetest.org/) for Website request/response time then this utility is for you.  

Of course, it can be used on single files and small directories but it's built to quickly burn through tens, hundreds if not thousands of web images.

## USE

`srcset` is built using Rust known for its speed plus it leverages modern multi-core architectures. Use the `-j` directive to turn on parallel jobs or tasks. 

`srcset` requires a file path, whether filename or file hierarcy. If a filename, that single file will resized. If a file hierarchy, the files within that directory will be resized. Specifying the option `r` it will walk the file hierarchy resizing any found images.

The utility resizes each image using the same compression as the original image; however, specify the desination type using the `-t` directive. *srcset* permits the use of an image in TIFF format -- that is often the second step after Nikon, Canon and other 'raw' native formats -- from which `srcset` can generate the final HTML-ready images. 

## FORMATS 

*srcset* offers the ability to convert images to and from the following formats: JPEG, PNG, TIFF, GIF, WEBP.

The newly added Webp format is recommended since it offers both lossless and lossy compression in one convenient format. Google claims that its lossless images are 26% smaller that PNGs while its lossy images are 25-34% smaller than JPEGS at the same quality.

## FILE STRUCTURE

Due to the large number of resized images, they are organized into a file structure. The name of the directory matches the original filename. The name of each resized image contains the width of the image and placed into the directory from `320w` to `1920w` by default. The original file is resized to the largest configured width (even if that enlarges it), placed into the directory and renamed to `legacy`. Therefore, `srcset` will skip over any files named `legacy`, `320w`,.... `1920w` to avoid duplicate work. 

For clarification, given an image named `my_image` the following directory will be constructed.

```
$ srcset my_image.jpg

- my_image/
        legacy.jpg
        320w.jpg
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
<img src="my_image/legacy.JPG" srcset="my_image/320w.JPG 320w, my_image/480w.JPG 480w, my_image/640w.JPG 640w, my_image/768w.JPG 768w, my_image/960w.JPG 960w, my_image/1024w.JPG 1024w, my_image/1366w.JPG 1366w, my_image/1600w.JPG 1600w. my_image/1920w.JPG 1920w" sizes="(min-width: 768px) 50vw, 100vw" alt="A file named my_image">
```

## ERRORS AND WARNINGS

Note that warnings / errors can be piped into a file. The most common warning is skipping a file due to its small size less than the `-m` directive that is simply a warning. `-e --quiet` will suppress all these warnings. This feature is most useful for ripping through a full directory opposed to burning a few images.

`srcset -r /root 2>srcset.err`

## BUILD, INSTALL, AND TEST

Install Rust and Cargo using the [Rust installation instructions](https://www.rust-lang.org/tools/install), then clone the project and build the optimized executable:

```sh
git clone https://github.com/adrianboston/srcset.git
cd srcset
make
```

The executable is at `target/release/srcset`. `make` uses `cargo build --release`; the optimized build is recommended for image processing.

### Install

On macOS or Linux, install the release executable to `/usr/local/bin/srcset` with:

```sh
sudo make install
```

The install target builds the release executable first and overwrites an existing executable at that path. To use a different installation prefix, set `PREFIX`; for example, `make install PREFIX="$HOME/.local"` installs to `~/.local/bin/srcset`.

### Test

Run the sample image conversions with:

```sh
make test
```

This runs `make test-tmp`, which performs real conversions using the two sample TIFF images. You can also invoke it directly:

```sh
make test-tmp
```

This writes JPEG and PNG images and their `srcset.txt` files under `/tmp/srcset-make-test`. Repeated runs overwrite matching output files. Files in `/tmp` may be removed by the operating system.

Run the Rust test suite separately with `cargo test`.

### Useful Resources

##### The srcset tag and responsive design

- https://developer.mozilla.org/en-US/docs/Learn/HTML/Multimedia_and_embedding/Responsive_images
- https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img
- https://css-tricks.com/responsive-images-youre-just-changing-resolutions-use-srcset/
- https://www.smashingmagazine.com/2015/06/efficient-image-resizing-with-imagemagick/
- https://coderpad.io/blog/development/the-definitive-guide-to-responsive-images-on-the-web/

##### The recommended webp image format

- https://developers.google.com/speed/webp


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
