use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::random::random_float;
use crate::random::random_float_in;
use crate::sphere::Sphere;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::Ray;
use crate::F;

#[enum_dispatch::enum_dispatch(Hittable)]
pub enum HittableEnum {
    Sphere,
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<HittableEnum>,
}

impl HittableList {
    pub fn add(&mut self, e: HittableEnum) {
        self.objects.push(e);
    }

    pub fn random_scene() -> Self {
        let mut world = HittableList::default();

        let ground_material = Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }
        .into();

        world.add(
            Sphere {
                center: Point3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                mat: ground_material,
            }
            .into(),
        );
        for a in (-11)..11 {
            for b in (-11)..11 {
                let choose_mat = random_float();
                let center = Point3::new(
                    a as F + 0.9 * random_float(),
                    0.2,
                    b as F + 0.9 * random_float(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let albedo = Color::random() * Color::random();
                        let sphere_material = Lambertian { albedo }.into();
                        world.add(
                            Sphere {
                                center,
                                radius: 0.2,
                                mat: sphere_material,
                            }
                            .into(),
                        );
                    } else if choose_mat < 0.95 {
                        let albedo = Color::random_in(0.5, 1.0);
                        let fuzz = random_float_in(0.0, 0.5);
                        let mat = Metal::new(albedo, fuzz).into();
                        world.add(
                            Sphere {
                                center,
                                radius: 0.2,
                                mat,
                            }
                            .into(),
                        );
                    } else {
                        let mat = Dielectric::new(1.5).into();
                        world.add(
                            Sphere {
                                center,
                                radius: 0.2,
                                mat,
                            }
                            .into(),
                        );
                    }
                }
            }
        }

        world.add(Sphere {center: Point3::new(0.0,1.0,0.0), radius: 1.0, mat: Dielectric::new(1.5).into()}.into());

        world.add(Sphere {center: Point3::new(-4.0,1.0,0.0), radius: 1.0, mat: Lambertian {albedo: Color::new(0.4,0.2,0.1)}.into()}.into());

        world.add(Sphere {center: Point3::new(4.0,1.0,0.0), radius: 1.0, mat: Metal::new(Color::new(0.7,0.6,0.5), 0.0).into()}.into());

        world
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: crate::ray::Ray,
        t_min: crate::vec3::F,
        t_max: crate::vec3::F,
    ) -> Option<crate::hittable::HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for object in self.objects.iter() {
            let asd = object.hit(r, t_min, closest_so_far);
            match asd {
                Some(x) => {
                    closest_so_far = x.t;
                    hit_record = Some(x);
                }
                None => (),
            }
        }
        return hit_record;
    }
}
