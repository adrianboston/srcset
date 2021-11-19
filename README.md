# NAME

**srcset** -- generate multiple responsive images for web and mobile.

## SYNOPSIS

`./srset [-hmz] [-q quality] [—t type] [-l legacysize] [-s sizes] [-o out path] [-f filename] filename`
`./srset [-hmz] [—n findname] [-q quality] [—t type] [-l legacysize] [-s sizes] [-o out path] [-f file hierarchy] file hierarchy`

## DESCRIPTION

The srcset utility generates multiple (eight) scaled versions of an image at particular breakpoints -- 320,480,640,768,960,1024,1280,1440 pixels wide -- that match common Mobile and widescreen viewports using Imagemagick's `convert` utility and outputs the needed `<img>` tag.

A file path, whether filename or file hierarcy is required. The options are as follows:

-i  specify the input path (single file or file hierarchy) for **srcset** to convert. The type of file path, whether file or file hierarchy is determined by **srcset**.

-o  specify a destination directory for the files converted by **srset.sh**. Otherwise the files are saved to the directory of the specified input file path.

-t   specify the type of image conversion for new images used by **srset**; defaults to the same type as the original image found in the input path.

-z   a flag with no argument directing **srcset** to run a test or dry run. File paths are traversed but no images are generated and no new file path is created. The `<img>` markup will be generated to the console, a `-m` directive will be ignored.

-h   display the help.

## The problem

Generating multiple responsive images using Photoshop, Lightroom or other GUI application is an irksome and error-prone task. Further, the needed `<img>` tag referencing multiple images in the `srcset` attribute is long and tedious to generate. On the other hand, the sweet script *srcset.sh* is a generator that can be be easily added into a automated build workflow. And that long `<img>` tag with the full set of `srcset` images is the standard output which can then be dropped into the target html file(s).

In addition and of interest, *srcset* permits the use of an image in its largest and highest resolution format TIFF format -- that is often the second step after Nikon, Canon and other 'raw' native formats -- from which `convert` can generate the final HTML-ready images. Or you can stick with the tried JPEG, PNG and GIF.

## Background

Images are important UI/UX aspects bu
t usually the largest payload of a web site or page. As it turns out, speed is User Experience too. Google suggests that a web page load in under 3 seconds or users will abandon the site. With Mobile the situation is aggravated: the connection is slower and expensive; users are even more likely to not bother waiting.

In comes the HTML5 `srcset` attribute to help, whether Mobile or desktop Web. The html `<img>` tag takes an optional set of images that should be scaled versions of the original. The Mobile or Web browser selects an image given its current width and resolution capabilities. 'srcset' recommends images that don't waste expensive Mobile bandwidth yet provide a image suitable for the device's resolution. In desktops the browser will select an image based on its current width (opposed to the device's width). In other words, the `srcset` attribute permits the use of an image that is not too big yet not too small. The `srcset` attribute is ignored and `src` is used in legacy browsers.

In order to speed up the web further it is suggested that images are compressed. There is no hard recommendation; `convert` uses `92` if it cannot determine a best fit. That runs high on the side of a image quality but low on overall web page download speed; load test a site for a balance between speed and beauty. During conversion *srcset.sh* can interlace the image versions as suggested by webpagetest.org.

### Requirements



### Useful Resources

##### Imagemagick list of formats
- http://imagemagick.sourceforge.net/http/www/formats.html

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
- Stick with release v0.0.5 if your shell lacks getops and then use the following for processing directories noting the use of `-a -name` to prevent file recursion

`find . -type f -atime +1s \( -name *.jpg -a ! -name "*-320w.*" -a ! -name "*-480w.*" -a ! -name "*-640w.*" -a ! -name "*-768w.*" -a ! -name "*-960w.*" -a ! -name "*-1024w.*" -a ! -name "*-1280w.*" -a ! -name "*-1440w.*" ! -name "*-srcw.*" \) -exec ./srcset.sh {} \;`
