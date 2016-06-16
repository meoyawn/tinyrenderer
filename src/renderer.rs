use tga::{TgaImage, TgaColor};
use std::mem::swap;
use geometry::*;
use std::f32;
use image::RgbaImage;

pub fn triangle(pts: [Vec3f; 3],
                zbuffer: &mut Vec<f32>,
                image: &mut TgaImage,
                color: TgaColor,
                texture: &RgbaImage,
                txts: &Vec<Vec2f>) {
    let mut bboxmin = Vec2f::new(f32::MAX, f32::MAX);
    let mut bboxmax = Vec2f::new(-f32::MAX, -f32::MAX);
    let clamp = Vec2f::new(image.width as f32 - 1f32, image.height as f32 - 1f32);
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = 0f32.max(bboxmin[j].min(pts[i][j]));
            bboxmax[j] = clamp[j].min(bboxmax[j].max(pts[i][j]));
        }
    }

    let mut tbboxmin = Vec2f::new(f32::MAX, f32::MAX);
    let mut tbboxmax = Vec2f::new(f32::MIN, f32::MIN);
    for i in 0..3 {
        for j in 0..2 {
            tbboxmin[j] = tbboxmin[j].min(txts[i][j]);
            tbboxmax[j] = tbboxmax[j].max(txts[i][j]);
        }
    }
    let tx_jump = (tbboxmax.x - tbboxmin.x) / (bboxmax.x - bboxmin.x);
    let ty_jump = (tbboxmax.y - tbboxmin.y) / (bboxmax.y - bboxmin.y);

    let mut tx = tbboxmin.x;
    let mut ty = tbboxmin.y;
    let mut p = Vec3f::new(0f32, 0f32, 0f32);
    let width = image.width;
    for i in bboxmin.x as i32..bboxmax.x as i32 + 1 {
        for j in bboxmin.y as i32..bboxmax.y as i32 + 1 {
            p.set(i, j);
            let bc_screen = barycentric(&pts[0], &pts[1], &pts[2], &p);

            if bc_screen.x < 0f32 || bc_screen.y < 0f32 || bc_screen.z < 0f32 {
                continue;
            }

            p.z = 0f32;
            for k in 0..3 {
                p.z += pts[k][2] * bc_screen[k];
            }

            let tw = texture.width() as f32;
            let th = texture.height() as f32;
            let xx = (tx * tw).min(tw - 1f32);
            let xy = (ty * th).min(th - 1f32);
            let text = texture[(xx as u32, xy as u32)];

            let idx = (p.x + p.y * width as f32) as usize;
            if zbuffer[idx] < p.z {
                zbuffer[idx] = p.z;
                image.set(p.x as usize,
                          p.y as usize,
                          TgaColor { bgra: [text[2], text[1], text[0], text[3]] });
            }

            tx += tx_jump;
        }
        ty += ty_jump;
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
