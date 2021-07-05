use cgmath::{Matrix, Matrix4, Transform};

use crate::{
    camera::{view_transform, Camera},
    light::{phong_shading, PointLight},
    material::Material,
    world::WorldBuilder,
};

mod camera;
mod color;
mod interaction;
mod intersection;
mod light;
mod material;
mod matrix;
mod ray;
mod shape;
mod transform;
mod vector;
mod world;

#[cfg(test)]
mod test;

fn main() {
    println!("Hello, world!");
    demo();
    demo_simple();
}

fn demo_simple() {
    use crate::color::Rgb;
    use crate::matrix::identity4;
    use crate::shape::{Shape, Sphere};
    use cgmath::{Point3, Rad, Vector3};
    use std::f32::consts::PI;

    let right_transf = Matrix4::from_translation(Vector3::new(2.5, 0.0, 0.0));
    let left_transf = Matrix4::from_translation(Vector3::new(-2.5, 0.0, 0.0));

    let identity = identity4();
    let material = Material::new(Rgb::new(1.0, 0.2, 1.0), 0.1, 0.9, 0.9, 200.0);
    let sphere1 = Sphere::new(&identity, &identity, false, &material);
    let sphere2 = Sphere::new(&right_transf, &left_transf, false, &material);
    let sphere3 = Sphere::new(&left_transf, &right_transf, false, &material);
    let light = PointLight::new(Rgb::white(), Point3::new(-10.0, 10.0, -10.0));

    let camera_transf = view_transform(
        Point3::new(0.0, 0.0, -4.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let camera = Camera::new(400, 400, Rad(PI / 2.0), camera_transf);

    let world = WorldBuilder::new()
        .point_light(light)
        .sphere(&sphere1)
        .sphere(&sphere2)
        .sphere(&sphere3)
        .build();
    let img = world.render(&camera);
    let _ = img.save("demo_simple.png");
}

fn demo() {
    use crate::color::Rgb;
    use crate::matrix::identity4;
    use crate::ray::Ray;
    use crate::shape::{Shape, Sphere};
    use cgmath::{InnerSpace, Point3, Rad, Vector3};
    use image::ImageBuffer;
    use std::f32::consts::PI;

    let floor_material = Material::new(Rgb::new(1.0, 0.9, 0.9), 0.1, 0.9, 0.0, 200.0);
    let floor_transf = Matrix4::from_nonuniform_scale(10.0, 0.01, 10.0);
    let floor_inv_transf = floor_transf.inverse_transform().unwrap();
    let floor = Sphere::new(&floor_transf, &floor_inv_transf, false, &floor_material);

    let left_wall_transf = Matrix4::from_translation(Vector3::new(0.0, 0.0, 5.0))
        * Matrix4::from_angle_y(Rad(PI / -4.0))
        * Matrix4::from_angle_x(Rad(PI / 2.0))
        * Matrix4::from_nonuniform_scale(10.0, 0.01, 10.0);
    let left_wall_inv_transf = left_wall_transf.inverse_transform().unwrap();
    let left_wall = Sphere::new(
        &left_wall_transf,
        &left_wall_inv_transf,
        false,
        &floor_material,
    );

    let right_wall_transf = Matrix4::from_translation(Vector3::new(0.0, 0.0, 5.0))
        * Matrix4::from_angle_y(Rad(PI / 4.0))
        * Matrix4::from_angle_x(Rad(PI / 2.0))
        * Matrix4::from_nonuniform_scale(10.0, 0.01, 10.0);
    let right_wall_inv_transf = right_wall_transf.inverse_transform().unwrap();
    let right_wall = Sphere::new(
        &right_wall_transf,
        &right_wall_inv_transf,
        false,
        &floor_material,
    );

    let middle_material = Material::new(Rgb::new(0.1, 1.0, 0.5), 0.1, 0.7, 0.3, 200.0);
    let middle_transf = Matrix4::from_translation(Vector3::new(-0.5, 1.0, 0.5));
    let middle_inv_transf = middle_transf.inverse_transform().unwrap();
    let middle = Sphere::new(&middle_transf, &middle_inv_transf, false, &middle_material);

    let right_material = Material::new(Rgb::new(0.5, 1.0, 0.1), 0.1, 0.7, 0.3, 200.0);
    let right_transf =
        Matrix4::from_translation(Vector3::new(1.5, 0.5, -0.5)) * Matrix4::from_scale(0.5);
    let right_inv_transf = right_transf.inverse_transform().unwrap();
    let right = Sphere::new(&right_transf, &right_inv_transf, false, &right_material);

    let left_material = Material::new(Rgb::new(1.0, 0.8, 0.1), 0.1, 0.7, 0.3, 200.0);
    let left_transf =
        Matrix4::from_translation(Vector3::new(-1.5, 0.33, -0.75)) * Matrix4::from_scale(0.33);
    let left_inv_transf = left_transf.inverse_transform().unwrap();
    let left = Sphere::new(&left_transf, &left_inv_transf, false, &left_material);

    let light1 = PointLight::new(Rgb::new(1.0, 1.0, 1.0), Point3::new(-10.0, 10.0, -10.0));
    let light2 = PointLight::new(Rgb::new(0.2, 0.0, 0.4), Point3::new(10.0, 10.0, -10.0));

    let camera_transf = view_transform(
        Point3::new(0.0, 1.5, -5.0),
        Point3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let camera = Camera::new(400, 200, Rad(PI / 3.0), camera_transf);

    let world = WorldBuilder::new()
        .point_light(light1)
        .point_light(light2)
        .sphere(&floor)
        .sphere(&left_wall)
        .sphere(&right_wall)
        .sphere(&middle)
        .sphere(&right)
        .sphere(&left)
        .build();
    let img = world.render(&camera);
    let _ = img.save("demo.png");
}
