use crate::random::random_float;
use crate::vec3::Vec3;
use crate::{hittable::HitRecord, ray::Ray, vec3::Color, F};

#[derive(Copy, Clone)]
#[enum_dispatch::enum_dispatch]
pub enum MaterialEnum {
    Lambertian,
    Metal,
    Dielectric,
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
    fn scatter(&self, _r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_dir)))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: F,
}

impl Metal {
    pub fn new(color: Color, fuzz: F) -> Metal {
        Metal {
            albedo: color,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().unit_vector().reflect(rec.normal);

        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: F, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: F) -> Self {
        Self { ir }
    }
    fn reflectance(cosine: F, ref_idx: F) -> F {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();

        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_float() {
                Vec3::reflect(unit_direction, rec.normal)
            } else {
                Vec3::refract(unit_direction, rec.normal, refraction_ratio)
            };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.p, direction)))
    }
}
