mod vec3;
mod ray;
mod sphere;
mod scene;
mod camera;
mod lambertian;
mod metal;
mod light;
mod mtexture;
use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::lambertian::Lambertian;
use crate::light::Light;
use crate::metal::Metal;
use crate::mtexture::Mtexture;
use std::io::Write;
use std::io::stderr;
extern crate rand;
use rand::Rng;
use std::num;
extern crate image;

const X_RES: i32 = 720;
const Y_RES: i32 = 480;
const ORIGIN_VEC: Vec3 = Vec3 {e: [0.0,0.0,0.0]};
const TEST_SPHERE: Sphere = Sphere {center: Vec3 {e: [-0.5, 0.0, -1.5]}, radius: 0.5, mat: &MIRROR};
const GROUND: Sphere = Sphere {center: Vec3 {e: [0.0, -1000.5, -1.0]}, radius: 1000.0, mat: &MIRROR};
const AA_SAMPLES: i32 = 70;
const REF_DEPTH: i32 = 30;
const GAMMA: f64 = 4.0;
const LAMB_RED: Lambertian = Lambertian {color: Vec3 {e: [0.5, 0.0, 0.0]}};
const LAMB_GREY: Lambertian = Lambertian {color: Vec3 {e:[0.5, 0.5, 0.5]}};
const MIRROR: Metal = Metal {color: Vec3 {e: [0.8, 0.99, 0.99]}};

pub trait Material {
    fn albedo(&self, hr: &HitRecord) -> Vec3;
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Ray>;
    fn absorb(&self) -> Vec3;
}

pub struct HitRecord<'d> {
    mat: &'d Material,
    p: Vec3,
    norm: Vec3,
    t: f64,
    ff: bool
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

fn main() {
    println!("P3");
    println!("{} {}", X_RES, Y_RES);
    println!("255");

    let ar = X_RES as f64 / Y_RES as f64;
   

    let mut rng = rand::thread_rng();

    let vph = 2.0;
    let vpw = ar * vph;
    let fl = 1.0;

    let cam = Camera::new(vph, ar* vph, 1.0);

    let floosh = Mtexture::new("floosh.bmp", 3.0);
    
    let flooshorb = Sphere {
        center: Vec3 {e: [0.0, -0.2, -0.7]},
        radius: 0.2,
        mat: &floosh
    };

    let mut world = Scene {list: vec![&TEST_SPHERE, &GROUND, &flooshorb]};

    world.add(&Sphere {
        center: Vec3 {e: [0.5, 0.0, -1.5]},
        radius: 0.5,
        mat: &LAMB_RED
    });

    /*world.add(&Sphere {
        center: Vec3 {e: [0.0, -0.2, -0.7]},
        radius: 0.2,
        mat: &Lambertian {color: Vec3 {e: [0.0, 0.0, 0.8]}}
    });*/

    world.add(&Sphere {
        center: Vec3 {e: [75.0, 75.0, 0.0]},
        radius: 100.0,
        mat: &Light {color: Vec3 {e: [1.0, 1.0, 0.8]}}
    });

    for i in (0..Y_RES).rev() {
        eprint!("\rLines remaining: {} ({:.2}%) ", i, 100.0*((Y_RES-i) as f32)/(Y_RES as f32));
        stderr().flush().expect("failed to flush stderr");
        for j in 0..X_RES {
            let mut color = Vec3 {e: [0.0, 0.0, 0.0]};
            for s in 0..AA_SAMPLES {
                let u = (j as f64 + rng.gen::<f64>()) / (X_RES - 1) as f64;
                let v = (i as f64 + rng.gen::<f64>()) / (Y_RES - 1) as f64;
                let pr = cam.get_ray(u, v);
                let scol = ray_color(&pr, &world, REF_DEPTH);
                color = color + scol;
            }
            
            write_color(color, AA_SAMPLES);
        }
    }
    eprintln!("\nDone!"); 
}

fn ray_color(ray: &Ray, scene: &dyn Object, depth: i32) -> Vec3 {
    if depth <= 0 {
        Vec3 {e: [0.0, 0.0, 0.0]}

    } else {
        let h = scene.hit(ray, 0.001, f64::INFINITY);

        match h {
            Some(hr) => {
                match hr.mat.scatter(ray, &hr) {
                    Some(sr) => ray_color(&sr, scene, depth - 1)*hr.mat.albedo(&hr),
                    None => hr.mat.absorb(),
                }
            },

            None => sky_color(ray),
        }
    }
}

fn sky_color(ray: &Ray) -> Vec3 {
    let t = 0.5*(ray.dir().unit_vector().y() + 1.0);
    (Vec3 {e:[1.0,1.0,1.0]})*(1.0 - t)*0.1 + (Vec3 {e: [0.5, 0.7, 1.0]})*t*0.1
}

fn clamp(n: f64, min: f64, max: f64) -> f64 {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

fn write_color(color: Vec3, samples: i32) {
    let scale = 1.0 / samples as f64;
    let r = clamp((scale*color.x()).powf(1.0/GAMMA), 0.0, 0.999);
    let g = clamp((scale*color.y()).powf(1.0/GAMMA), 0.0, 0.999);
    let b = clamp((scale*color.z()).powf(1.0/GAMMA), 0.0, 0.999);
    println!("{} {} {}", 
             (r*256.0) as i32, 
             (g*256.0) as i32, 
             (b*256.0) as i32);
}


