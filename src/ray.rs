use crate::vec3::Point3;
use crate::vec3::F;
use crate::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }
    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: F) -> Point3 {
        self.orig + self.dir * t
    }
}
