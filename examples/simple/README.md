
## Simple Example


Convert the provided Library of Congress [Free To Use](https://www.loc.gov/free-to-use/)  images using srcset 

##### A verbose dry run using the test `-z` directive on an example `tif` image, converting to `jpg` format:

`srcset examples/simple/pnp-bellcm-10100-10106u.tif -t jpg -vz`

##### A verbose dry run using the `-z` directive on the second example `tif` image, converting to `png` format:

`srcset examples/simple/pnp-fsa-8c02000-8c02400u -t png -vz`

##### Convert both images to `jpg` using the recurse `-r` option and dump the images in the `/var/www/images` directory.

```
srcset examples/simple -r -t jpg -o /var/www/images -v
```


