use tga::{TgaImage, TgaColor};

pub fn line(x0: u16, y0: u16, x1: u16, y1: u16, image: &mut TgaImage, color: TgaColor) -> () {
    for x in x0..x1 + 1 {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = y0 as f32 * (1f32 - t) + y1 as f32 * t;
        image.set(x as usize, y as usize, color);
    }
}
