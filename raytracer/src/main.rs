#![allow(clippy::float_cmp)]
#![allow(clippy::eq_op)]
#![allow(clippy::many_single_char_names)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::let_and_return)]
#![allow(clippy::clone_on_copy)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_clone)]
#![allow(clippy::manual_swap)]
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_map)]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_return)]

mod aabb;
mod bvh_static;
mod camera;
mod cornell_box_static;
mod hittable_list;
mod hittable_static;
mod material_static;
mod onb;
mod pdf_static;
mod perlin;
mod ray;
mod rectangle_static;
mod scene;
mod sphere_static;
mod texture_static;
mod vec3;

pub use camera::Camera;
pub use hittable_list::HittableList;
pub use hittable_static::Hittable;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use material_static::Lambertian;
pub use pdf_static::{HittablePDF, MixturePDF, PDF};
use rand::{thread_rng, Rng};
pub use ray::Ray;
pub use rectangle_static::XzRect;
pub use sphere_static::Sphere;
use std::ops::Deref;
use std::sync::mpsc::channel;
use std::sync::Arc;
pub use texture_static::SolidColor;
use threadpool::ThreadPool;
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
    match 0 {
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
            lights.add(Arc::new(XzRect::new(
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
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 1000;
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
        8 => {
            world = scene::final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 1000; //10,000 is too big
            background = Vec3::zero();
            lookfrom = Vec3::new(578.0, 0.0, -800.0);
            lookat = Vec3::new(378.0, 200.0, 0.0);
            vfov = 40.0;
        }
        9 => {
            world = scene::my_scene();
            lights.add(Arc::new(XzRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                354.0,
                Lambertian::new(SolidColor::new_with_vec(Vec3::new(7.0, 7.0, 7.0))),
            )));
            aspect_ratio = 2.0;
            image_width = 800;
            samples_per_pixel = 100;
            background = Vec3::new(0.7, 0.8, 1.0);
            // background = Vec3::zero();
            lookfrom = Vec3::new(478.0, 178.0, -800.0);
            lookat = Vec3::new(378.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = scene::solar_system();
            aspect_ratio = 1.5;
            image_width = 900;
            samples_per_pixel = 10;
            background = Vec3::new(0.90, 0.90, 0.97);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
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
                        let v = (image_height as f64 - y as f64 + random_0_1())
                            / (image_height - 1) as f64;
                        let r = cam.get_ray(u, v);
                        color += ray_color(
                            &r,
                            &background,
                            &world_ptr,
                            &Arc::new(light_ptr.clone()),
                            max_depth,
                        );
                    }
                    let pixel = img.get_pixel_mut(x, img_y as u32);
                    write_color(&mut color, samples_per_pixel, pixel);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
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
    result.save("output/my_scene.png").unwrap();
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
                let light_ptr = HittablePDF::new(lights.deref().clone(), &rec.p);
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
    //deal with NAN
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
    //return an i32 in[min,max]
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
