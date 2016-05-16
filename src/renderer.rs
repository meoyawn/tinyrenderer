use tga::{TgaImage, TgaColor};

fn line(x0: u16, y0: u16, x1: u16, y1: u16, image: &mut TgaImage, color: TgaColor) -> () {
    let mut t = 0f32;
    while t < 1f32 {
        let x = x0 as f32 * (1f32 - t) + x1 as f32 * t;
        let y = y0 as f32 * (1f32 - t) + y1 as f32 * t;
        image.set(x as usize, y as usize, color.clone());
        t += 0.01f32
    }
}
