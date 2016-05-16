use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Result;
use nalgebra::DMatrix;

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
    pub argb: [u8; 4],
}

impl TgaColor {
    fn write<W: WriteBytesExt>(p: &Self, w: &mut W) -> Result<()> {
        let mut i = 4;
        while i > 0 {
            i -= 1;
            try!(w.write_u8(p.argb[i]));
        }
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

    fn write<W: WriteBytesExt>(h: &Self, w: &mut W) -> Result<()> {
        try!(w.write_u8(h.id_length));
        try!(w.write_u8(h.map_type));
        try!(w.write_u8(h.image_type));
        try!(w.write_u16::<LittleEndian>(h.map_origin));
        try!(w.write_u16::<LittleEndian>(h.map_length));
        try!(w.write_u8(h.map_entry_size));
        try!(w.write_u16::<LittleEndian>(h.x_origin));
        try!(w.write_u16::<LittleEndian>(h.y_origin));
        try!(w.write_u16::<LittleEndian>(h.image_width));
        try!(w.write_u16::<LittleEndian>(h.image_height));
        try!(w.write_u8(h.pixel_depth));
        try!(w.write_u8(h.image_desc));
        Ok(())
    }
}

pub struct TgaImage {
    width: usize,
    height: usize,
    data: DMatrix<TgaColor>,
}

impl TgaImage {
    pub fn new<'a>(w: usize, h: usize) -> TgaImage {
        let c = TgaColor { argb: [0, 0, 0, 0] };
        TgaImage {
            width: w,
            height: h,
            data: DMatrix::from_element(h, w, c),
        }
    }

    pub fn set(self: &mut Self, x: usize, y: usize, c: TgaColor) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.width {
            false
        } else {
            self.data[(y, x)] = c;
            true
        }
    }
}
