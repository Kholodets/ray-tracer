use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::Material;
use crate::HitRecord;
use image::{RgbImage, Pixel};
use image::open;

pub struct Mtexture {
    pub texture: RgbImage,
    pub x: u32,
    pub gamma: f64
}

impl Mtexture {
    pub fn new(name: &str, g: f64) -> Mtexture {
        let tex = open(name).unwrap().into_rgb8();
        Mtexture {
            x: tex.width(),
            texture: tex,
            gamma: g
        }
    }
}


impl Material for Mtexture {
    fn scatter(&self, _ray: &Ray, hr: &HitRecord) -> Option<Ray> {
        let sd = hr.norm + Vec3::random_unit_vector();

        if sd.near_zero() {
            Some(Ray::new(hr.p, hr.norm))
        } else {
            Some(Ray::new(hr.p, sd))
        }
    }

    fn albedo(&self, hr: &HitRecord, time: u32) -> Vec3 {
        let xpos = ((hr.norm.x()/2.0 + 0.5)*(self.x as f64)) as u32;
        let xp = (xpos + time) % self.x;
        let yp = (((-hr.norm.y())/2.0 + 0.5)*(self.texture.height() as f64)) as u32;

        let pix = (*self.texture.get_pixel(xp, yp)).channels();
        
        Vec3 {e: [(pix[0] as f64 /255.0).powf(self.gamma), (pix[1] as f64 /255.0).powf(self.gamma), (pix[2] as f64 /255.0).powf(self.gamma)]}
    }

    fn absorb(&self) -> Vec3 {
        Vec3 {e: [0.0, 0.0, 0.0]}
    }
}

