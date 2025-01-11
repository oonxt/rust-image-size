use imagesize::bmp::Bmp;
use imagesize::{Image, ImageReader};
use std::{fs, io};
use std::io::{Cursor, Read, Seek};
use binrw::{BinRead, BinReaderExt};
use imagesize::gif::Gif;
use imagesize::jpeg::{Jpeg, Segment};
use imagesize::jpeg::Marker::APP1;
use imagesize::png::{Chunk, Info, Png};
use imagesize::tiff::Tiff;

#[test]
fn test_bmp() {
    let bmp = Image::from_file("tests/images/bmp.bmp").unwrap();
    println!("{:?}", bmp);
}

#[test]
fn test_gif() {
    let bmp = Image::from_file("tests/images/gif.gif").unwrap();
    println!("{:?}", bmp);

}
#[derive(Debug, BinRead)]
#[br(big)]
#[br(magic = b"\x89PNG\x0D\x0A\x1A\x0A")]
pub struct Pngf {
    #[br(ignore)]
    pub info: Option<Info>,
}
#[test]
fn test_png() {
    let bmp = Image::from_file("tests/images/png.png").unwrap();
    println!("{:?}", bmp);
}

#[test]
fn test_jpeg() {
    let bmp = Image::from_file("tests/images/jpeg.jpg").unwrap();
    println!("{:?}", bmp);
}