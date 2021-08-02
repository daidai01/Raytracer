#![allow(non_snake_case)]
#![warn(unused_imports)]

mod aabb;
mod bvh_static;
mod camera;
mod cornell_box_static;
mod hittable_static;
mod hittable_list;
mod material_static;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rectangle_static;
mod scene;
mod sphere_static;
mod texture_static;
#[allow(clippy::float_cmp)]
mod vec3;

pub use camera::Camera;
pub use hittable_list::HittableList;
pub use hittable_static::Hittable;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use material_static::Lambertian;
pub use pdf::{MixturePDF, HittablePDF, PDF};
use rand::{thread_rng, Rng};
pub use ray::Ray;
pub use rectangle_static::xzRect;
pub use sphere_static::Sphere;
pub use texture_static::SolidColor;
use std::sync::Arc;
pub use vec3::Vec3;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub const INF: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

fn main() {
    //image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width: u32 = 400;
    let mut samples_per_pixel = 100;
    let max_depth = 50;

    //world
    let mut world = HittableList::new();
    let mut lookfrom = Vec3::zero();
    let mut lookat = Vec3::zero();
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = Vec3::zero();
    let mut lights = HittableList::new();
    // let mut lights_arc: Arc<dyn Hittable> = Arc::new(HittableList::new());
    match 6 {
        1 => {
            world = scene::random_scene();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::zero();
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = scene::two_spheres();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::zero();
            vfov = 20.0;
        }
        3 => {
            world = scene::two_perlin_spheres();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::zero();
            vfov = 20.0;
        }
        4 => {
            world = scene::earth();
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            background = Vec3::new(0.7, 0.8, 1.0);
            vfov = 20.0;
        }
        5 => {
            world = scene::simple_light();
            samples_per_pixel = 400;
            background = Vec3::zero();
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            world = scene::cornell_box();
            lights.add(Arc::new(xzRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                554.0,
                Lambertian::new(SolidColor::new_with_vec(Vec3::zero())),
            )));
            lights.add(Arc::new(Sphere::new(
                Vec3::new(190.0, 90.0, 190.0),
                90.0,
                Lambertian::new(SolidColor::new_with_vec(Vec3::zero())),
            )));
            // lights_arc = Arc::new(lights);
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 1000; //todo change
            background = Vec3::zero();
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = scene::cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = scene::final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 1000; //10,000 is too big
            background = Vec3::zero();
            lookfrom = Vec3::new(478.0, 278.0, -600.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }

    //camera
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    //render
    let n_jobs: usize = 32;
    let n_workers: usize = 8;
    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);

    let mut result: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(n_jobs as u64);

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
        let light_ptr = lights.clone();
        pool.execute(move || {
            let row_begin = image_height as usize * i / n_jobs;
            let row_end = image_height as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width, render_height as u32);
            for x in 0..image_width {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let mut color = Vec3::zero();
                    for s in 0..samples_per_pixel {
                        let u = (x as f64 + random_0_1()) / (image_width - 1) as f64;
                        let v = (image_height as f64 - y as f64 + random_0_1()) / (image_height - 1) as f64;
                        let r = cam.get_ray(u, v);
                        color += ray_color(&r, &background, &world_ptr, &Arc::new(light_ptr.clone()), max_depth);
                    }
                    let pixel = img.get_pixel_mut(x, img_y as u32);
                    write_color(&mut color, samples_per_pixel, pixel);
                }
            }
            tx.send((row_begin..row_end, img)).expect("failed to send result");
        });
    }

    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width {
                let row = row as u32;
                let idx = idx as u32;
                *result.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
            }
        }
        bar.inc(1);
    }
    result.save("output/test.png").unwrap();
    bar.finish();

    // let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    // let bar = ProgressBar::new(image_width as u64);
    //
    // for x in 0..image_width {
    //     for y in 0..image_height {
    //         let mut color = Vec3::zero();
    //         for s in 0..samples_per_pixel {
    //             let u = (x as f64 + random_0_1()) / (image_width - 1) as f64;
    //             let v = (y as f64 + random_0_1()) / (image_height - 1) as f64;
    //             let r = cam.get_ray(u, v);
    //             color += ray_color(&r, &background, &world, &Arc<HittableList>, max_depth);
    //         }
    //         let pixel = img.get_pixel_mut(x, image_height - 1 - y);
    //         write_color(&mut color, samples_per_pixel, pixel);
    //     }
    //     bar.inc(1);
    // }
    //
    // img.save("output/test.png").unwrap();
    // bar.finish();
}

