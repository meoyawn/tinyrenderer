use std::ops::{Add, Sub, Mul, Index};

pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Vec2i {
        Vec2i { x: x, y: y }
    }
    pub fn newf32(x: f32, y: f32) -> Vec2i {
        Vec2i {
            x: x as i32,
            y: y as i32,
        }
    }
    pub fn index_set(&mut self, index: usize, v: i32) {
        match index {
            0 => self.x = v,
            1 => self.y = v,
            _ => panic!("exhaust"),
        }
    }
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

impl Add for Vec2i {
    type Output = Vec2i;
    fn add(self, rhs: Vec2i) -> Vec2i {
        Vec2i::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;
    fn sub(self, rhs: Vec2i) -> Vec2i {
        Vec2i::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2i {
    type Output = Vec2i;
    fn mul(self, rhs: f32) -> Vec2i {
        Vec2i::new((self.x as f32 * rhs) as i32, (self.y as f32 * rhs) as i32)
    }
}

impl Index<usize> for Vec2i {
    type Output = i32;
    fn index(&self, index: usize) -> &i32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("exhaust"),
        }
    }
}

pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x: x, y: y, z: z }
    }
    pub fn newi32(x: i32, y: i32, z: i32) -> Vec3f {
        Vec3f::new(x as f32, y as f32, z as f32)
    }
}

impl Mul for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(self.y * rhs.z - self.z * rhs.y,
                   self.z * rhs.x - self.x * rhs.z,
                   self.x * rhs.y - self.y * rhs.x)
    }
}

pub fn barycentric(pts: &[Vec2i; 3], p: &Vec2i) -> Vec3f {
    let u = Vec3f::newi32(pts[2].x - pts[0].x, pts[1].x - pts[0].x, pts[0].x - p.x) *
            Vec3f::newi32(pts[2].y - pts[0].y, pts[1].y - pts[0].y, pts[0].y - p.y);
    if u.z.abs() < 1f32 {
        return Vec3f::new(-1f32, 1f32, 1f32);
    }
    Vec3f::new(1f32 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
}
