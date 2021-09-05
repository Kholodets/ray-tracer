mod vec3;
mod ray;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::io::Write;
use std::io::stderr;

const X_RES: i32 = 720;
const Y_RES: i32 = 480;
const ASPECT_RATIO: f64 = (X_RES / Y_RES) as f64;
const ORIGIN_VEC: Vec3 = Vec3 {e: [0.0,0.0,0.0]};

pub trait Material {
    fn albedo(&self) -> Vec3;
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> (Vec3, bool);
}

pub struct HitRecord {
    hit: bool,
    p: Vec3,
    norm: Vec3,
    mat: &Material,
    t: f64,
    ff: bool
}

pub trait Object {
    fn hit(&self, ray: &Ray) -> HitRecord;
    fn material(&self) -> i32;
}

fn main() {

    println!("P3");
    println!("{} {}", X_RES, Y_RES);
    println!("255");

    let vph = 2.0;
    let vpw = ASPECT_RATIO * vph;
    let fl = 1.0;

    let hor = Vec3 {e: [vpw, 0.0, 0.0]};
    let vert = Vec3 {e: [0.0, vph, 0.0]};
    let vp_llc = ORIGIN_VEC - hor / 2.0 - vert / 2.0 - Vec3 {e: [0.0, 0.0, fl]};

    for i in (0..Y_RES).rev() {
        eprint!("\rLines remaining: {} ({}%) ", i, 100.0*((Y_RES) as f32)/(Y_RES as f32));
        stderr().flush().expect("failed to flush stderr");
        for j in 0..X_RES {
            let u = (j as f64) / (X_RES - 1) as f64;
            let v = (i as f64) / (Y_RES - 1) as f64;
            let pr = create_ray(ORIGIN_VEC, vp_llc + hor*u + vert*v - ORIGIN_VEC);

            
            write_color(sky_color(&pr));
        }
    }
    eprintln!("\nDone!");
}

fn sky_color(ray: &Ray) -> Vec3 {
    let t = 0.5*(ray.dir().unit_vector().y() + 1.0);
    (Vec3 {e:[1.0,1.0,1.0]})*(1.0 - t) + (Vec3 {e: [0.5, 0.7, 1.0]})*t
}

fn write_color(color: Vec3) {
    println!("{} {} {}", (color.x()*255.99) as i32, (color.y()*255.99) as i32, (color.z()*255.99) as i32);
}

fn create_ray(o: Vec3, d: Vec3) -> Ray {
    Ray {origin: o, direction: d}
}
