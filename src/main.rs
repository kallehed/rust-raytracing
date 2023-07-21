use material::Material;
use ray::Ray;

use crate::{
    camera::Camera,
    hittable::Hittable,
    hittable_list::HittableList,
    material::{Metal, Lambertian},
    random::random_float,
    sphere::Sphere,
    vec3::{write_color, Color, Point3, Vec3, F},
};

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod random;
mod ray;
mod sphere;
mod vec3;

fn ray_color<T: Hittable>(ray: Ray, world: &T, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    match world.hit(ray, 0.001, F::INFINITY) {
        Some(rec) => {
            match rec.mat.scatter(ray, rec) {
                Some((attenuation, scattered)) => {
                    return attenuation * ray_color(scattered, world, depth - 1);
                }
                None => {
                    return Color::default();
                    // return Color::new(1.0,0.0,0.0);
                }
            };
        }
        None => (),
    }
    let unit_dir = ray.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as F / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    };
    let material_left = Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    };
    let material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    };

    // World
    let mut world = HittableList::default();
    world.add(
        (Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            mat: material_ground.into(),
        })
        .into(),
    );
    world.add(
        (Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            mat: material_center.into(),
        })
        .into(),
    );
    world.add(
        (Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            mat: material_left.into(),
        })
        .into(),
    );
    world.add(
        (Sphere {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            mat: material_right.into(),
        })
        .into(),
    );

    // Camera
    let cam = Camera::new();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for i in (0..image_height).rev() {
        eprint!("{i} ");

        for j in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (j as F + random_float()) / (image_width - 1) as F;
                let v = (i as F + random_float()) / (image_height - 1) as F;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            write_color(&mut std::io::stdout(), pixel_color, samples_per_pixel);
        }
    }
}
