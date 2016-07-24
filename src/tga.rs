use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Result, Write};
use image::Rgba;

// Header used by TGA image files
struct Header {
    id_length: u8,
    map_type: u8,
    image_type: u8,
    map_origin: u16,
    map_length: u16,
    map_entry_size: u8,
    x_origin: u16,
    y_origin: u16,
    image_width: u16,
    image_height: u16,
    pixel_depth: u8,
    image_desc: u8,
}

#[derive(Clone, Copy)]
pub struct TgaColor {
    pub bgra: [u8; 4],
}

impl TgaColor {
    pub fn new(rgba: &Rgba<u8>) -> TgaColor {
        TgaColor { bgra: [rgba[2], rgba[1], rgba[0], rgba[3]] }
    }
    fn write<W: Write>(&self, w: &mut W) -> Result<usize> {
        w.write(&self.bgra)
    }
}

const RAW_TRUE_COLOR: u8 = 2;
const ARGB_DEPTH: u8 = 32;

impl Header {
    fn new(w: u16, h: u16) -> Header {
        Header {
            id_length: 0,
            map_type: 0,
            image_type: RAW_TRUE_COLOR,
            map_origin: 0,
            map_length: 0,
            map_entry_size: 0,
            x_origin: 0,
            y_origin: 0,
            image_width: w,
            image_height: h,
            pixel_depth: ARGB_DEPTH,
            image_desc: 0,
        }
    }
    fn write<W: WriteBytesExt>(&self, w: &mut W) -> Result<()> {
        try!(w.write_u8(self.id_length));
        try!(w.write_u8(self.map_type));
        try!(w.write_u8(self.image_type));
        try!(w.write_u16::<LittleEndian>(self.map_origin));
        try!(w.write_u16::<LittleEndian>(self.map_length));
        try!(w.write_u8(self.map_entry_size));
        try!(w.write_u16::<LittleEndian>(self.x_origin));
        try!(w.write_u16::<LittleEndian>(self.y_origin));
        try!(w.write_u16::<LittleEndian>(self.image_width));
        try!(w.write_u16::<LittleEndian>(self.image_height));
        try!(w.write_u8(self.pixel_depth));
        try!(w.write_u8(self.image_desc));
        Ok(())
    }
}

pub struct TgaImage {
    pub width: usize,
    pub height: usize,
    data: Vec<TgaColor>,
}

const BLACK: TgaColor = TgaColor { bgra: [0, 0, 0, 255] };

impl TgaImage {
    pub fn new(w: usize, h: usize) -> TgaImage {
        TgaImage {
            width: w,
            height: h,
            data: vec![BLACK; w * h],
        }
    }
    pub fn write<W: Write>(&self, w: &mut W) -> Result<()> {
        try!(Header::new(self.width as u16, self.height as u16).write(w));
        for c in &self.data {
            try!(c.write(w));
        }
        Ok(())
    }
    pub fn set(&mut self, x: usize, y: usize, t: TgaColor) -> () {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = t
        }
    }
}
