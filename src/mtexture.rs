use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::Material;
use crate::HitRecord;
use image::{RgbImage, Rgb};
use image::{GenericImage, GenericImageView, ImageBuffer, open};

pub struct Mtexture {
    pub texture: RgbImage
}

impl Mtexture {
    pub fn new(name: &str) -> Mtexture {
        Mtexture {
            texture: open("floosh.bmp").unwrap().into_rgb8()
        }
    }
}


impl Material for Mtexture {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Ray> {
        let sd = hr.norm + Vec3::random_unit_vector();

        if sd.near_zero() {
            Some(Ray::new(hr.p, hr.norm))
        } else {
            Some(Ray::new(hr.p, sd))
        }
    }

    fn albedo(&self, hr: &HitRecord) -> Vec3 {
        Vec3 {e: [0.0, 0.0, 0.0]}
    }

    fn absorb(&self) -> Vec3 {
        Vec3 {e: [0.0, 0.0, 0.0]}
    }
}
