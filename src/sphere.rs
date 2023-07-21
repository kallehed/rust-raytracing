use crate::{
    hittable::{HitRecord, Hittable},
    material::MaterialEnum,
    ray::Ray,
    vec3::{Point3, F},
};

pub struct Sphere {
    pub center: Point3,
    pub radius: F,
    pub mat: MaterialEnum,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: F, t_max: F) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let rec_p = r.at(root);
        Some(
            HitRecord::new(rec_p, (rec_p - self.center) / self.radius, self.mat, root, false)
                .set_face_normal(r, (rec_p - self.center) / self.radius),
        )
    }
}
