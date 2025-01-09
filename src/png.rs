use crate::ImageReader;
use binrw::helpers::until_eof;
use binrw::BinRead;
use std::io::{BufRead, Seek};

#[derive(Debug, BinRead)]
#[br(big)]
#[br(magic = b"\x89PNG\x0D\x0A\x1A\x0A")]
pub struct Png {
    #[br(ignore)]
    pub info: Option<Info>,

    #[br(parse_with = until_eof)]
    chunks: Vec<Chunk>,
}
#[derive(Debug, BinRead)]
#[br(big)]
pub enum Chunk {
    IHDR(IHDRChunk),
    IEND(IENDChunk),
    PHYS(PHYSChunk),
    Other(OtherChunk),
}
#[derive(Debug, BinRead)]
#[br(big)]
pub struct IHDRChunk {
    pub length: u32,
    #[br(magic(b"IHDR"))]
    pub width: u32,
    pub height: u32,
    #[br(count = length - 8)]
    data: Vec<u8>,
    pub crc: u32,
}
#[derive(Debug, BinRead)]
#[br(big)]
pub struct IENDChunk {
    pub length: u32,
    #[br(magic(b"IEND"))]
    #[br(count=length)]
    pub data: Vec<u8>,
    pub crc: u32,
}
#[derive(Debug, BinRead)]
#[br(big)]
pub struct PHYSChunk {
    pub length: u32,
    #[br(magic(b"pHYs"))]
    pub x_ppm: u32,
    pub y_ppm: u32,
    pub unit: u32,
    #[br(count=length - 12)]
    pub data: Vec<u8>,
    pub crc: u32,
}

#[derive(Debug, BinRead)]
#[br(big)]
pub struct OtherChunk {
    pub length: u32,
    pub chunk_type: [u8; 4],
    #[br(count=length)]
    pub data: Vec<u8>,
    pub crc: u32,
}

#[derive(Debug, Default)]
pub struct Info {
    pub width: u32,

    pub height: u32,

    pub x_ppu: u32,

    pub y_ppu: u32,

    pub unit: u32,
}

impl Png {
    pub fn new<R: BufRead + Seek>(reader: &mut R) -> crate::Result<Self> {
        let mut png = Png::read(reader)?;
        let mut info = Info::default();

        png.chunks.iter().for_each(|c| {
            if let Chunk::IHDR(chunk) = c {
                info.width = chunk.width;
                info.height = chunk.height;
            } else if let Chunk::PHYS(chunk) = c {
                info.x_ppu = chunk.x_ppm;
                info.y_ppu = chunk.y_ppm;
                info.unit = chunk.unit;
            }
        });
        png.info = Some(info);
        Ok(png)
    }
}

impl ImageReader for Png {
    fn dimension(&self) -> (u32, u32) {
        if let Some(x) = &self.info {
            (x.width, x.height)
        } else {
            (0, 0)
        }
    }

    fn x_dpi(&self) -> u32 {
        if let Some(x) = &self.info {
            _dpi(x.unit, x.x_ppu)
        } else {
            72
        }
    }

    fn y_dpi(&self) -> u32 {
        if let Some(x) = &self.info {
            _dpi(x.unit, x.y_ppu)
        } else {
            72
        }
    }
}

fn _dpi(unit: u32, ppm: u32) -> u32 {
    if unit == 1 {
        (ppm as f32 * 0.254) as u32
    } else {
        72
    }
}
