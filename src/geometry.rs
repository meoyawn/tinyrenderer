use std::ops::{Add, Sub, Mul, Index, BitXor, IndexMut};

pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Index<usize> for Vec2f {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("exhaust"),
        }
    }
}

impl IndexMut<usize> for Vec2f {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("exhaust"),
        }
    }
}

impl IndexMut<usize> for Vec3f {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("exhaust"),
        }
    }
}

impl IndexMut<usize> for Vec2i {
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("exhaust"),
        }
    }
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x: x, y: y }
    }
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

#[derive(Clone,Copy)]
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
    fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Vec3f {
        let n = self.norm();
        Vec3f::new(self.x / n, self.y / n, self.z / n)
    }
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x as f32;
        self.y = y as f32;
    }
}

impl Index<usize> for Vec3f {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("exhaust"),
        }
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// cross product
impl BitXor for Vec3f {
    type Output = Vec3f;
    fn bitxor(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(self.y * rhs.z - self.z * rhs.y,
                   self.z * rhs.x - self.x * rhs.z,
                   self.x * rhs.y - self.y * rhs.x)
    }
}

// dot product
impl Mul for Vec3f {
    type Output = f32;
    fn mul(self, rhs: Vec3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

pub fn barycentric(a: &Vec3f, b: &Vec3f, c: &Vec3f, p: &Vec3f) -> Vec3f {
    let mut s = [Vec3f::new(0f32, 0f32, 0f32); 2];
    for i in (0..2).rev() {
        s[i][0] = c[i] - a[i];
        s[i][1] = b[i] - a[i];
        s[i][2] = a[i] - p[i];
    }
    let u = s[0] ^ s[1];
    if u[2].abs() > 1e-2 {
        return Vec3f::new(1f32 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
    }
    Vec3f::new(-1f32, 1f32, 1f32)
}
