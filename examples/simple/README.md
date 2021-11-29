
## Simple Example


Convert the provided Library of Congress [Free To Use](https://www.loc.gov/free-to-use/)  images using srcset 

##### A verbose dry run using the test `-z` directive on an example `tif` image, converting to `jpg` format with `-t`:

`srcset examples/simple/pnp-bellcm-10100.tif -t jpg -vz`

##### A verbose dry run using the `-z` directive on the second example `tif` image, converting to `png` format with `-t`:

`srcset examples/simple/pnp-fsa-8c02000 -t png -vz`

##### Recurse over the input file hierarchy using the `-r` option and convert the images into `jpg` format and output the files in the `/var/www/images` directory.

```
srcset examples/simple -r -t jpg -o /var/www/images -v
```


