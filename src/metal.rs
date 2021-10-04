use crate::Material;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::ray::Ray;

pub struct Metal {
    pub color: Vec3
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Ray> {
        let reflected = Vec3::reflect(ray.dir().unit_vector(), hr.norm);
        if reflected.dot(&hr.norm) > 0.0 {
            Some(Ray::new(hr.p, reflected))
        } else {
            None
        }
    }

    fn albedo(&self, hr: &HitRecord) -> Vec3 {
        self.color
    }

    fn absorb(&self) -> Vec3 {
            Vec3 {e: [0.0, 0.0, 0.0]}
    }
}
