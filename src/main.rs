extern crate byteorder;
extern crate core;
extern crate obj;

mod tga;
mod renderer;
mod geometry;

use tga::*;
use std::fs::File;
use renderer::*;
use geometry::Point;
use std::path::Path;
use obj::*;
use std::rc::Rc;

const WHITE: TgaColor = TgaColor { bgra: [255, 255, 255, 255] };
const RED: TgaColor = TgaColor { bgra: [0, 0, 255, 255] };
const BLACK: TgaColor = TgaColor { bgra: [0, 0, 0, 255] };
const GREEN: TgaColor = TgaColor { bgra: [0, 255, 0, 255] };

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn draw_head<'a>(image: &mut TgaImage<'a>, color: &'a TgaColor) {
    let obj = head();
    let verts = obj.position();
    for o in obj.object_iter() {
        for g in o.group_iter() {
            for tups in g.indices() {
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
                    line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, image, color);
                }
            }
        }
    }
}

fn main() {
    let b = &BLACK;
    let w = &WHITE;
    let r = &RED;
    let g = &GREEN;

    let mut image = TgaImage::new(WIDTH, HEIGHT, b);

    // draw_head(&mut image, w);

    let t0 = [Point::new(10, 70), Point::new(50, 160), Point::new(70, 80)];
    let t1 = [Point::new(180, 50), Point::new(150, 1), Point::new(70, 180)];
    let t2 = [Point::new(180, 150), Point::new(120, 160), Point::new(130, 180)];

    triangle(t0[0], t0[1], t0[2], &mut image, r);
    triangle(t1[0], t1[1], t1[2], &mut image, w);
    triangle(t2[0], t2[1], t2[2], &mut image, g);

    let mut f = File::create("foo.tga").unwrap();
    image.write(&mut f).unwrap();
}

fn head() -> Obj<Rc<Material>, SimplePolygon> {
    let f = load::<SimplePolygon>(Path::new("obj/african_head/african_head.obj"));
    f.unwrap()
}
