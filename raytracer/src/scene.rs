use crate::bvh_static::BvhNode;
use crate::cornell_box_static::Box;
use crate::hittable_list::HittableList;
use crate::hittable_static::{ConstantMedium, FlipFace, Hittable, RotateY, Translate};
use crate::material_static::{Dielectric, DiffuseLight, Isotropic, Lambertian, Metal};
use crate::rectangle_static::{XyRect, XzRect, YzRect};
use crate::sphere_static::{MovingSphere, Sphere};
use crate::texture_static::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use crate::vec3::Vec3;
use crate::{random_0_1, random_int, random_min_max};
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = CheckerTexture::new(
        SolidColor::new_with_col(0.2, 0.3, 0.1),
        SolidColor::new_with_col(0.9, 0.9, 0.9),
    );
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(checker),
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
                    let sphere_material = Lambertian::new(SolidColor::new_with_vec(albedo));
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
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Lambertian::new(SolidColor::new_with_col(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = CheckerTexture::new(
        SolidColor::new_with_col(0.2, 0.3, 0.1),
        SolidColor::new_with_col(0.9, 0.9, 0.9),
    );
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(checker.clone()),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(checker.clone()),
    )));
    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = NoiseTexture::new(4.0);
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(pertext.clone()),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(pertext.clone()),
    )));
    objects
}

pub fn earth() -> HittableList {
    let mut objects = HittableList::new();
    let earth_texture = ImageTexture::new("jpg/earthmap.jpg");
    let earth_surface = Lambertian::new(earth_texture);
    objects.add(Arc::new(Sphere::new(Vec3::zero(), 2.0, earth_surface)));
    objects
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = NoiseTexture::new(4.0);
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(pertext.clone()),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(pertext.clone()),
    )));
    let difflight = DiffuseLight::new(SolidColor::new_with_col(4.0, 4.0, 4.0));
    objects.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Lambertian::new(SolidColor::new_with_col(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new_with_col(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new_with_col(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::new_with_col(15.0, 15.0, 15.0));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(FlipFace::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    ))));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let box1_temp1 = Box::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white.clone());
    // let aluminum = Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0));
    // let mut box1: Arc<dyn Hittable> = Arc::new(Box::new(
    //     Vec3::zero(),
    //     Vec3::new(165.0, 330.0, 165.0),
    //     aluminum,
    // ));
    let box1_temp2 = RotateY::new(box1_temp1, 15.0);
    let box1: Arc<dyn Hittable> =
        Arc::new(Translate::new(box1_temp2, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);
    // let mut box2: Arc<dyn Hittable> = Arc::new(Box::new(
    //     Vec3::zero(),
    //     Vec3::new(165.0, 165.0, 165.0),
    //     white.clone(),
    // ));
    // box2 = Arc::new(RotateY::new(&box2, -18.0));
    // box2 = Arc::new(Translate::new(&box2, Vec3::new(130.0, 0.0, 65.0)));
    // objects.add(box2);
    let glass = Dielectric::new(1.5);
    objects.add(Arc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));
    objects
}

pub fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();
    let red = Lambertian::new(SolidColor::new_with_col(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new_with_col(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new_with_col(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::new_with_col(7.0, 7.0, 7.0));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let box1_temp1 = Box::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box1_temp2 = RotateY::new(box1_temp1, 15.0);
    let box1 = Translate::new(box1_temp2, Vec3::new(265.0, 0.0, 295.0));
    objects.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        Isotropic::new(SolidColor::new_with_vec(Vec3::zero())),
    )));
    let box2_temp1 = Box::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white.clone());
    let box2_temp2 = RotateY::new(box2_temp1, -18.0);
    let box2 = Translate::new(box2_temp2, Vec3::new(130.0, 0.0, 65.0));
    objects.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        Isotropic::new(SolidColor::new_with_vec(Vec3::ones())),
    )));
    objects
}

