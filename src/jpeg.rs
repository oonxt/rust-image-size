use crate::tiff::{IfdEntry, Tiff};
use crate::ImageReader;
use binrw::helpers::{until, until_eof};
use binrw::{BinRead, BinReaderExt};
use std::cmp::PartialEq;
use std::io::{BufRead, Cursor, Seek};

#[derive(BinRead, Debug)]
#[br(big)]
pub struct Jpeg {
    #[br(ignore)]
    pub width: u32,
    #[br(ignore)]
    pub height: u32,
    #[br(ignore)]
    pub x_dpi: u32,
    #[br(ignore)]
    pub y_dpi: u32,
    #[br(parse_with = until(|seg: &Segment| seg.is_sos()))]
    segments: Vec<Segment>,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub enum Segment {
    #[br(magic = 0xFFD8u16)]
    SOI,
    #[br(magic(0xFFE0u16))]
    App0(App0),
    #[br(magic(0xFFE1u16))]
    App1(App1),
    #[br(magic(0xFFD9u16))]
    EOI,
    #[br(magic(0xFFDAu16))]
    SOS(Sos),
    SOF(Sof),
    Other(OtherSegment),
}

impl Segment {
    pub fn is_sos(&self) -> bool {
        match self {
            Segment::SOS(_) => true,
            _ => false,
        }
    }
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct OtherSegment {
    pub marker: Marker,
    pub length: u16,
    #[br(count = length - 2)]
    pub data: Vec<u8>,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct Sof {
    pub marker: SofMarker,
    pub length: u16,
    pub precision: u8,
    pub height: u16,
    pub width: u16,
    pub num_components: u8,
    #[br(count = num_components)]
    pub components: Vec<SofComponent>,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct SofComponent {
    pub component_id: u8,
    pub sampling_factors: u8, // [horizontal_sampling_factor, vertical_sampling_factor]
    pub quant_table_selector: u8,
}
#[derive(BinRead, Debug)]
#[br(big)]
pub struct Sos {
    pub length: u16,
    pub num_components: u8,
    #[br(count = num_components)]
    pub components: Vec<SosComponent>,
    pub spectral_selection: u8,
    pub successive_high: u8,
    pub successive_low: u8,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct SosComponent {
    pub component_id: u8,
    pub huffman_table_selector: u8,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct App0 {
    pub length: u16,
    pub identifier: [u8; 5], // 'JFIF\0'
    pub version_major: u8,
    pub version_minor: u8,
    pub units: u8,
    pub x_density: u16,
    pub y_density: u16,
    pub thumbnail_x_size: u8,
    pub thumbnail_y_size: u8,
    #[br(count = thumbnail_x_size * thumbnail_y_size * 3)]
    pub thumbnail_data: Vec<u8>,
}

impl App0 {
    fn _dpi(&self, density: u32) -> u32 {
        if self.units == 1 {
            density
        } else if self.units == 2 {
            (density as f32 * 2.54).round() as u32
        } else {
            72
        }
    }
    pub fn x_dpi(&self) -> u32 {
        self._dpi(self.x_density as u32)
    }
    pub fn y_dpi(&self) -> u32 {
        self._dpi(self.y_density as u32)
    }
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct App1 {
    pub length: u16,
    #[br(magic(b"Exif\0\0"))]
    #[br(count = length - 8)]
    pub data: Vec<u8>,
}

impl App1 {
    pub fn tiff(&self) -> crate::Result<Tiff> {
        let mut cursor = Cursor::new(&self.data);
        Ok(Tiff::new(&mut cursor)?)
    }
}

#[derive(Debug, PartialEq, BinRead)]
#[br(big)]
pub enum SofMarker {
    #[br(magic(0xFFC0u16))]
    SOF0,
    #[br(magic(0xFFC1u16))]
    SOF1,
    #[br(magic(0xFFC2u16))]
    SOF2,
    #[br(magic(0xFFC3u16))]
    SOF3,
    #[br(magic(0xFFC5u16))]
    SOF5,
    #[br(magic(0xFFC6u16))]
    SOF6,
    #[br(magic(0xFFC7u16))]
    SOF7,
    #[br(magic(0xFFC9u16))]
    SOF9,
    #[br(magic(0xFFCAu16))]
    SOFA,
    #[br(magic(0xFFCBu16))]
    SOFB,
    #[br(magic(0xFFCDu16))]
    SOFD,
    #[br(magic(0xFFCEu16))]
    SOFE,
    #[br(magic(0xFFCFu16))]
    SOFF,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(big)]
pub enum Marker {
    #[br(magic(0xFF01u16))]
    TEM,
    #[br(magic(0xFFC4u16))]
    DHT,
    #[br(magic(0xFFCCu16))]
    DAC,
    #[br(magic(0xFFC8u16))]
    JPG,

    #[br(magic(0xFFC0u16))]
    SOF0,
    #[br(magic(0xFFC1u16))]
    SOF1,
    #[br(magic(0xFFC2u16))]
    SOF2,
    #[br(magic(0xFFC3u16))]
    SOF3,
    #[br(magic(0xFFC5u16))]
    SOF5,
    #[br(magic(0xFFC6u16))]
    SOF6,
    #[br(magic(0xFFC7u16))]
    SOF7,
    #[br(magic(0xFFC9u16))]
    SOF9,
    #[br(magic(0xFFCAu16))]
    SOFA,
    #[br(magic(0xFFCBu16))]
    SOFB,
    #[br(magic(0xFFCDu16))]
    SOFD,
    #[br(magic(0xFFCEu16))]
    SOFE,
    #[br(magic(0xFFCFu16))]
    SOFF,

    #[br(magic(0xFFD0u16))]
    RST0,
    #[br(magic(0xFFD1u16))]
    RST1,
    #[br(magic(0xFFD2u16))]
    RST2,
    #[br(magic(0xFFD3u16))]
    RST3,
    #[br(magic(0xFFD4u16))]
    RST4,
    #[br(magic(0xFFD5u16))]
    RST5,
    #[br(magic(0xFFD6u16))]
    RST6,
    #[br(magic(0xFFD7u16))]
    RST7,

    #[br(magic(0xFFD8u16))]
    SOI,
    #[br(magic(0xFFD9u16))]
    EOI,
    #[br(magic(0xFFDAu16))]
    SOS,
    #[br(magic(0xFFDBu16))]
    DQT,
    #[br(magic(0xFFDCu16))]
    DNL,
    #[br(magic(0xFFDDu16))]
    DRI,
    #[br(magic(0xFFDEu16))]
    DHP,
    #[br(magic(0xFFDFu16))]
    EXP,

    #[br(magic(0xFFE0u16))]
    APP0,
    #[br(magic(0xFFE1u16))]
    APP1,
    #[br(magic(0xFFE2u16))]
    APP2,
    #[br(magic(0xFFE3u16))]
    APP3,
    #[br(magic(0xFFE4u16))]
    APP4,
    #[br(magic(0xFFE5u16))]
    APP5,
    #[br(magic(0xFFE6u16))]
    APP6,
    #[br(magic(0xFFE7u16))]
    APP7,
    #[br(magic(0xFFE8u16))]
    APP8,
    #[br(magic(0xFFE9u16))]
    APP9,
    #[br(magic(0xFFEAu16))]
    APPA,
    #[br(magic(0xFFEBu16))]
    APPB,
    #[br(magic(0xFFECu16))]
    APPC,
    #[br(magic(0xFFEDu16))]
    APPD,
    #[br(magic(0xFFEEu16))]
    APPE,
    #[br(magic(0xFFEFu16))]
    APPF,

    #[br(magic(0xFFFEu16))]
    COM,

    Other(u16),
}

impl Jpeg {
    pub fn new<R: std::io::BufRead + std::io::Seek>(reader: &mut R) -> crate::Result<Self> {
        let mut jpeg = Jpeg::read(reader)?;

        for seg in &jpeg.segments {
            if let Segment::App0(app0) = seg {
                jpeg.x_dpi = app0.x_dpi();
                jpeg.y_dpi = app0.y_dpi();
            } else if let Segment::App1(app1) = seg {
                let tiff = app1.tiff()?;
                jpeg.x_dpi = tiff.x_dpi();
                jpeg.y_dpi = tiff.x_dpi();
            } else if let Segment::SOF(sof) = seg {
                jpeg.width = sof.width as u32;
                jpeg.height = sof.height as u32;
            }
        }

        Ok(jpeg)
    }
}

impl crate::ImageReader for Jpeg {
    fn dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn x_dpi(&self) -> u32 {
        self.x_dpi
    }

    fn y_dpi(&self) -> u32 {
        self.y_dpi
    }
}
