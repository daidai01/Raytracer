use crate::onb::ONB;
use crate::vec3::Vec3;
use crate::{random_0_1, Hittable, PI};

pub trait PDF: Sync + Send {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

#[derive(Clone)]
pub struct CosinePDF {
    pub uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> Self {
        let mut _uvw = ONB::new();
        _uvw.build_from_w(w.clone());
        Self { uvw: _uvw }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.unit() * self.uvw.w;
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_with_vec(Vec3::random_cosine_direction())
    }
}

#[derive(Clone)]
pub struct HittablePDF<T: Hittable> {
    pub orig: Vec3,
    pub ptr: T,
}

impl<T: Hittable> HittablePDF<T> {
    pub fn new(p: T, origin: &Vec3) -> Self {
        Self {
            ptr: p,
            orig: origin.clone(),
        }
    }
}

impl<T: Hittable> PDF for HittablePDF<T> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.orig, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.orig)
    }
}

#[derive(Clone)]
pub struct MixturePDF<U: PDF, V: PDF> {
    p0: U,
    p1: V,
}

impl<U: PDF, V: PDF> MixturePDF<U, V> {
    pub fn new(p0: U, p1: V) -> Self {
        Self { p0, p1 }
    }
}

impl<U: PDF, V: PDF> PDF for MixturePDF<U, V> {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_0_1() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
