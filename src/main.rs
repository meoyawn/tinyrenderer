extern crate byteorder;
extern crate wavefront_obj;
extern crate core;

mod tga;
mod renderer;

use tga::{TgaColor, TgaImage};
use std::fs::File;
use renderer::line;
use wavefront_obj::obj::{parse,ObjSet};
use std::io::Read;

const WHITE: TgaColor = TgaColor { rgba: [255, 255, 255, 255] };
const RED: TgaColor = TgaColor { rgba: [255, 0, 0, 255] };

fn main() {
    let mut image = TgaImage::new(100, 100);

    line(13, 20, 80, 40, &mut image, WHITE);
    line(20, 13, 40, 80, &mut image, RED);
    line(80, 40, 13, 20, &mut image, RED);

    println!("{:?}", head().objects[0].geometry[0].shapes.len());

    let mut f = File::create("foo.tga").unwrap();
    image.write(&mut f).unwrap();
}

fn head() -> ObjSet {
let mut input = String::new();
let mut f =     File::open("obj/african_head.obj").unwrap();
f.read_to_string(&mut input).unwrap();
parse(input).unwrap()
}
