use crate::vec3::Vec3;
use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

#[derive(Copy, Clone)]
#[enum_dispatch::enum_dispatch]
pub enum MaterialEnum {
    Lambertian,
    Metal,
}

#[enum_dispatch::enum_dispatch(MaterialEnum)]
pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_dir)))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().unit_vector().reflect(rec.normal);

        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
