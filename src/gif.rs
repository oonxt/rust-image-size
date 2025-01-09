use crate::length::Length;
use crate::{ImageReader, Result};
use binrw::BinRead;
use std::io::{BufRead, Seek};

#[derive(Debug, BinRead)]
#[br(little)]
#[br(magic(b"GIF"))]
pub struct Gif {
    r#type: Type,
    pub width: u16,
    pub height: u16,
}
#[derive(Debug, BinRead)]
#[br(little)]
pub enum Type {
    #[br(magic(b"87a"))]
    Gif87a,
    #[br(magic(b"89a"))]
    Gif89a,
}

impl Gif {
    pub fn new<R: BufRead + Seek>(reader: &mut R) -> Result<Self> {
        Ok(Gif::read(reader)?)
    }
}

impl ImageReader for Gif {
    fn dimension(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn x_dpi(&self) -> u32 {
        72
    }

    fn y_dpi(&self) -> u32 {
        72
    }
}
