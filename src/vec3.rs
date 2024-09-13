use rand::prelude::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Vector = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: rng.gen::<f32>(),
        }
    }

    pub fn random_in(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_normalized() -> Self {
        loop {
            let p = Self::random_in(-1., 1.);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                break p.normalize();
            }
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn magnitude(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negate() {
        let a = Vec3::new(1., -2., 3.5);

        assert_eq!(-a, Vec3::new(-1., 2., -3.5));
    }

    #[test]
    fn add() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(1., -3., 4.);

        assert_eq!(a + b, Vec3::new(2., -1., 7.));
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn subtract() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(1., -3., 4.);

        assert_eq!(a - b, Vec3::new(0., 5., -1.));
        assert_eq!(b - a, Vec3::new(0., -5., 1.));
    }

    #[test]
    fn multiply() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(1., -3., 4.);

        assert_eq!(a * 3.5, Vec3::new(3.5, 7., 10.5));
        assert_eq!(a * b, Vec3::new(1., -6., 12.));
    }

    #[test]
    fn divide() {
        let a = Vec3::new(1., 2., 3.);

        assert_eq!(a / 2.0, Vec3::new(0.5, 1., 1.5));
    }

    #[test]
    fn length_squared() {
        let a = Vec3::new(1., 2., 3.);

        assert_eq!(a.length_squared(), 14.0);
    }

    #[test]
    fn magnitude() {
        let a = Vec3::new(1., 2., 3.);

        assert_eq!(a.magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn normalize() {
        let a = Vec3::new(1., 2., 3.);
        let r14 = 14.0_f32.sqrt();

        assert_eq!(a.normalize(), Vec3::new(1.0 / r14, 2.0 / r14, 3.0 / r14));
    }

    #[test]
    fn dot() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(2., 3., 4.);

        assert_eq!(Vec3::dot(a, b), 20.0);
    }

    #[test]
    fn cross() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(2., 3., 4.);

        assert_eq!(Vec3::cross(a, b), Vec3::new(-1., 2., -1.));
        assert_eq!(Vec3::cross(b, a), Vec3::new(1., -2., 1.));
    }
}
