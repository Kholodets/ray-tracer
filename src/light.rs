use crate::Material;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::HitRecord;

pub struct Light {
    pub color: Vec3
}

impl Material for Light {
    fn albedo(&self, _hr: &HitRecord, _time: u32) -> Vec3 {
        Vec3 {e: [0.0, 0.0, 0.0]}
    }

    fn scatter(&self, _ray: &Ray, _hr: &HitRecord) -> Option<Ray> {
        None
    }

    fn absorb(&self) -> Vec3 {
        self.color
    }
}
