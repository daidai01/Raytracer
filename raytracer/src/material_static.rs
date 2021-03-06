use crate::hittable_static::HitRecord;
use crate::pdf_static::CosinePDF;
use crate::ray::Ray;
use crate::texture_static::Texture;
use crate::vec3::Vec3;
use crate::{random_0_1, PI};
use num_traits::pow;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub specular_ray: Ray,
    pub pdf_ptr: CosinePDF,
    pub is_specular: bool,
}

impl ScatterRecord {
    pub fn new() -> Self {
        Self {
            attenuation: Vec3::zero(),
            specular_ray: Ray::new(Vec3::zero(), Vec3::zero(), 0.0),
            pdf_ptr: CosinePDF::new(&Vec3::ones()),
            is_specular: false,
        }
    }
}

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, r: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, r: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(a: T) -> Self {
        Self { albedo: a }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let s_rec = ScatterRecord {
            // specular_ray: Ray::new(rec.p, direction, r.tm),
            specular_ray: Ray::new(Vec3::zero(), Vec3::zero(), 0.0),
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf_ptr: CosinePDF::new(&rec.normal),
            is_specular: false,
        };
        Some(s_rec)
    }

    fn scattering_pdf(&self, r: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.normal * scattered.dir.unit();
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Vec3, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(r.dir.unit(), rec.normal);
        let s_rec = ScatterRecord {
            specular_ray: Ray::new(
                rec.p,
                reflected + Vec3::random_in_unit_sphere() * self.fuzz,
                0.0,
            ),
            attenuation: self.albedo,
            is_specular: true,
            pdf_ptr: CosinePDF::new(&Vec3::ones()),
        };
        Some(s_rec)
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * pow(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r.dir.unit();
        let mut cos_theta = -unit_direction * rec.normal;
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let flag = refraction_ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, refraction_ratio) > random_0_1();
        let direction = if flag {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
        let s_rec = ScatterRecord {
            attenuation: Vec3::ones(),
            specular_ray: Ray::new(rec.p, direction, r.tm),
            pdf_ptr: CosinePDF::new(&Vec3::ones()),
            is_specular: true,
        };
        Some(s_rec)
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(a: T) -> Self {
        Self { emit: a }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, r: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Vec3::zero()
        }
    }
}

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(a: T) -> Self {
        Self { albedo: a }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let s_rec = ScatterRecord {
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            specular_ray: Ray::new(rec.p, Vec3::random_in_unit_sphere(), r.tm),
            pdf_ptr: CosinePDF::new(&Vec3::ones()),
            is_specular: true,
        };
        Some(s_rec)
    }
}
