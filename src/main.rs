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
use geometry::{Vec2i, Vec3f, Vec3i};
use std::path::Path;
use obj::*;
use std::rc::Rc;
use std::i32;
use image::{open, ImageResult, RgbaImage};

const WHITE: TgaColor = TgaColor { bgra: [255, 255, 255, 255] };
const RED: TgaColor = TgaColor { bgra: [0, 0, 255, 255] };
const GREEN: TgaColor = TgaColor { bgra: [0, 255, 0, 255] };
const BLUE: TgaColor = TgaColor { bgra: [255, 0, 0, 255] };

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn uv(face: Vec<usize>, nvert: usize) -> usize {
    face[nvert]
}

fn head_triangles(image: &mut TgaImage, light_dir: Vec3f) {
    let obj = head();
    let texture = texture();
    println!("{:?}", texture.width());

    let verts = obj.position();
    let textures = obj.texture();

    let mut zbuffer = vec![-i32::MAX; WIDTH*HEIGHT];
    for o in obj.object_iter() {
        for g in o.group_iter() {
            for tups in g.indices() {
                let face = tups.iter().map(|&(i, _, _)| i).collect::<Vec<_>>();
                let text = tups.iter().map(|&(_, i, _)| i.unwrap()).collect::<Vec<_>>();

                let mut screen_coords =
                    [Vec3i::new(0, 0, 0), Vec3i::new(0, 0, 0), Vec3i::new(0, 0, 0)];
                let mut world_coords =
                    [Vec3f::newi32(0, 0, 0), Vec3f::newi32(0, 0, 0), Vec3f::newi32(0, 0, 0)];
                for j in 0..3 {
                    let v = verts[face[j]];
                    let f_width = WIDTH as f32;
                    let f_height = HEIGHT as f32;
                    screen_coords[j] = Vec3i::newf32((v[0] + 1f32) * f_width / 2f32,
                                                     (v[1] + 1f32) * f_height / 2f32,
                                                     v[2]);
                    world_coords[j] = Vec3f::new(v[0], v[1], v[2]);
                }
                let n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);
                let n = n.normalize();
                // println!("world {:?}", n);
                let intensity = n * light_dir;
                if intensity > 0f32 {
                    let mut uvs = [Vec2i::new(0, 0), Vec2i::new(0, 0), Vec2i::new(0, 0)];
                    for k in 0..3 {
                        let fuck = textures[text[k]];
                        println!("fuck {:?}", fuck);
                        uvs[k] = Vec2i::new((fuck[0] * texture.width() as f32) as i32,
                                            (fuck[1] * texture.height() as f32) as i32);
                        println!("uv {:?}", uvs[k]);
                    }

                    triangle(screen_coords, uvs, image, intensity, &mut zbuffer, &texture);
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
    load::<SimplePolygon>(Path::new("obj/african_head/african_head.obj")).unwrap()
}

fn texture() -> RgbaImage {
    open(Path::new("obj/african_head/african_head_diffuse.tga")).unwrap().to_rgba()
}
