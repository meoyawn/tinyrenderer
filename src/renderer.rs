use tga::{TgaImage, TgaColor};
use std::mem::swap;
use geometry::Point;

pub fn triangle<'a>(t0: Point,
                    t1: Point,
                    t2: Point,
                    image: &mut TgaImage<'a>,
                    color: &'a TgaColor) {
    if t0.y == t1.y && t0.y == t2.y {
        return;
    }

    let mut t0 = t0;
    let mut t1 = t1;
    let mut t2 = t2;

    if t0.y > t1.y {
        swap(&mut t0, &mut t1);
    }
    if t0.y > t2.y {
        swap(&mut t0, &mut t2);
    }
    if t1.y > t2.y {
        swap(&mut t1, &mut t2);
    }

    let total_height = t2.y - t0.y;
    for i in 0..total_height {
        let second_half = i > (t1.y - t0.y) || t1.y == t0.y;
        let segment_height = if second_half {
            t2.y - t1.y
        } else {
            t1.y - t0.y
        };
        let alpha = i as f32 / total_height as f32;
        let mns = if second_half {
            t1.y - t0.y
        } else {
            0
        };
        let beta = (i - mns) as f32 / segment_height as f32;
        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = if second_half {
            t1 + (t2 - t1) * beta
        } else {
            t0 + (t1 - t0) * beta
        };
        if a.x > b.x {
            swap(&mut a, &mut b);
        }
        for j in a.x..b.x + 1 {
            image.set(j as usize, (t0.y + i) as usize, color);
        }
    }
}

fn pLine<'a>(v1: &Point, v2: &Point, image: &mut TgaImage<'a>, color: &'a TgaColor) {
    line(v1.x, v1.y, v2.x, v2.y, image, color)
}

pub fn line<'a>(x0: i32,
                y0: i32,
                x1: i32,
                y1: i32,
                image: &mut TgaImage<'a>,
                color: &'a TgaColor)
                -> () {
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
