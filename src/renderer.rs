use tga::{TgaImage, TgaColor};
use std::mem::swap;
use geometry::{Vec2i, barycentric};
use std::cmp::{max, min};

pub fn triangle(pts: [Vec2i; 3], image: &mut TgaImage, color: TgaColor) {
    let mut bboxmin = Vec2i::new(image.width as i32 - 1, image.height as i32 - 1);
    let mut bboxmax = Vec2i::new(0, 0);
    let clamp = Vec2i::new(image.width as i32 - 1, image.height as i32 - 1);
    for i in 0..3 {
        for j in 0..2 {
            let m = bboxmin[j];
            bboxmin.index_set(j, max(0, min(m, pts[i][j])));
            let m = bboxmax[j];
            bboxmax.index_set(j, min(clamp[j], max(m, pts[i][j])));
        }
    }
    let mut p = Vec2i::new(0, 0);
    for i in bboxmin.x..bboxmax.x + 1 {
        for j in bboxmin.y..bboxmax.y + 1 {
            p.set(i, j);
            let bc_screen = barycentric(&pts, &p);
            if bc_screen.x < 0f32 || bc_screen.y < 0f32 || bc_screen.z < 0f32 {
                continue;
            }
            image.set(p.x as usize, p.y as usize, color);
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
