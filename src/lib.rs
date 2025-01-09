pub mod length;
pub mod bmp;
pub mod gif;
pub mod png;
pub mod jpeg;
pub mod tiff;

use std::io::{BufRead, Read, Seek};
use crate::length::Length;

pub enum MimeType {
    Bmp,
    Gif,
    Jpeg,
    Png,
    Tiff
}

impl MimeType {
    pub fn from_ext(ext: &str) -> Option<MimeType> {
        match ext {
            "bmp" => Some(MimeType::Bmp),
            "gif" => Some(MimeType::Gif),
            "jpeg" | "jpg" => Some(MimeType::Jpeg),
            "png" => Some(MimeType::Png),
            "tiff" | "tif" => Some(MimeType::Tiff),
            _ => None
        }
    }

    pub fn from_content_type(content_type: &str) -> Option<MimeType> {
        match content_type {
            "image/bmp" => Some(MimeType::Bmp),
            "image/gif" => Some(MimeType::Gif),
            "image/jpeg" => Some(MimeType::Jpeg),
            "image/png" => Some(MimeType::Png),
            "image/tiff" => Some(MimeType::Tiff),
            _ => None
        }
    }

    pub fn content_type(&self) -> &'static str {
        match self {
            MimeType::Bmp => "image/bmp",
            MimeType::Gif => "image/gif",
            MimeType::Jpeg => "image/jpeg",
            MimeType::Png => "image/png",
            MimeType::Tiff => "image/tiff"
        }
    }

    pub fn default_ext(&self) -> &'static str {
        match self {
            MimeType::Bmp => "bmp",
            MimeType::Gif => "gif",
            MimeType::Jpeg => "jpg",
            MimeType::Png => "png",
            MimeType::Tiff => "tiff"
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
