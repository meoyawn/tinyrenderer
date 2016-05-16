extern crate byteorder;
extern crate nalgebra;

mod tga;
mod renderer;

use tga::{TgaColor, TgaImage};
use std::fs::File;
use renderer::line;

const WHITE: TgaColor = TgaColor { rgba: [255, 255, 255, 255] };
const RED: TgaColor = TgaColor { rgba: [255, 0, 0, 255] };

fn main() {
    let mut image = TgaImage::new(100, 100);

    line(13, 20, 80, 40, &mut image, WHITE);
    line(20, 13, 40, 80, &mut image, RED);
    line(80, 40, 13, 20, &mut image, RED);

    let mut f = File::create("foo.tga").unwrap();
    image.flip_vertically();
    image.write(&mut f).unwrap();
}
