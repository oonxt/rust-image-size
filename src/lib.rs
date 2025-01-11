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
#[derive(Debug)]
pub struct Image {
    pub crc32: u32,
    pub dimensions: (u32, u32),
    pub width: Length,
    pub height: Length,
    pub x_dpi: u32,
    pub y_dpi: u32,
    pub content_type: String,
    pub ext: String,
}

#[derive(Debug)]
pub enum ImageType {
    Bmp(Bmp),
    Gif(Gif),
    Jpeg(Jpeg),
    Png(Png),
    Tiff(Tiff),
}

impl Image {
    pub fn from_file(path: &str) -> Result<Image> {
        let ext = path.clone().split('.').last().ok_or(Error::CommonError("文件名不正确".to_string()))?.to_lowercase();
        let data = std::fs::read(path)?;
        let crc32 = const_crc32::crc32(&data);
        let mut reader = Cursor::new(data);
        let r#type = match ext.as_str() {
            "bmp" => Ok(ImageType::Bmp(Bmp::new(&mut reader)?)),
            "gif" => Ok(ImageType::Gif(Gif::new(&mut reader)?)),
            "jpeg" | "jpg" => Ok(ImageType::Jpeg(Jpeg::new(&mut reader)?)),
            "png" => Ok(ImageType::Png(Png::new(&mut reader)?)),
            "tiff" | "tif" => Ok(ImageType::Tiff(Tiff::new(&mut reader)?)),
            _ => Err(Error::CommonError("不支持的格式".to_string())),
        }?;

        Ok(Self {
            crc32,
            dimensions: r#type.dimensions(),
            width: r#type.width(),
            height: r#type.height(),
            x_dpi: r#type.x_dpi(),
            y_dpi: r#type.y_dpi(),
            content_type: r#type.content_type().to_string(),
            ext: r#type.default_ext().to_string(),
        })
    }
}

impl ImageType {
    pub fn content_type(&self) -> &'static str {
        match self {
            ImageType::Bmp(_) => "image/bmp",
            ImageType::Gif(_) => "image/gif",
            ImageType::Jpeg(_) => "image/jpeg",
            ImageType::Png(_) => "image/png",
            ImageType::Tiff(_) => "image/tiff",
        }
    }

    pub fn default_ext(&self) -> &'static str {
        match self {
            ImageType::Bmp(_) => "bmp",
            ImageType::Gif(_) => "gif",
            ImageType::Jpeg(_) => "jpeg",
            ImageType::Png(_) => "png",
            ImageType::Tiff(_) => "tiff",
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ImageType::Bmp(r) => r.dimension(),
            ImageType::Gif(r) => r.dimension(),
            ImageType::Jpeg(r) => r.dimension(),
            ImageType::Png(r) => r.dimension(),
            ImageType::Tiff(r) => r.dimension(),
        }
    }

    pub fn width(&self) -> Length {
        match self {
            ImageType::Bmp(r) => r.width(),
            ImageType::Gif(r) => r.width(),
            ImageType::Jpeg(r) => r.width(),
            ImageType::Png(r) => r.width(),
            ImageType::Tiff(r) => r.width(),
        }
    }

    pub fn height(&self) -> Length {
        match self {
            ImageType::Bmp(r) => r.height(),
            ImageType::Gif(r) => r.height(),
            ImageType::Jpeg(r) => r.height(),
            ImageType::Png(r) => r.height(),
            ImageType::Tiff(r) => r.height(),
        }
    }

    pub fn x_dpi(&self) -> u32 {
        match self {
            ImageType::Bmp(r) => r.x_dpi(),
            ImageType::Gif(r) => r.x_dpi(),
            ImageType::Jpeg(r) => r.x_dpi(),
            ImageType::Png(r) => r.x_dpi(),
            ImageType::Tiff(r) => r.x_dpi(),
        }
    }

    pub fn y_dpi(&self) -> u32 {
        match self {
            ImageType::Bmp(r) => r.y_dpi(),
            ImageType::Gif(r) => r.y_dpi(),
            ImageType::Jpeg(r) => r.y_dpi(),
            ImageType::Png(r) => r.y_dpi(),
            ImageType::Tiff(r) => r.y_dpi(),
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
