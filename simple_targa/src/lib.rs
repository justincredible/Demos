use std::fs::File;
use std::io::{Read, Result, Write};

pub struct TargaImage {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl TargaImage {
    pub fn new(bytes: Vec<u8>, width: u16, height: u16) -> Self {
        let width = width as u32;
        let height = height as u32;

        TargaImage {
            bytes,
            width,
            height,
        }
    }
}

const TGA_HDR: usize = 18;
const TGA_WIDTH: usize = 12;
const TGA_HEIGHT: usize = 14;

pub fn read_targa(path: &str) -> Result<TargaImage> {
    const COMPONENTS: usize = 16;

    let mut file = File::open(path)?;

    let mut data = Vec::new();
    let _read = file.read_to_end(&mut data)?;

    let components = data[COMPONENTS];
    if components != 32 {
        panic!("unexpected TGA format");
    }
    let width = data[TGA_WIDTH + 1] as u32 * 256 + data[TGA_WIDTH] as u32;
    let height = data[TGA_HEIGHT + 1] as u32 * 256 + data[TGA_HEIGHT] as u32;
    let mut bytes = Vec::new();
    for i in 0..(width * height) as usize {
        let index = TGA_HDR + 4 * i;

        bytes.push(data[index + 2]);
        bytes.push(data[index + 1]);
        bytes.push(data[index + 0]);
        bytes.push(data[index + 3]);
    }

    Ok(TargaImage {
        bytes,
        width,
        height,
    })
}

pub fn write_targa(path: &str, mut image: TargaImage) -> Result<()> {
    let mut header = [0u8, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0];
    header[TGA_WIDTH] = (image.width % 256) as u8;
    header[TGA_WIDTH + 1] = (image.width / 256) as u8;
    header[TGA_HEIGHT] = (image.height % 256) as u8;
    header[TGA_HEIGHT + 1] = (image.height / 256) as u8;

    let mut file = File::create(path)?;

    file.write_all(&header)?;

    for i in 0..(image.width * image.height) as usize {
        let index = 4 * i;

        let byte = image.bytes[index];
        image.bytes[index] = image.bytes[index + 2];
        image.bytes[index + 2] = byte;
    }
    file.write_all(&image.bytes)?;

    Ok(())
}
