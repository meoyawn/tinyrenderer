extern crate byteorder;
extern crate core;
extern crate obj;

mod tga;
mod renderer;

use tga::*;
use std::fs::File;
use renderer::line;
use std::path::*;
use obj::*;
use std::rc::Rc;

const WHITE: TgaColor = TgaColor { rgba: [255, 255, 255, 255] };
const RED: TgaColor = TgaColor { rgba: [255, 0, 0, 255] };
const BLACK: TgaColor = TgaColor { rgba: [0, 0, 0, 255] };

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let b = &BLACK;
    let w = &WHITE;

    let mut image = TgaImage::new(WIDTH, HEIGHT, b);

    let obj = head();
    let verts = obj.position();
    for o in obj.object_iter() {
        for g in o.group_iter() {
            let is = g.indices();
            println!("{:?}", is.len());
            for tups in is {
                let face = tups.iter().map(|&(i, _, _)| i).collect::<Vec<_>>();
                for j in 0..3 {
                    let v0 = verts[face[j]];
                    let v1 = verts[face[(j + 1) % 3]];
                    let f_width = WIDTH as f32;
                    let f_height = HEIGHT as f32;
                    let x0 = (v0[0] + 1f32) * f_width / 2f32;
                    let y0 = (v0[1] + 1f32) * f_height / 2f32;
                    let x1 = (v1[0] + 1f32) * f_width / 2f32;
                    let y1 = (v1[1] + 1f32) * f_height / 2f32;
                    line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, &mut image, w);
                }
            }
        }
    }

    let mut f = File::create("foo.tga").unwrap();
    image.write(&mut f).unwrap();
}

fn head() -> Obj<Rc<Material>, SimplePolygon> {
    let f = load::<SimplePolygon>(Path::new("obj/african_head/african_head.obj"));
    f.unwrap()
}
