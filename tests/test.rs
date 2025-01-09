use imagesize::bmp::Bmp;
use imagesize::{Image, ImageReader};
use std::{fs, io};
use std::io::{Cursor, Read, Seek};
use binrw::{BinRead, BinReaderExt};
use imagesize::gif::Gif;
use imagesize::jpeg::{Jpeg, Segment};
use imagesize::jpeg::Marker::APP1;
use imagesize::png::Png;
use imagesize::tiff::Tiff;

#[test]
fn test_bmp() {
    let bmp = Image::from_file("tests/images/bmp.bmp").unwrap();
    println!("{:?}", bmp);
    let width = bmp.width();
    let height = bmp.height();
    let x_dpi = bmp.x_dpi();
    let y_dpi = bmp.y_dpi();
    println!("{:?}", (width.value(), height.value(), x_dpi, y_dpi));
}

#[test]
fn test_gif() {
    let bmp = Image::from_file("tests/images/gif.gif").unwrap();
    println!("{:?}", bmp);
    let width = bmp.width();
    let height = bmp.height();
    let x_dpi = bmp.x_dpi();
    let y_dpi = bmp.y_dpi();
    println!("{:?}", (width.value(), height.value(), x_dpi, y_dpi));
}

#[test]
fn test_png() {
    let bmp = Image::from_file("tests/images/png.png").unwrap();
    println!("{:?}", bmp);
    let width = bmp.width();
    let height = bmp.height();
    let x_dpi = bmp.x_dpi();
    let y_dpi = bmp.y_dpi();
    println!("{:?}", (width.value(), height.value(), x_dpi, y_dpi));
}

#[test]
fn test_jpeg() {
    let bmp = Image::from_file("tests/images/jpeg.jpg").unwrap();
    println!("{:?}", bmp);
    let width = bmp.width();
    let height = bmp.height();
    let x_dpi = bmp.x_dpi();
    let y_dpi = bmp.y_dpi();
    println!("{:?}", (width.value(), height.value(), x_dpi, y_dpi));

    // let mut reader = Cursor::new(reader);
}