pub fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Lambertian::new(SolidColor::new_with_col(0.48, 0.83, 0.53));
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
    objects.add(Arc::new(BvhNode::new_with_list(&mut boxes1, 0.0, 1.0)));
    let light = DiffuseLight::new(SolidColor::new_with_col(7.0, 7.0, 7.0));
    objects.add(Arc::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));
    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new(SolidColor::new_with_col(0.7, 0.3, 0.1));
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
        Dielectric::new(1.5),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0),
    )));
    let mut boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    objects.add(Arc::new(boundary.clone()));
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        Isotropic::new(SolidColor::new_with_col(0.2, 0.4, 0.9)),
    )));
    boundary = Sphere::new(Vec3::zero(), 5000.0, Dielectric::new(1.5));
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Isotropic::new(SolidColor::new_with_vec(Vec3::ones())),
    )));
    let emat = Lambertian::new(ImageTexture::new("jpg/earthmap.jpg"));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = NoiseTexture::new(0.1);
    objects.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(pertext),
    )));

    let mut boxes2 = HittableList::new();
    let white = Lambertian::new(SolidColor::new_with_col(0.73, 0.73, 0.73));
    for i in 0..1000 {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::random_min_max(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    let arc_bvh = BvhNode::new_with_list(&mut boxes2, 0.0, 1.0);
    let arc_rotate = RotateY::new(arc_bvh, 15.0);
    objects.add(Arc::new(Translate::new(
        arc_rotate,
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}

pub fn my_scene() -> HittableList {
    let mut objects = HittableList::new();
    let sphereR = 40.0;
    let basepairR = 4.0;
    let upLimit = 2000.0;
    let xPos = -750.0;
    let yPos = 300.0;
    let zPos = 100.0;
    let R = 120.0;
    let spacingAtoms = 100.0;
    let mut t: f64 = 0.0;
    let mut x1 = 0.0;
    objects.add(Arc::new(XyRect::new(
        -1500.0,
        1500.0,
        -1000.0,
        1000.0,
        600.0,
        Lambertian::new(ImageTexture::new("jpg/blue_dna1.jpg")),
    )));
    objects.add(Arc::new(FlipFace::new(XzRect::new(
        413.0,
        243.0,
        427.0,
        232.0,
        599.0,
        DiffuseLight::new(SolidColor::new_with_col(15.0, 15.0, 15.0)),
    ))));
    let metal_vec = generate_color();
    // let green = Lambertian::new(SolidColor::new_with_col(0.12, 0.45, 0.15));
    let p = Isotropic::new(SolidColor::new_with_col(
        128.0 / 256.0,
        170.0 / 256.0,
        255.0 / 256.0,
    ));
    while x1 < upLimit {
        //sphere1
        x1 = t * spacingAtoms + xPos;
        let y1 = R * t.sin() + yPos;
        let z1 = R * t.cos() + zPos;
        objects.add(Arc::new(ConstantMedium::new(
            Sphere::new(Vec3::new(x1, y1, z1), sphereR, Dielectric::new(1.5)),
            0.2,
            p.clone(),
        )));
        // objects.add(Arc::new(Sphere::new(
        //     Vec3::new(x1, y1, z1),
        //     sphereR,
        //     green.clone(),
        // )));
        //sphere2
        let x2 = x1;
        let y2 = -R * t.sin() + yPos;
        let z2 = -R * t.cos() + zPos;
        objects.add(Arc::new(ConstantMedium::new(
            Sphere::new(Vec3::new(x2, y2, z2), sphereR, Dielectric::new(1.5)),
            0.2,
            p.clone(),
        )));
        // objects.add(Arc::new(Sphere::new(
        //     Vec3::new(x2, y2, z2),
        //     sphereR,
        //     green.clone(),
        // )));
        //base-pair
        let distance =
            ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2)).sqrt();
        let num = (distance / basepairR) as i32;
        let r1 = random_int(0, 3) as usize;
        let col1 = metal_vec[r1].clone();
        let mut r2 = 0;
        loop {
            r2 = random_int(0, 3) as usize;
            if r1 != r2 {
                break;
            }
        }
        let col2 = metal_vec[r2].clone();
        for _i in 0..num {
            let n = num as f64;
            let i = _i as f64;
            let x = (i * x1 + (n - i) * x2) / n;
            let y = (i * y1 + (n - i) * y2) / n;
            let z = (i * z1 + (n - i) * z2) / n;
            let col = if _i * 2 < num {
                col1.clone()
            } else {
                col2.clone()
            };
            objects.add(Arc::new(Sphere::new(Vec3::new(x, y, z), basepairR, col)));
        }
        t += 0.5;
    }
    objects
}

fn generate_color() -> Vec<Metal> {
    let mut v: Vec<Metal> = vec![];
    let fuzz = 0.8;
    v.push(Metal::new(Vec3::new(0.0, 1.0, 0.0), fuzz));
    v.push(Metal::new(Vec3::new(1.0, 0.0, 0.0), fuzz));
    v.push(Metal::new(Vec3::new(0.0, 0.0, 1.0), fuzz));
    v.push(Metal::new(Vec3::new(1.0, 1.0, 0.0), fuzz));
    v
}
