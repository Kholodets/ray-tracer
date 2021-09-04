const NX: i32 = 400;
const NY: i32 = 200;

fn main() {
    println!("P3");
    println!("{} {}", NX, NY);
    println!("255");

    let mut i = NY-1;
    
    while i >= 0 {
        let mut j = 0;
        while j < NX {
            let r = (j as f32) / (NX as f32);
            let g = (i as f32) / (NY as f32);
            let b = 0.2;
            let ir = (255.99*r) as i32;
            let ig = (255.99*g) as i32;
            let ib = (255.99*b) as i32;

            println!("{} {} {}", ir, ig, ib);

            j += 1;
        }
        i -= 1;
    }   
}

