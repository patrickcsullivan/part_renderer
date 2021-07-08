mod axis;
mod camera;
mod color;
mod interaction;
mod intersection;
mod light;
mod material;
mod math;
mod matrix;
mod mesh;
mod object;
mod primitive;
mod ray;
mod transform;
mod vector;
mod world;

#[cfg(test)]
mod test;

use crate::{
    camera::{view_transform, Camera},
    light::{phong_shading, PointLight},
    material::Material,
    primitive::Primitive,
    world::WorldBuilder,
};
use cgmath::{Matrix, Matrix4, Transform};

fn main() {
    println!("Hello, world!");
    demo();
    demo_simple();
}

fn demo_simple() {
    use crate::color::Rgb;
    use crate::matrix::identity4;
    use crate::object::Object;
    use cgmath::{Point3, Rad, Vector3};
    use std::f32::consts::PI;

    let right_transf = Matrix4::from_translation(Vector3::new(2.5, 0.0, 0.0));
    let left_transf = Matrix4::from_translation(Vector3::new(-2.5, 0.0, 0.0));

    let identity = identity4();
    let material = Material::new(Rgb::new(1.0, 0.2, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0);
    let sphere1 = Object::sphere(&identity, &identity, false);
    let sphere2 = Object::sphere(&right_transf, &left_transf, false);
    let sphere3 = Object::sphere(&left_transf, &right_transf, false);
    let light = PointLight::new(Rgb::white(), Point3::new(-10.0, 10.0, -10.0));

    let camera_transf = view_transform(
        Point3::new(0.0, 0.0, -4.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let camera = Camera::new(400, 400, Rad(PI / 2.0), camera_transf);

    let world = WorldBuilder::new()
        .point_light(light)
        .primitive(Primitive::new(&sphere1, &material))
        .primitive(Primitive::new(&sphere2, &material))
        .primitive(Primitive::new(&sphere3, &material))
        .build();
    let img = world.render(&camera, 5);
    let _ = img.save("demo_simple.png");
}

fn demo() {
    use crate::color::Rgb;
    use crate::matrix::identity4;
    use crate::object::Object;
    use crate::ray::Ray;
    use cgmath::{InnerSpace, Point3, Rad, Vector3};
    use image::ImageBuffer;
    use std::f32::consts::PI;

    let identity = identity4();

    let floor_material = Material::new(Rgb::new(1.0, 0.9, 0.9), 0.1, 0.9, 0.0, 200.0, 0.1);
    let floor = Object::plane(&identity, &identity, false);

    let middle_material = Material::new(Rgb::new(0.1, 1.0, 0.5), 0.1, 0.7, 0.3, 200.0, 0.5);
    let middle_transf = Matrix4::from_translation(Vector3::new(-0.5, 0.0, 0.5));
    let middle_inv_transf = middle_transf.inverse_transform().unwrap();
    let middle = Object::sphere(&middle_transf, &middle_inv_transf, false);

    let right_material = Material::new(Rgb::new(0.5, 1.0, 0.1), 0.1, 0.7, 0.3, 200.0, 0.25);
    let right_transf =
        Matrix4::from_translation(Vector3::new(1.5, 0.5, -0.5)) * Matrix4::from_scale(0.5);
    let right_inv_transf = right_transf.inverse_transform().unwrap();
    let right = Object::sphere(&right_transf, &right_inv_transf, false);

    let left_material = Material::new(Rgb::new(1.0, 0.8, 0.1), 0.1, 0.7, 0.3, 200.0, 0.0);
    let left_transf =
        Matrix4::from_translation(Vector3::new(-1.5, 0.33, -0.75)) * Matrix4::from_scale(0.33);
    let left_inv_transf = left_transf.inverse_transform().unwrap();
    let left = Object::sphere(&left_transf, &left_inv_transf, false);

    let back_material = Material::new(Rgb::new(1.0, 0.1, 0.3), 0.1, 0.7, 0.3, 200.0, 0.0);
    let back_transf =
        Matrix4::from_translation(Vector3::new(0.0, 1.0, 1.5)) * Matrix4::from_scale(0.55);
    let back_inv_transf = back_transf.inverse_transform().unwrap();
    let back = Object::sphere(&back_transf, &back_inv_transf, false);

    let triangle_material = Material::new(Rgb::new(0.3, 0.3, 0.5), 0.1, 0.7, 0.3, 200.0, 0.8);
    let triangle = Object::triangle(
        &identity,
        &identity,
        false,
        Point3::new(0.0, 0.0, 0.1),
        Point3::new(0.6, 0.2, -0.1),
        Point3::new(0.3, 0.5, 0.0),
    );

    let light1 = PointLight::new(Rgb::new(1.0, 1.0, 1.0), Point3::new(-10.0, 10.0, -10.0));
    let light2 = PointLight::new(Rgb::new(0.2, 0.0, 0.4), Point3::new(10.0, 10.0, -10.0));

    let camera_transf = view_transform(
        Point3::new(0.0, 1.5, -5.0),
        Point3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    // let camera = Camera::new(400, 200, Rad(PI / 3.0), camera_transf);
    let camera = Camera::new(1200, 600, Rad(PI / 3.0), camera_transf);

    let world = WorldBuilder::new()
        .point_light(light1)
        .point_light(light2)
        .primitive(Primitive::new(&floor, &floor_material))
        .primitive(Primitive::new(&middle, &middle_material))
        .primitive(Primitive::new(&right, &right_material))
        .primitive(Primitive::new(&left, &left_material))
        .primitive(Primitive::new(&back, &back_material))
        .primitive(Primitive::new(&triangle, &triangle_material))
        .build();
    let img = world.render(&camera, 5);
    let _ = img.save("demo.png");
}
