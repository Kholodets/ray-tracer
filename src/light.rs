use crate::Material;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::HitRecord;

pub struct Light {
    pub color: Vec3
}

impl Material for Light {
    fn albedo(&self) -> Vec3 {
        Vec3 {e: [0.0, 0.0, 0.0]}
    }

    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Ray> {
        None
    }

    fn absorb(&self) -> Vec3 {
        self.color
    }
}
