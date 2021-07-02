mod color;
mod interaction;
mod intersection;
mod matrix;
mod ray;
mod shape;
mod transform;
mod vector;

#[cfg(test)]
mod test;

fn main() {
    println!("Hello, world!");
    demo();
}

fn demo() {
    use crate::matrix::identity4;
    use crate::ray::Ray;
    use cgmath::InnerSpace;
    use cgmath::Point3;
    use image::ImageBuffer;

    use crate::shape::{Shape, Sphere};

    // Assume LH coordinate system.
    let ray_origin = Point3::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_width = 7.0;
    let half_width = wall_width / 2.0;

    let canvas_width = 100; // pixels
    let pixel_size = wall_width / canvas_width as f32;

    let identity = identity4();
    let sphere = Sphere::new(&identity, &identity);

    let red = image::Rgb([255_u8, 0_u8, 0_u8]);
    let black = image::Rgb([0_u8, 0_u8, 0_u8]);

    let img = ImageBuffer::from_fn(canvas_width, canvas_width, |x, y| {
        let world_x = -1.0 * half_width + pixel_size * x as f32;
        let world_y = half_width - pixel_size * y as f32;
        let point_on_wall = Point3::new(world_x, world_y, wall_z);
        let ray = Ray {
            origin: ray_origin,
            direction: (point_on_wall - ray_origin).normalize(),
        };
        let intersections = sphere.ray_intersections(&ray);

        if intersections.hit().is_some() {
            red
        } else {
            black
        }
    });
    let _ = img.save("demo.png");
}
