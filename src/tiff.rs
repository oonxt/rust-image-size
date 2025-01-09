use crate::ImageReader;
use binrw::BinRead;
use std::io::{BufRead, Cursor, Seek};

#[derive(Debug)]
pub struct Tiff {
    pub width: u32,
    pub height: u32,
    pub x_resolution: u32,
    pub y_resolution: u32,
    pub resolution_unit: u32,
}

#[derive(BinRead, Debug)]
pub struct TiffHeader {
    pub byte_order: ByteOrder,
    #[br(is_little = (byte_order == ByteOrder::Little))]
    pub version: u16,
    #[br(is_little = (byte_order == ByteOrder::Little))]
    pub ifd0_offset: u32,
}
#[derive(BinRead, Debug, PartialEq)]
pub enum ByteOrder {
    #[br(magic(0x4D4Du16))]
    Big,
    #[br(magic(0x4949u16))]
    Little,
}

#[derive(BinRead, Debug)]
pub struct Ifd {
    pub entry_count: u16,
    #[br(count = entry_count)]
    pub data: Vec<IfdEntry>,
    pub next_offset: u32,
}

#[derive(BinRead, Debug)]
pub struct IfdEntry {
    pub tag: TagType,
    pub data_type: EntryType,
    pub data_count: u32,
    #[br(if(data_count == 1 && (data_type == EntryType::Short || data_type == EntryType::Long)))]
    pub data_value: [u16; 2],
    #[br(if(data_count != 1 || !(data_type == EntryType::Short || data_type == EntryType::Long)))]
    pub data_offset: u32,
}

impl IfdEntry {
    pub fn read_value(&self) -> u32 {
        match self.data_type {
            EntryType::Short | EntryType::Long => self.data_value[0] as u32,
            _ => self.data_offset,
        }
    }
}

#[derive(BinRead, Debug, PartialEq)]
pub enum EntryType {
    #[br(magic(3u16))]
    Short,
    #[br(magic(4u16))]
    Long,
    #[br(magic(5u16))]
    Rational,

    Other(u16),
}

#[derive(BinRead, Debug, PartialEq)]
pub enum TagType {
    #[br(magic(0x0100u16))]
    ImageWidth,
    #[br(magic(0x0101u16))]
    ImageLength,
    #[br(magic(0x011Au16))]
    XResolution,
    #[br(magic(0x011Bu16))]
    YResolution,
    #[br(magic(0x0128u16))]
    ResolutionUnit,
    Other(u16),
}

fn _dpi(unit: u32, resolution: u32) -> u32 {
    if unit == 1 {
        72
    } else {
        let upi = if unit == 2 { 1.0 } else { 2.54 };
        (resolution as f32 * upi) as u32
    }
}

impl Tiff {
    pub fn new<R: BufRead + Seek>(reader: &mut R) -> crate::Result<Self> {
        let header = TiffHeader::read_le(reader)?;
        let mut ifd: Ifd;
        if header.byte_order == ByteOrder::Little {
            reader
                .seek(std::io::SeekFrom::Start(header.ifd0_offset as u64))?;
            ifd = Ifd::read_le(reader)?;
        } else {
            reader
                .seek(std::io::SeekFrom::Start(header.ifd0_offset as u64))?;
            ifd = Ifd::read_be(reader)?;
        }

        let mut tiff = Tiff {
            width: 0,
            height: 0,
            x_resolution: 0,
            y_resolution: 0,
            resolution_unit: 0,
        };
        ifd.data.iter().for_each(|x| match x.tag {
            TagType::ImageWidth => tiff.width = x.read_value(),
            TagType::ImageLength => tiff.height = x.read_value(),
            TagType::XResolution => tiff.x_resolution = x.read_value(),
            TagType::YResolution => tiff.y_resolution = x.read_value(),
            TagType::ResolutionUnit => tiff.resolution_unit = x.read_value(),
            _ => {}
        });

        Ok(tiff)
    }
}

impl ImageReader for Tiff {
    fn dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn x_dpi(&self) -> u32 {
        _dpi(self.resolution_unit, self.x_resolution)
    }

    fn y_dpi(&self) -> u32 {
        _dpi(self.resolution_unit, self.y_resolution)
    }
}
