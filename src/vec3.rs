pub type F = f64;
#[derive(Copy, Clone, Debug)]
pub struct Vec3(F, F, F);

pub type Color = Vec3;
pub type Point3 = Vec3;

use crate::random::random_float_in;

impl Vec3 {
    pub fn new(a: F, b: F, c: F) -> Self {
        Self(a, b, c)
    }
    pub fn x(self) -> F {
        self.0
    }
    pub fn y(self) -> F {
        self.1
    }
    pub fn z(self) -> F {
        self.2
    }

    pub fn length(self) -> F {
        self.length_squared().sqrt()
    }
    pub fn length_squared(self) -> F {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(self, o: Self) -> F {
        self.0 * o.0 + self.1 * o.1 + self.2 * o.2
    }
    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
    pub fn random() -> Self {
        Self(random_float(), random_float(), random_float())
    }
    pub fn random_in(min: F, max: F) -> Self {
        Self(
            random_float_in(min, max),
            random_float_in(min, max),
            random_float_in(min, max),
        )
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_in(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s)
    }

    pub fn reflect(self, other: Self) -> Self {
        self - other * self.dot(other) * 2.0
    }
}

use std::io::Write;

use crate::random::random_float;

pub fn write_color<T: Write>(stream: &mut T, pixel_color: Color, samples_per_pixel: i32) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as F;
    let r = (r * scale).sqrt();
    let g = (g * scale).sqrt();
    let b = (b * scale).sqrt();

    write!(
        stream,
        "{} {} {}\n",
        ((r.clamp(0.0, 0.999)) * 256.0) as i32,
        ((g.clamp(0.0, 0.999)) * 256.0) as i32,
        ((b.clamp(0.0, 0.999)) * 256.0) as i32
    )
    .unwrap();
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = F;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Mul<F> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::Div<F> for Vec3 {
    type Output = Self;
    fn div(self, rhs: F) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}
