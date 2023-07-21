use crate::Point3;
use crate::Ray;
use crate::Vec3;
use crate::F;
use crate::material::MaterialEnum;
use enum_dispatch::enum_dispatch;
use crate::material::Lambertian;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: MaterialEnum,
    pub t: F,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, mat: MaterialEnum, t: F, front_face: bool) -> Self {
        Self {
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(self, r: Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        HitRecord {
            p: self.p,
            normal: (if front_face {
                outward_normal
            } else {
                -outward_normal
            }),
            mat: self.mat, 
            t: self.t,
            front_face,
        }
    }
}

impl Default for HitRecord {

    fn default() -> Self {
        Self{p: Point3::default(), normal: Vec3::default(), mat: MaterialEnum::Lambertian(Lambertian{albedo: crate::Color::default()}), t: 0.0, front_face: false}  
    }
}

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, r: Ray, t_min: F, t_max: F) -> Option<HitRecord>;
}