//utility functions
fn ray_color(
    r: &Ray,
    background: &Vec3,
    world: &HittableList,
    lights: &Arc<HittableList>,
    depth: i32,
) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    return if let Some(rec) = world.hit(r, 0.001, INF) {
        let emitted = rec.mat_ptr.emitted(r, &rec, rec.u, rec.v, &rec.p);
        if let Some(s_rec) = rec.mat_ptr.scatter(r, &rec) {
            if s_rec.is_specular {
                Vec3::elemul(
                    s_rec.attenuation,
                    ray_color(&s_rec.specular_ray, background, world, lights, depth - 1),
                )
            } else {
                let light_ptr = Arc::new(HittablePDF::new(lights, &rec.p));
                let p = MixturePDF::new(light_ptr, s_rec.pdf_ptr);
                let scattered = Ray::new(rec.p, p.generate(), r.tm);
                let pdf_val = p.value(&scattered.dir);
                emitted
                    + Vec3::elemul(
                    s_rec.attenuation * rec.mat_ptr.scattering_pdf(r, &rec, &scattered),
                    ray_color(&scattered, background, world, lights, depth - 1) / pdf_val,
                )
            }
        } else {
            emitted
        }
    } else {
        background.clone()
    };
}

fn write_color(color: &mut Vec3, samples_per_pixel: i32, pixel: &mut image::Rgb<u8>) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;
    if r != r { //deal with NAN
        r = 0.0;
    }
    if g != g {
        g = 0.0;
    }
    if b != b {
        b = 0.0;
    }
    let scale = 1.0 / samples_per_pixel as f64;
    r = clamp((scale * r).sqrt(), 0.0, 0.999) * 256.0;
    g = clamp((scale * g).sqrt(), 0.0, 0.999) * 256.0;
    b = clamp((scale * b).sqrt(), 0.0, 0.999) * 256.0;
    *pixel = image::Rgb([r as u8, g as u8, b as u8]);
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn random_0_1() -> f64 {
    //return an f64 in [0,1)
    thread_rng().gen::<f64>()
}

fn random_min_max(min: f64, max: f64) -> f64 {
    //return an f64 in [min,max)
    thread_rng().gen_range(min..max)
}

