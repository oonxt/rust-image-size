pub mod bmp;
pub mod gif;
pub mod jpeg;
pub mod length;
pub mod png;
pub mod tiff;

use crate::length::Length;
use std::io::{BufRead, Cursor, Read, Seek};
use thiserror::Error;
use crate::bmp::Bmp;
use crate::gif::Gif;
use crate::jpeg::Jpeg;
use crate::png::Png;
use crate::tiff::Tiff;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Common error: {0}")]
    CommonError(String),
    #[error("Error reading file: {0}")]
    ParseError(#[from] binrw::Error)
}
pub type Result<T> = std::result::Result<T, Error>;
pub enum Image {
    Bmp(Bmp),
    Gif(Gif),
    Jpeg(Jpeg),
    Png(Png),
    Tiff(Tiff),
}

impl Image {

    pub fn from_file(path: &str) -> Result<Image> {
        let ext = path.split('.').last().ok_or(Error::CommonError("文件名不正确".to_string()))?.to_lowercase().as_str();
        let mut reader = Cursor::new(std::fs::read(path)?);
        match ext {
            "bmp" => Ok(Image::Bmp(Bmp::new(&mut reader)?)),
            "gif" => Ok(Image::Gif(Gif::new(&mut reader)?)),
            "jpeg" | "jpg" => Ok(Image::Jpeg(Jpeg::new(&mut reader)?)),
            "png" => Ok(Image::Png(Png::new(&mut reader)?)),
            "tiff" | "tif" => Ok(Image::Tiff(Tiff::new(&mut reader)?)),
            _ => Err(Error::CommonError("不支持的格式".to_string())),
        }
    }

    pub fn content_type(&self) -> &'static str {
        match self {
            Image::Bmp(_) => "image/bmp",
            Image::Gif(_) => "image/gif",
            Image::Jpeg(_) => "image/jpeg",
            Image::Png(_) => "image/png",
            Image::Tiff(_) => "image/tiff",
        }
    }

    pub fn default_ext(&self) -> &'static str {
        match self {
            Image::Bmp(_) => "bmp",
            Image::Gif(_) => "gif",
            Image::Jpeg(_) => "jpg",
            Image::Png(_) => "png",
            Image::Tiff(_) => "tiff",
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Image::Bmp(r) => r.dimension(),
            Image::Gif(r) => r.dimension(),
            Image::Jpeg(r) => r.dimension(),
            Image::Png(r) => r.dimension(),
            Image::Tiff(r) => r.dimension(),
        }
    }

    pub fn width(&self) -> Length {
        match self {
            Image::Bmp(r) => r.width(),
            Image::Gif(r) => r.width(),
            Image::Jpeg(r) => r.width(),
            Image::Png(r) => r.width(),
            Image::Tiff(r) => r.width(),
        }
    }

    pub fn height(&self) -> Length {
        match self {
            Image::Bmp(r) => r.height(),
            Image::Gif(r) => r.height(),
            Image::Jpeg(r) => r.height(),
            Image::Png(r) => r.height(),
            Image::Tiff(r) => r.height(),
        }
    }

    pub fn x_dpi(&self) -> u32 {
        match self {
            Image::Bmp(r) => r.x_dpi(),
            Image::Gif(r) => r.x_dpi(),
            Image::Jpeg(r) => r.x_dpi(),
            Image::Png(r) => r.x_dpi(),
            Image::Tiff(r) => r.x_dpi(),
        }
    }

    pub fn y_dpi(&self) -> u32 {
        match self {
            Image::Bmp(r) => r.y_dpi(),
            Image::Gif(r) => r.y_dpi(),
            Image::Jpeg(r) => r.y_dpi(),
            Image::Png(r) => r.y_dpi(),
            Image::Tiff(r) => r.y_dpi(),
        }
    }
}

pub trait ImageReader {
    fn dimension(&self) -> (u32, u32);

    fn width(&self) -> Length {
        Length::Inches((self.dimension().0 as f32) / (self.x_dpi() as f32))
    }

    fn height(&self) -> Length {
        Length::Inches((self.dimension().1 as f32) / (self.y_dpi() as f32))
    }
    fn x_dpi(&self) -> u32;
    fn y_dpi(&self) -> u32;
}
