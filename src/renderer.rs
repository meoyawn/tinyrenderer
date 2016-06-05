use tga::{TgaImage, TgaColor};
use std::mem::swap;

pub fn line<'a>(x0: i32,
                y0: i32,
                x1: i32,
                y1: i32,
                image: &mut TgaImage<'a>,
                color: &'a TgaColor)
                -> () {
    let mut steep = false;

    let mut finx0 = x0;
    let mut finx1 = x1;
    let mut finy0 = y0;
    let mut finy1 = y1;

    if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut finx0, &mut finy0);
        swap(&mut finx1, &mut finy1);
        steep = true;
    }
    if finx0 > finx1 {
        swap(&mut finx0, &mut finx1);
        swap(&mut finy0, &mut finy1);
    }

    let dx = finx1 - finx0;
    let dy = finy1 - finy0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = finy0;

    for x in finx0..finx1 + 1 {
        if steep {
            image.set(y as usize, x as usize, color);
        } else {
            image.set(x as usize, y as usize, color);
        }
        error2 += derror2;
        if error2 > dx {
            y += if finy1 > finy0 {
                1
            } else {
                -1
            };
            error2 -= dx * 2;
        }
    }
}
