
//! An extension trait of DynamicImage simply to permit passing quality into encoder

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use image::ColorType;
use image::error::{ImageError, ImageFormatHint, ImageResult};
use image::{GenericImageView, ImageFormat, ImageEncoder};

#[cfg(feature = "bmp")]
use image::codecs::bmp;
#[cfg(feature = "gif")]
use image::codecs::gif;
#[cfg(feature = "hdr")]
use image::codecs::hdr;
#[cfg(feature = "ico")]
use image::codecs::ico;
#[cfg(feature = "jpeg")]
use image::codecs::jpeg;
#[cfg(feature = "png")]
use image::codecs::png;
#[cfg(feature = "pnm")]
use image::codecs::pnm;
#[cfg(feature = "tga")]
use image::codecs::tga;
#[cfg(feature = "dds")]
use image::codecs::dds;
#[cfg(feature = "tiff")]
use image::codecs::tiff;
#[cfg(feature = "webp")]
use image::codecs::webp;
#[cfg(feature = "farbfeld")]
use image::codecs::farbfeld;
#[cfg(any(feature = "avif-encoder", feature = "avif-decoder"))]
use image::codecs::avif;


pub trait ImgExt {
    fn save_with_quality<Q>(&self, path: Q, quality: u8) -> ImageResult<()>
    where
        Q: AsRef<Path>;
}


impl ImgExt for image::DynamicImage {
   
    /// Saves the buffer to a file at the path specified.
    ///
    /// The image format is derived from the file extension.
    /// Currently only jpeg and png files are supported.
    fn save_with_quality<Q>(&self, path: Q, quality: u8) -> ImageResult<()>
    where
        Q: AsRef<Path>,
    {
        let color = self.color();
        // This is valid as the subpixel is u8.
        save_buffer_with_quality(
            path.as_ref(),
            self.as_bytes(),
            self.width(),
            self.height(),
            color,
            quality
        )
    }
}




#[allow(unused_variables)]
// Most variables when no features are supported
pub fn save_buffer_with_quality(
    path: &Path,
    buf: &[u8],
    width: u32,
    height: u32,
    color: ColorType,
    quality: u8
) -> ImageResult<()> {
    let fout = &mut BufWriter::new(File::create(path)?);
    let format =  ImageFormat::from_path(path)?;
    save_buffer_with_format_quality(path, buf, width, height, color, format, quality)
}


#[allow(unused_variables)]
pub fn save_buffer_with_format_quality(
    path: &Path,
    buf: &[u8],
    width: u32,
    height: u32,
    color: ColorType,
    format: ImageFormat,
    quality: u8
) -> ImageResult<()> {
    let fout = &mut BufWriter::new(File::create(path)?);

    match format {
       #[cfg(feature = "gif")]
        image::ImageFormat::Gif => gif::GifEncoder::new(fout).encode(buf, width, height, color),
        #[cfg(feature = "ico")]
        image::ImageFormat::Ico => ico::IcoEncoder::new(fout).write_image(buf, width, height, color),
        #[cfg(feature = "jpeg")]
        image::ImageFormat::Jpeg => jpeg::JpegEncoder::new_with_quality(fout, quality).write_image(buf, width, height, color),
        #[cfg(feature = "png")]
        image::ImageFormat::Png => png::PngEncoder::new(fout).write_image(buf, width, height, color),
        #[cfg(feature = "pnm")]
        image::ImageFormat::Pnm => {
            let ext = path.extension()
            .and_then(|s| s.to_str())
            .map_or("".to_string(), |s| s.to_ascii_lowercase());
            match &*ext {
                "pbm" => pnm::PnmEncoder::new(fout)
                    .with_subtype(pnm::PNMSubtype::Bitmap(pnm::SampleEncoding::Binary))
                    .write_image(buf, width, height, color),
                "pgm" => pnm::PnmEncoder::new(fout)
                    .with_subtype(pnm::PNMSubtype::Graymap(pnm::SampleEncoding::Binary))
                    .write_image(buf, width, height, color),
                "ppm" => pnm::PnmEncoder::new(fout)
                    .with_subtype(pnm::PNMSubtype::Pixmap(pnm::SampleEncoding::Binary))
                    .write_image(buf, width, height, color),
                "pam" => pnm::PnmEncoder::new(fout).write_image(buf, width, height, color),
                _ => Err(ImageError::Unsupported(ImageFormatHint::Exact(format).into())), // Unsupported Pnm subtype.
            }
        },
        #[cfg(feature = "farbfeld")]
        image::ImageFormat::Farbfeld => farbfeld::FarbfeldEncoder::new(fout).write_image(buf, width, height, color),        
        #[cfg(feature = "avif-encoder")]
        image::ImageFormat::Avif => avif::AvifEncoder::new(fout).write_image(buf, width, height, color),
        //#[cfg(feature = "hdr")]
        //image::ImageFormat::Hdr => hdr::HdrEncoder::new(fout).encode(&[Rgb<f32>], width, height), // usize
        #[cfg(feature = "bmp")]
        image::ImageFormat::Bmp => bmp::BmpEncoder::new(fout).write_image(buf, width, height, color),
        #[cfg(feature = "tiff")]
        image::ImageFormat::Tiff => tiff::TiffEncoder::new(fout)
            .write_image(buf, width, height, color),
       #[cfg(feature = "tga")]
        image::ImageFormat::Tga => tga::TgaEncoder::new(fout).write_image(buf, width, height, color),

        format => Err(ImageError::Unsupported(ImageFormatHint::Exact(format).into())),
    }
}
