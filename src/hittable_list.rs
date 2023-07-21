use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::sphere::Sphere;
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
