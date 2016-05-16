mod tga;
mod renderer;
extern crate byteorder;
extern crate nalgebra;

use tga::{TgaColor};

const WHITE:TgaColor = TgaColor { argb: [255,255,255,255] };
const RED:TgaColor = TgaColor { argb: [255,0,0,255] };

fn main() {
    println!("Hello, world!");
}
