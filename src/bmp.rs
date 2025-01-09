use std::io::{BufRead, Read, Seek};
use binrw::BinRead;
use crate::ImageReader;
use std::io::SeekFrom;
use crate::length::Length;

const PPM_FACTOR: f32 = 0.0254;

#[derive(BinRead, Debug)]
#[br(little)]
#[br(magic(b"BM"))]
pub struct Bmp {
    #[br(seek_before = SeekFrom::Start(0x12))]
    pub width: u32,
    pub height: u32,

    #[br(seek_before = SeekFrom::Start(0x26))]
    x_ppm: u32,
    y_ppm: u32,
}

impl Bmp {
    pub fn new<R: BufRead + Seek>(reader: &mut R) -> Self {
        Bmp::read(reader).unwrap()
    }
}

impl ImageReader for Bmp {

    fn dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn x_dpi(&self) -> u32 {
        _dpi(self.x_ppm)
    }

    fn y_dpi(&self) -> u32 {
        _dpi(self.y_ppm)
    }
}


fn _dpi(ppm: u32) -> u32 {
    if ppm == 0 {
        return 96;
    }
    (ppm as f32 * PPM_FACTOR) as u32
}