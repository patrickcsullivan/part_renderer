mod color;
mod interaction;
mod ray;
mod shape;

use image::ImageBuffer;

fn main() {
    println!("Hello, world!");
    let img = ImageBuffer::from_fn(512, 512, |x, y| {
        let s = (x + y) as f32 / 1024.0;
        let c: image::Rgb<u8> = color::Rgb::new(s, s, s).into();
        c
    });
    let _ = img.save("test.png");
}
