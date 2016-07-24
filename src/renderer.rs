use tga::{TgaImage, TgaColor};
use std::mem::swap;
use geometry::*;
use std::f32;
use image::RgbaImage;
use image::Rgba;

fn diffuse(img: &RgbaImage, uv: Vec2i) -> Rgba<u8> {
    img[(uv.x as u32, uv.y as u32)]
}

pub fn triangle(ts: [Vec3i; 3],
                uvs: [Vec2i; 3],
                image: &mut TgaImage,
                intensity: f32,
                zbuffer: &mut Vec<i32>,
                texture: &RgbaImage) {
    let mut t0 = ts[0];
    let mut t1 = ts[1];
    let mut t2 = ts[2];
    if t0.y == t1.y && t0.y == t2.y {
        return;
    }

    let mut uv0 = uvs[0];
    let mut uv1 = uvs[1];
    let mut uv2 = uvs[2];

    if t0.y > t1.y {
        swap(&mut t0, &mut t1);
        swap(&mut uv0, &mut uv1);
    }
    if t0.y > t2.y {
        swap(&mut t0, &mut t2);
        swap(&mut uv0, &mut uv2);
    }
    if t1.y > t2.y {
        swap(&mut t1, &mut t2);
        swap(&mut uv1, &mut uv2);
    }

    let total_height = t2.y - t0.y;
    for i in 0..total_height {
        let second_half = i > t1.y - t0.y || t1.y == t0.y;
        let segment_height = if second_half {
            t2.y - t1.y
        } else {
            t1.y - t0.y
        };
        let alpha = i as f32 / total_height as f32;
        let xxx = if second_half {
            t1.y - t0.y
        } else {
            0
        };
        let beta = (i - xxx) as f32 / segment_height as f32;

        let mut A = t0 + Vec3f::newVec3i(t2 - t0) * alpha;
        let mut B = if second_half {
            t1 + Vec3f::newVec3i(t2 - t1) * beta
        } else {
            t0 + Vec3f::newVec3i(t1 - t0) * beta
        };
        let mut uvA = uv0 + (uv2 - uv0) * alpha;
        let mut uvB = if second_half {
            uv1 + (uv2 - uv1) * beta
        } else {
            uv0 + (uv1 - uv0) * beta
        };
        if A.x > B.x {
            swap(&mut A, &mut B);
            swap(&mut uvA, &mut uvB);
        }

        for j in A.x..B.x + 1 {
            let phi = if B.x == A.x {
                1f32
            } else {
                (j - A.x) as f32 / (B.x - A.x) as f32
            };
            let P: Vec3i = Vec3i::newVec3f(Vec3f::newVec3i(A) + Vec3f::newVec3i(B - A) * phi);
            let uvP: Vec2i = uvA + (uvB - uvA) * phi;
            let idx = (P.x + P.y * image.width as i32) as usize;
            if zbuffer[idx] < P.z {
                zbuffer[idx] = P.z;
                let color = diffuse(texture, uvP);
                image.set(P.x as usize, P.y as usize, TgaColor::new(&color, intensity))
            }
        }
    }
}

pub fn v_line(v1: &Vec2i, v2: &Vec2i, image: &mut TgaImage, color: TgaColor) {
    line(v1.x, v1.y, v2.x, v2.y, image, color)
}

pub fn rasterize(p0: &Vec2i,
                 p1: &Vec2i,
                 image: &mut TgaImage,
                 color: TgaColor,
                 y_buffer: &mut Vec<i32>) {
    let mut p0 = p0;
    let mut p1 = p1;
    if p0.x > p1.x {
        swap(&mut p0, &mut p1);
    }
    for x in p0.x..p1.x + 1 {
        let t = (x - p0.x) as f32 / (p1.x - p0.x) as f32;
        let y = (p0.y as f32 * (1f32 - t) + p1.y as f32 * t) as i32;
        let idx = x as usize;
        if y_buffer[idx] < y {
            y_buffer[idx] = y;
            for i in 0..16 {
                image.set(idx, i, color);
            }
        }
    }
}

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut TgaImage, color: TgaColor) -> () {
    let mut steep = false;

    let mut x0 = x0;
    let mut x1 = x1;
    let mut y0 = y0;
    let mut y1 = y1;

    if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    for x in x0..x1 + 1 {
        if steep {
            image.set(y as usize, x as usize, color);
        } else {
            image.set(x as usize, y as usize, color);
        }
        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 {
                1
            } else {
                -1
            };
            error2 -= dx * 2;
        }
    }
}
