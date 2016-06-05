extern crate byteorder;
extern crate core;
extern crate obj;
extern crate rand;

mod tga;
mod renderer;
mod geometry;

use tga::*;
use std::fs::File;
use renderer::*;
use geometry::Vec2i;
use std::path::Path;
use obj::*;
use std::rc::Rc;
use rand::Rng;

const WHITE: TgaColor = TgaColor { bgra: [255, 255, 255, 255] };
const RED: TgaColor = TgaColor { bgra: [0, 0, 255, 255] };
const GREEN: TgaColor = TgaColor { bgra: [0, 255, 0, 255] };

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn head_lines(image: &mut TgaImage, color: TgaColor) {
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
                    line(x0 as i32,
                         y0 as i32,
                         x1 as i32,
                         y1 as i32,
                         image,
                         color.clone());
                }
            }
        }
    }
}

fn head_triangles(image: &mut TgaImage) {
    let obj = head();
    let verts = obj.position();
    let mut rng = rand::thread_rng();
    for o in obj.object_iter() {
        for g in o.group_iter() {
            for tups in g.indices() {
                let face = tups.iter().map(|&(i, _, _)| i).collect::<Vec<_>>();
                let mut screen_coords = [Vec2i::new(0, 0), Vec2i::new(0, 0), Vec2i::new(0, 0)];
                for j in 0..3 {
                    let world_coords = verts[face[j]];
                    let f_width = WIDTH as f32;
                    let f_height = HEIGHT as f32;
                    screen_coords[j] = Vec2i::newf32((world_coords[0] + 1f32) * f_width / 2f32,
                                                     (world_coords[1] + 1f32) * f_height / 2f32)
                }
                triangle(screen_coords,
                         image,
                         TgaColor {
                             bgra: [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), 255],
                         });
            }
        }
    }
}

fn main() {
    let mut image = TgaImage::new(WIDTH, HEIGHT);

    // head_lines(&mut image, w);
    head_triangles(&mut image);

    let mut f = File::create("foo.tga").unwrap();
    image.write(&mut f).unwrap();
}

fn head() -> Obj<Rc<Material>, SimplePolygon> {
    let f = load::<SimplePolygon>(Path::new("obj/african_head/african_head.obj"));
    f.unwrap()
}
