extern crate byteorder;
extern crate core;
extern crate obj;
extern crate image;

mod tga;
mod renderer;
mod geometry;

use tga::*;
use std::fs::File;
use renderer::*;
use geometry::{Vec2i, Vec3f};
use std::path::Path;
use obj::*;
use std::rc::Rc;
use std::f32;

const WHITE: TgaColor = TgaColor { bgra: [255, 255, 255, 255] };
const RED: TgaColor = TgaColor { bgra: [0, 0, 255, 255] };
const GREEN: TgaColor = TgaColor { bgra: [0, 255, 0, 255] };
const BLUE: TgaColor = TgaColor { bgra: [255, 0, 0, 255] };

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
                    line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, image, color);
                }
            }
        }
    }
}

fn head_triangles(image: &mut TgaImage, light_dir: Vec3f) {
    let obj = head();
    let verts = obj.position();
    let mut zbuffer = vec![-f32::MAX; WIDTH*HEIGHT];
    for o in obj.object_iter() {
        for g in o.group_iter() {
            for tups in g.indices() {
                let face = tups.iter().map(|&(i, _, _)| i).collect::<Vec<_>>();
                let mut screen_coords =
                [Vec3f::newi32(0, 0, 0), Vec3f::newi32(0, 0, 0), Vec3f::newi32(0, 0, 0)];
                let mut world_coords =
                [Vec3f::newi32(0, 0, 0), Vec3f::newi32(0, 0, 0), Vec3f::newi32(0, 0, 0)];
                for j in 0..3 {
                    let v = verts[face[j]];
                    let f_width = WIDTH as f32;
                    let f_height = HEIGHT as f32;
                    screen_coords[j] = Vec3f::new((v[0] + 1f32) * f_width / 2f32,
                                                  (v[1] + 1f32) * f_height / 2f32,
                                                  v[2]);
                    world_coords[j] = Vec3f::new(v[0], v[1], v[2]);
                }
                let n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);
                let n = n.normalize();
                let intensity = n * light_dir;
                if intensity > 0f32 {
                    let mut uvs = [Vec2i::new(0, 0), Vec2i::new(0, 0), Vec2i::new(0, 0)];
                    let c = (intensity * 255f32) as u8;
                    triangle(screen_coords,
                             &mut zbuffer,
                             image,
                             TgaColor { bgra: [c, c, c, 255] });
                }
            }
        }
    }
}

fn main() {
    let mut image = TgaImage::new(WIDTH, HEIGHT);
    head_triangles(&mut image, Vec3f::new(0f32, 0f32, -1f32));

    let mut f = File::create("foo.tga").unwrap();
    image.write(&mut f).unwrap();
}

fn head() -> Obj<Rc<Material>, SimplePolygon> {
    let f = load::<SimplePolygon>(Path::new("obj/african_head/african_head.obj"));
    f.unwrap()
}
