use std::char::MAX;

fn main() {
    // Image

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev(){
        for i in 0..IMAGE_WIDTH {
            let r: f32 = i as f32 / IMAGE_WIDTH as f32;
            let g: f32 = j as f32 / IMAGE_HEIGHT as f32;
            let b: f32 = 0.2;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib)
        }
    }
}
