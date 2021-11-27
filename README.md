# NAME

**srcset** -- generate multiple responsive images for web and mobile.

## SYNOPSIS

`./srset [-rjnzh] [-q quality] [—t type] [-s sizes] [-o out path] filename`

`./srset [-rjnzh] [-q quality] [—t type] [-s sizes] [-o out path] file hierarchy`

## DESCRIPTION

The srcset utility generates multiple (eight) scaled versions of an image at particular breakpoints -- 320,480,640,768,960,1024,1280,1440 pixels wide -- that match common Mobile and widescreen viewports and outputs the needed `<img>` tag.

A file path, whether filename or file hierarcy is required. Specify the path (file or file hierarchy) to generate images. The type of file path, whether file or file hierarchy is determined by **srcset**.

The options are as follows:

-r  **recurse** the provided directory. ignored for single file.

-o  an **output** directory for the resized image. Otherwise the files are saved to the directory of the specified input file path. defaults to /`tmp/srcset/`

-t  the **type** of image conversion (png, jpg, ... ); defaults to the same type as the original image found in the input path.

-s  the sizes tag used in the **srcset** image tag. defaults to `(min-width: 768px) 50vw, 100vw`

-j  whether to use parallel or threaded **jobs** on image conversion.

-n  use a **nested** directory hierarchy on the output.

-z  run a test or dry run. File paths are traversed but no images are generated and no new file path is created. The `<img>` markup will be generated to the console.

-h   display the help.

## The problem

Generating multiple responsive images using Photoshop, Lightroom or other GUI application is an irksome task for opposable-thumbed humans. Further, the needed `<img>` tag referencing multiple images in the `srcset` attribute is long and tedious to generate. On the other hand, the sweet RUSTy *srcset* is a generator that can be be easily added into a automated build workflow. And that long `<img>` tag with the full set of `srcset` images is the standard output which can then be dropped into the target html file(s).

In addition and of interest, *srcset* permits the use of an image in its largest and highest resolution format TIFF format -- that is often the second step after Nikon, Canon and other 'raw' native formats -- from which `convert` can generate the final HTML-ready images. Or you can stick with the tried JPEG, PNG and GIF.

## Background

Images are important UI/UX aspects but usually the largest payload of a web site or page. As it turns out, speed is User Experience too. Google suggests that a web page load in under 3 seconds or users will abandon the site. Amazon correctly measures this in amount of dollars lost per second. With Mobile the situation is aggravated: the connection is slower and expensive; users are even more likely to abandon the site.

In comes the HTML5 `srcset` attribute to help, whether Mobile or desktop Web. The html `<img>` tag takes an optional set of images that should be scaled versions of the original. The Mobile or Web browser selects an image given its current width and resolution capabilities. 'srcset' recommends images that don't waste expensive Mobile bandwidth yet provide a image suitable for the device's resolution. In desktops the browser will select an image based on its current width (opposed to the device's width). In other words, the `srcset` attribute permits the use of an image that is not too big yet not too small. The `srcset` attribute is ignored and `src` is used in legacy browsers.


### Useful Resources

##### The srcset tag and responsive design

- https://developer.mozilla.org/en-US/docs/Learn/HTML/Multimedia_and_embedding/Responsive_images
- https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img
- https://css-tricks.com/responsive-images-youre-just-changing-resolutions-use-srcset/

##### Common screen sizes

- http://mediag.com/news/popular-screen-resolutions-designing-for-all/
- https://mydevice.io/devices/
- https://deviceatlas.com/blog/most-used-smartphone-screen-resolutions-in-2016

##### A word about Speed

- https://developers.google.com/speed/
- Use the following performance tool for measuring your web page speed https://www.webpagetest.org

### NOTES

- Make sure to `chmod u+x srcset.sh` for executable permissions
- Move to a known path such as `/usr/local/bin`
