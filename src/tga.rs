use std::ops::Index;
use std::ops::Mul;

struct TgaHeader {
    idlength: i8,
    colormaptype: i8,
    datatypecode: i8,
    colormaporigin: i16,
    colormaplength: i16,
    colormapdepth: i8,
    x_origin: i16,
    y_origin: i16,
    width: i16,
    height: i16,
    bitsperpixel: i8,
    imagedescriptor: i8,
}

#[derive(Clone, Copy)]
struct TgaColor {
    bgra: [u8; 4],
    bytespp: u8,
}

impl Index<usize> for TgaColor {
    type Output = u8;

    fn index(&self, ind: usize) -> &u8 {
        &self.bgra[ind]
    }
}

impl Mul<f32> for TgaColor {
    type Output = TgaColor;

    fn mul(self, intensity: f32) -> TgaColor {
        let bounded = 1f32.min(0f32.max(intensity));
        TgaColor {
            bgra: [(self[0] as f32 * bounded) as u8,
                   (self[1] as f32 * bounded) as u8,
                   (self[2] as f32 * bounded) as u8,
                   (self[3] as f32 * bounded) as u8],
            bytespp: self.bytespp,
        }
    }
}

fn make_empty() -> TgaColor {
    TgaColor {
        bgra: [0; 4],
        bytespp: 1,
    }
}

fn make_argb(r: u8, g: u8, b: u8, a: u8) -> TgaColor {
    TgaColor {
        bgra: [b, g, r, a],
        bytespp: 4,
    }
}

fn make_empty_v(v: u8) -> TgaColor {
    TgaColor {
        bgra: [0; 4],
        bytespp: v,
    }
}

enum Format {
    GRAYSCALE,
    RGB,
    RGBA,
}

fn to_int(f: &Format) -> u8 {
    match *f {
        Format::GRAYSCALE => 1,
        Format::RGB => 3,
        Format::RGBA => 4,
    }
}

struct TgaImage {
    data: Vec<i8>,
    width: i16,
    height: i16,
    bytespp: i16,
}
