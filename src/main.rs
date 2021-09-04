mod vec3;
use crate::vec3::Vec3;
const NX: i32 = 400;
const NY: i32 = 200;

fn main() {
    
    let a = Vec3 {e: [1.0, 2.0, 3.0]};
    let b = Vec3 {e: [4.0, 5.0, 6.0]};

    let n: f64 = 2.0;

    println!("Vector a: {:?}", a);
    println!("Vector b: {:?}", b);

    println!("a+b: {:?}", a + b);
    println!("b-a: {:?}", b - a);

    println!("a * b: {:?}", a * b);
    println!("a * n: {:?}", a * n);

    println!("b / n: {:?}", b / n);
    
    println!("a cross b: {:?}", a.cross(&b));
    println!("a dot b: {}", a.dot(&b));

    /* println!("P3");
    println!("{} {}", NX, NY);
    println!("255");

    for i in (0..NY-1).rev() {
        for j in 0..NX {
            let r = (j as f32) / (NX as f32);
            let g = (i as f32) / (NY as f32);
            let b = 0.2;
            let ir = (255.99*r) as i32;
            let ig = (255.99*g) as i32;
            let ib = (255.99*b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }  */ 
}

