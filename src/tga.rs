use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Result;

/// Header used by TGA image files
#[derive(Debug)]
struct Header {
    id_length: u8, // length of ID string
    map_type: u8, // color map type
    image_type: u8, // image type code
    map_origin: u16, // starting index of map
    map_length: u16, // length of map
    map_entry_size: u8, // size of map entries in bits
    x_origin: u16, // x-origin of image
    y_origin: u16, // y-origin of image
    image_width: u16, // width of image
    image_height: u16, // height of image
    pixel_depth: u8, // bits per pixel
    image_desc: u8, // image descriptor
}

#[derive(Clone,Copy)]
pub struct TgaColor {
    pub rgba: [u8; 4],
}

impl TgaColor {
    fn write<W: WriteBytesExt>(self: &Self, w: &mut W) -> Result<()> {
        try!(w.write_u8(self.rgba[2]));
        try!(w.write_u8(self.rgba[1]));
        try!(w.write_u8(self.rgba[0]));
        try!(w.write_u8(self.rgba[3]));
        Ok(())
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

    fn write<W: WriteBytesExt>(self: &Self, w: &mut W) -> Result<()> {
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

struct Matrix<T> {
    width: usize,
    height: usize,
    vec: Vec<T>,
}

pub struct TgaImage {
    m: Matrix<TgaColor>,
}

impl<T: Copy> Matrix<T> {
    pub fn set(self: &mut Self, i: usize, j: usize, t: T) -> () {
        self.vec[j * self.height + i] = t
    }
    fn get(self: &Self, i: usize, j: usize) -> T {
        self.vec[j * self.height + i]
    }
}

impl TgaImage {
    pub fn new(w: usize, h: usize) -> TgaImage {
        let c = TgaColor { rgba: [0, 0, 0, 255] };
        TgaImage {
            m: Matrix {
                width: w,
                height: h,
                vec: vec![c; w * h],
            },
        }
    }
    pub fn write<W: WriteBytesExt>(self: &Self, w: &mut W) -> Result<()> {
        try!(Header::new(self.m.width as u16, self.m.height as u16).write(w));
        for c in &self.m.vec {
            try!(c.write(w));
        }
        Ok(())
    }
    pub fn set(self: &mut Self, i: usize, j: usize, c: TgaColor) -> () {
        self.m.set(i, j, c)
    }
}
