use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::Material;
use crate::HitRecord;

pub struct Lambertian {
    pub color: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hr: &HitRecord) -> Option<Ray> {
        let sd = hr.norm + Vec3::random_unit_vector();

        if sd.near_zero() {
            Some(Ray::new(hr.p, hr.norm))
        } else {
            Some(Ray::new(hr.p, sd))
        }
    }

    fn albedo(&self, _hr: &HitRecord, _time: u32) -> Vec3 {
        self.color
    }

    fn absorb(&self) -> Vec3 {
        Vec3 {e: [0.0, 0.0, 0.0]}
    }
}
