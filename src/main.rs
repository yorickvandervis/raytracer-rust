mod ray;
mod vec3;

use vec3::{Color, Vec3};

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 128;

    // Render
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                (i as f32) / ((IMAGE_WIDTH - 1) as f32),
                (j as f32) / ((IMAGE_HEIGHT - 1) as f32),
                0.25,
            );

            println!("{}", pixel_color.format_color());
        }
    }
}
