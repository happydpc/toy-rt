use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, Neg, Index, IndexMut};
use packed_simd::{f32x4, shuffle};

#[derive(Clone, Copy)]
pub struct Vec3(f32x4);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(f32x4::new(x, y, z, 0.))
    }

    pub fn splat(xyz: f32) -> Self {
        Self::new(xyz, xyz, xyz)
    }

    pub fn random() -> Self {
        use rand::random;
        Self(f32x4::new(random(), random(), random(), random()))
    }

    pub fn x(&self) -> f32 { unsafe { self.0.extract_unchecked(0) } }
    pub fn y(&self) -> f32 { unsafe { self.0.extract_unchecked(1) } }
    pub fn z(&self) -> f32 { unsafe { self.0.extract_unchecked(2) } }

    pub fn r(&self) -> f32 { self.x() }
    pub fn g(&self) -> f32 { self.y() }
    pub fn b(&self) -> f32 { self.z() }

    pub fn set(&mut self, idx: usize, value: f32) {
        self.0 = unsafe { self.0.replace_unchecked(idx, value) }
    }

    pub fn get(&self, idx: usize) -> f32 {
        unsafe { self.0.extract_unchecked(idx) }
    }

    pub fn len(&self) -> f32 {
        // let e = &self.0;
        self.squared_len().sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        // let e = &self.0;
        // e[0] * e[0] + e[1] * e[1] + e[2] * e[2]
        // self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
        (self.0 * self.0).sum()
    }

    pub fn unit(self) -> Vec3 {
        self / self.len()
    }

    pub fn sqrt(self) -> Vec3 {
        Self(self.0.sqrt())
    }

    pub fn dot(self, other: Vec3) -> f32 {
        (self.0 * other.0).sum()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        let Self(a) = self;
        let Self(b) = other;

        let r1: f32x4 = shuffle!(a, [1, 2, 0, 3]);
        let r2: f32x4 = shuffle!(b, [2, 0, 1, 3]);
        let r3: f32x4 = shuffle!(a, [2, 0, 1, 3]);
        let r4: f32x4 = shuffle!(b, [1, 2, 0, 3]);

        Self((r1 * r2) - (r3 * r4))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;
    use super::Vec3;

    #[bench]
    fn dot(bencher: &mut Bencher) {
        let v1 = Vec3::random();

        let vecs = std::iter::repeat_with(Vec3::random)
            .take(10_000)
            .collect::<Vec<_>>();

        bencher.iter(move ||
            vecs.iter()
                .fold(0., |s, &v2| s + v1.dot(v2))
        )
    }

    #[bench]
    fn cross(bencher: &mut Bencher) {
        let v1 = Vec3::random();

        let vecs = std::iter::repeat_with(Vec3::random)
            .take(10_000)
            .collect::<Vec<_>>();

        bencher.iter(move ||
            vecs.iter()
                .fold(0., |s, &v2| s + v1.cross(v2).squared_len())
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0)
        // Vec3([
        //     self.x() + rhs.x(),
        //     self.y() + rhs.y(),
        //     self.z() + rhs.z(),
        // ])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0)
        // Vec3([
        //     self.x() - rhs.x(),
        //     self.y() - rhs.y(),
        //     self.z() - rhs.z(),
        // ])
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0)
        // Vec3([
        //     self.x() * rhs.x(),
        //     self.y() * rhs.y(),
        //     self.z() * rhs.z(),
        // ])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs)
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.0 /= rhs.0
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0)
    }
}

// impl Index<usize> for Vec3 {
//     type Output = f32;

//     fn index(&self, idx: usize) -> &Self::Output {
//         &self.0.extract(idx)
//     }
// }

// impl IndexMut<usize> for Vec3 {
//     fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
//         &mut self.0.extract(idx)
//     }
// }
