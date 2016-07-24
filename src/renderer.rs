use tga::{TgaImage, TgaColor};
use std::mem::swap;
use geometry::*;
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

            let pre = Vec3f::newVec3i(A) + Vec3f::newVec3i(B - A) * phi;
            let p = Vec3i::newVec3f(pre);

            let uv_p: Vec2i = uvA + (uvB - uvA) * phi;
            let idx = (p.x + p.y * image.width as i32) as usize;
            if zbuffer[idx] < p.z {
                zbuffer[idx] = p.z;
                let color = diffuse(texture, uv_p);
                image.set(p.x as usize, p.y as usize, TgaColor::new(&color, intensity))
            }
        }
    }
}
