extern crate byteorder;
extern crate nalgebra;

mod tga;

use tga::{TgaColor, TgaImage};
use std::fs::File;

const WHITE: TgaColor = TgaColor { argb: [255, 255, 255, 255] };
const RED: TgaColor = TgaColor { argb: [255, 0, 0, 255] };

fn main() {
    let mut img = TgaImage::new(100, 100);
    img.set(52, 41, RED);

    let mut f = File::create("foo.tga").unwrap();
    img.write(&mut f).unwrap();
}
