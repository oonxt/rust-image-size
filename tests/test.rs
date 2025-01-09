use imagesize::bmp::Bmp;
use imagesize::ImageReader;
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
    let reader = fs::read("tests/images/bmp.bmp").unwrap();
    let bmp = &Bmp::new(&mut Cursor::new(reader));
    println!("{:?}", bmp);
    let width = <Bmp as ImageReader>::width(bmp);
    let height = <Bmp as ImageReader>::height(bmp);
    let x_dpi = <Bmp as ImageReader>::x_dpi(bmp);
    let y_dpi = <Bmp as ImageReader>::y_dpi(bmp);
    println!("{:?}", (width.value(), height.value(), x_dpi, y_dpi));
}

#[test]
fn test_gif() {
    let reader = fs::read("tests/images/gif.gif").unwrap();
    let bmp = &Gif::new(&mut Cursor::new(reader));
    println!("{:?}", bmp);
    let width = <Gif as ImageReader>::width(bmp);
    let height = <Gif as ImageReader>::height(bmp);
    let x_dpi = <Gif as ImageReader>::x_dpi(bmp);
    let y_dpi = <Gif as ImageReader>::y_dpi(bmp);
    println!("{:?}", (width.value(), height.value(), x_dpi, y_dpi));
}

#[test]
fn test_png() {
    let reader = fs::read("tests/images/png.png").unwrap();
    let bmp = &Png::new(&mut Cursor::new(reader));
    println!("{:?}", bmp);
    let width = <Png as ImageReader>::width(bmp);
    let height = <Png as ImageReader>::height(bmp);
    let x_dpi = <Png as ImageReader>::x_dpi(bmp);
    let y_dpi = <Png as ImageReader>::y_dpi(bmp);
    println!("{:?}", (width.value(), height.value(), x_dpi, y_dpi));
}

#[test]
fn test_jpeg() {
    let data = fs::read("tests/images/jpeg.jpg").unwrap();
    let bmp = &Jpeg::new(&mut Cursor::new(data));
    println!("{:?}", bmp);
    let width = <Jpeg as ImageReader>::width(bmp);
    let height = <Jpeg as ImageReader>::height(bmp);
    let x_dpi = <Jpeg as ImageReader>::x_dpi(bmp);
    let y_dpi = <Jpeg as ImageReader>::y_dpi(bmp);
    println!("{:?}", (width.value(), height.value(), x_dpi, y_dpi));

    // let mut reader = Cursor::new(reader);
}