fn random_int(min: i32, max: i32) -> i32 {
    //return an i32 in[min,max)
    random_min_max(min as f64, max as f64 + 1.0) as i32
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/*
#![allow(non_snake_case)]
#![warn(unused_imports)]
mod aabb;
mod bvh;
mod camera;
mod cornell_box;
mod hittable;
mod hittable_list;
mod material;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rectangle;
mod sphere;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
pub use aabb::AABB;
pub use bvh::bvhNode;
pub use camera::Camera;
pub use cornell_box::Box;
pub use hittable::{ConstantMedium, FlipFace, HitRecord, Hittable, RotateY, Translate};
pub use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
pub use onb::ONB;
pub use pdf::{CosinePDF, HittablePDF, MixturePDF, PDF};
use rand::{thread_rng, Rng};
pub use ray::Ray;
pub use rectangle::{xyRect, xzRect, yzRect};
pub use sphere::{MovingSphere, Sphere};
pub use texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
use std::sync::Arc;
pub use vec3::Vec3;
pub const INF: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;
fn main() {
    //image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width: u32 = 400;
    let mut samples_per_pixel = 100;
    let max_depth = 50;
    //world
    let mut world = HittableList::new();
    let mut lookfrom = Vec3::zero();
    let mut lookat = Vec3::zero();
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = Vec3::zero();
    let mut lights = HittableList::new();
    let mut lights_arc: Arc<dyn Hittable> = Arc::new(HittableList::new());
    match 6 {
        1 => {
            world = random_scene();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::zero();
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::zero();
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::zero();
            vfov = 20.0;
        }
        4 => {
            world = earth();
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            background = Vec3::new(0.7, 0.8, 1.0);
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Vec3::zero();
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            world = cornell_box();
            lights.add(Arc::new(xzRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                554.0,
                Arc::new(Lambertian::new_with_vec(Vec3::zero())),
            )));
            lights.add(Arc::new(Sphere::new(
                Vec3::new(190.0, 90.0, 190.0),
                90.0,
                Arc::new(Lambertian::new_with_vec(Vec3::zero())),
            )));
            lights_arc = Arc::new(lights);
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 1000; //todo change
            background = Vec3::zero();
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 1000; //todo change
            background = Vec3::zero();
            lookfrom = Vec3::new(478.0, 278.0, -600.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }
    //camera
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    //render
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    for x in 0..image_width {
        for y in 0..image_height {
            let mut color = Vec3::zero();
            for s in 0..samples_per_pixel {
                let u = (x as f64 + random_0_1()) / (image_width - 1) as f64;
                let v = (y as f64 + random_0_1()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                let test = ray_color(&r, &background, &world, &lights_arc, max_depth);
                // println!("{:?}", test);
                color += test;
            }
            // println!("{:?}", color);
            let pixel = img.get_pixel_mut(x, image_height - 1 - y);
            write_color(&mut color, samples_per_pixel, pixel);
        }
        bar.inc(1);
    }
    img.save("output/test.png").unwrap();
    bar.finish();
}
//utility functions
fn ray_color(
    r: &Ray,
    background: &Vec3,
    world: &HittableList,
    lights: &Arc<dyn Hittable>,
    depth: i32,
) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    return if let Some(rec) = world.hit(r, 0.001, INF) {
        let emitted = rec.mat_ptr.emitted(r, &rec, rec.u, rec.v, &rec.p);
        if let Some(s_rec) = rec.mat_ptr.scatter(r, &rec) {
            if s_rec.is_specular {
                Vec3::elemul(
                    s_rec.attenuation,
                    ray_color(&s_rec.specular_ray, background, world, lights, depth - 1),
                )
            } else {
                let light_ptr = Arc::new(HittablePDF::new(lights, &rec.p));
                let p = MixturePDF::new(light_ptr, s_rec.pdf_ptr);
                let scattered = Ray::new(rec.p, p.generate(), r.tm);
                let pdf_val = p.value(&scattered.dir);
                emitted
                    + Vec3::elemul(
                    s_rec.attenuation * rec.mat_ptr.scattering_pdf(r, &rec, &scattered),
                    ray_color(&scattered, background, world, lights, depth - 1) / pdf_val,
                )
            }
        } else {
            emitted
        }
    } else {
        background.clone()
    };
}
fn write_color(color: &mut Vec3, samples_per_pixel: i32, pixel: &mut image::Rgb<u8>) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;
    if r != r {
        r = 0.0;
    }
    if g != g {
        g = 0.0;
    }
    if b != b {
        b = 0.0;
    }
    let scale = 1.0 / samples_per_pixel as f64;
    r = clamp((scale * r).sqrt(), 0.0, 0.999) * 256.0;
    g = clamp((scale * g).sqrt(), 0.0, 0.999) * 256.0;
    b = clamp((scale * b).sqrt(), 0.0, 0.999) * 256.0;
    *pixel = image::Rgb([r as u8, g as u8, b as u8]);
}
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
fn random_0_1() -> f64 {
    //return an f64 in [0,1)
    thread_rng().gen::<f64>()
}
fn random_min_max(min: f64, max: f64) -> f64 {
    //return an f64 in [min,max)
    thread_rng().gen_range(min..max)
}
fn random_int(min: i32, max: i32) -> i32 {
    //return an i32 in[min,max)
    random_min_max(min as f64, max as f64 + 1.0) as i32
}
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_with_col(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_with_arc(checker)),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_0_1();
            let center = Vec3::new(
                a as f64 + 0.9 * random_0_1(),
                0.2,
                b as f64 + 0.9 * random_0_1(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Vec3::elemul(Vec3::random_0_1(), Vec3::random_0_1());
                    let sphere_material = Arc::new(Lambertian::new_with_vec(albedo));
                    let center2 = center + Vec3::new(0.0, random_min_max(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::random_min_max(0.5, 1.0);
                    let fuzz = random_min_max(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new_with_vec(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}
fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_with_col(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_with_arc(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_with_arc(checker.clone())),
    )));
    objects
}
fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_with_arc(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_with_arc(pertext.clone())),
    )));
    objects
}
fn earth() -> HittableList {
    let mut objects = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_with_arc(earth_texture));
    objects.add(Arc::new(Sphere::new(Vec3::zero(), 2.0, earth_surface)));
    objects
}
fn simple_light() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_with_arc(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_with_arc(pertext.clone())),
    )));
    let difflight = Arc::new(DiffuseLight::new_with_vec(Vec3::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(xyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objects
}
fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian::new_with_vec(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_with_vec(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_with_vec(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_with_vec(Vec3::new(15.0, 15.0, 15.0)));
    objects.add(Arc::new(yzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(yzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(FlipFace::new(Arc::new(xzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )))));
    objects.add(Arc::new(xzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(xzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(xyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let mut box1: Arc<dyn Hittable> = Arc::new(Box::new(
        Vec3::zero(),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    // let aluminum = Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0));
    // let mut box1: Arc<dyn Hittable> = Arc::new(Box::new(
    //     Vec3::zero(),
    //     Vec3::new(165.0, 330.0, 165.0),
    //     aluminum,
    // ));
    box1 = Arc::new(RotateY::new(&box1, 15.0));
    box1 = Arc::new(Translate::new(&box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);
    // let mut box2: Arc<dyn Hittable> = Arc::new(Box::new(
    //     Vec3::zero(),
    //     Vec3::new(165.0, 165.0, 165.0),
    //     white.clone(),
    // ));
    // box2 = Arc::new(RotateY::new(&box2, -18.0));
    // box2 = Arc::new(Translate::new(&box2, Vec3::new(130.0, 0.0, 65.0)));
    // objects.add(box2);
    let glass = Arc::new(Dielectric::new(1.5));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));
    objects
}
fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian::new_with_vec(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_with_vec(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_with_vec(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_with_vec(Vec3::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(yzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(yzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(xzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Arc::new(xzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(xzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(xyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let mut box1: Arc<dyn Hittable> = Arc::new(Box::new(
        Vec3::zero(),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(&box1, 15.0));
    box1 = Arc::new(Translate::new(&box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(Arc::new(ConstantMedium::new_with_col(
        box1,
        0.01,
        Vec3::zero(),
    )));
    let mut box2: Arc<dyn Hittable> = Arc::new(Box::new(
        Vec3::zero(),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(&box2, -18.0));
    box2 = Arc::new(Translate::new(&box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(Arc::new(ConstantMedium::new_with_col(
        box2,
        0.01,
        Vec3::ones(),
    )));
    objects
}
fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new_with_vec(Vec3::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as f64 * w;
            let x1 = x0 + w;
            let y1 = random_min_max(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Arc::new(Box::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let mut objects = HittableList::new();
    objects.add(Arc::new(bvhNode::new_with_list(&mut boxes1, 0.0, 1.0)));
    let light = Arc::new(DiffuseLight::new_with_vec(Vec3::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(xzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));
    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new_with_vec(Vec3::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    )));
    let mut boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new_with_col(
        boundary,
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new(
        Vec3::zero(),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new_with_col(
        boundary,
        0.0001,
        Vec3::ones(),
    )));
    let emat = Arc::new(Lambertian::new_with_arc(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_with_arc(pertext)),
    )));
    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new_with_vec(Vec3::new(0.73, 0.73, 0.73)));
    for i in 0..1000 {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::random_min_max(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    let arc_bvh: Arc<dyn Hittable> = Arc::new(bvhNode::new_with_list(&mut boxes2, 0.0, 1.0));
    let arc_rotate: Arc<dyn Hittable> = Arc::new(RotateY::new(&arc_bvh, 15.0));
    objects.add(Arc::new(Translate::new(
        &arc_rotate,
        Vec3::new(-100.0, 270.0, 395.0),
    )));
    objects
}
*/