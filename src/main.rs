mod camera;
mod color;
mod efloat;
mod interaction;
mod light;
mod material;
mod math;
mod mesh;
mod ray;
mod renderable;
mod shape;
mod transform;
mod world;

#[cfg(test)]
mod test;

use crate::{
    camera::{view_transform, Camera},
    light::{phong_shading, PointLight},
    material::Material,
    mesh::Mesh,
    renderable::{Primitive, Renderable},
    world::World,
};
use cgmath::{Matrix, Matrix4, Transform};

fn main() {
    println!("Hello, world!");
    demo();
    demo_simple();
}

fn demo_simple() {
    use crate::color::Rgb;
    use crate::math::matrix::identity4;
    use crate::shape::Shape;
    use cgmath::{Point3, Rad, Vector3};
    use std::f32::consts::PI;

    let right_transf = Matrix4::from_translation(Vector3::new(2.5, 0.0, 0.0));
    let left_transf = Matrix4::from_translation(Vector3::new(-2.5, 0.0, 0.0));

    let identity = identity4();
    let material = Material::new(Rgb::new(1.0, 0.2, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0);
    let sphere1 = Shape::sphere(&identity, &identity, false);
    let sphere2 = Shape::sphere(&right_transf, &left_transf, false);
    let sphere3 = Shape::sphere(&left_transf, &right_transf, false);
    let light = PointLight::new(Rgb::white(), Point3::new(-10.0, 10.0, -10.0));

    let camera_transf = view_transform(
        Point3::new(0.0, 0.0, -4.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let camera = Camera::new(400, 400, Rad(PI / 2.0), camera_transf);

    let world = World::new(
        Renderable::Vector(vec![
            Renderable::primitive(sphere1, &material),
            Renderable::primitive(sphere2, &material),
            Renderable::primitive(sphere3, &material),
        ]),
        vec![light],
    );
    let img = world.render(&camera, 5);
    let _ = img.save("demo_simple.png");
}

fn demo() {
    use crate::color::Rgb;
    use crate::math::matrix::identity4;
    use crate::shape::Shape;
    use cgmath::{Point3, Rad, Vector3};
    use std::f32::consts::PI;

    let identity = identity4();

    let floor_material = Material::new(Rgb::new(1.0, 0.9, 0.9), 0.1, 0.9, 0.0, 200.0, 0.1);
    let floor = Shape::plane(&identity, &identity, false);

    let middle_material = Material::new(Rgb::new(0.1, 1.0, 0.5), 0.1, 0.7, 0.3, 200.0, 0.5);
    let middle_transf = Matrix4::from_translation(Vector3::new(-0.5, 0.0, 0.5));
    let middle_inv_transf = middle_transf.inverse_transform().unwrap();
    let middle = Shape::sphere(&middle_transf, &middle_inv_transf, false);

    let right_material = Material::new(Rgb::new(0.5, 1.0, 0.1), 0.1, 0.7, 0.3, 200.0, 0.25);
    let right_transf =
        Matrix4::from_translation(Vector3::new(1.5, 0.5, -0.5)) * Matrix4::from_scale(0.5);
    let right_inv_transf = right_transf.inverse_transform().unwrap();
    let right = Shape::sphere(&right_transf, &right_inv_transf, false);

    let left_material = Material::new(Rgb::new(1.0, 0.8, 0.1), 0.1, 0.7, 0.3, 200.0, 0.0);
    let left_transf =
        Matrix4::from_translation(Vector3::new(-1.5, 0.33, -0.75)) * Matrix4::from_scale(0.33);
    let left_inv_transf = left_transf.inverse_transform().unwrap();
    let left = Shape::sphere(&left_transf, &left_inv_transf, false);

    let back_material = Material::new(Rgb::new(1.0, 0.1, 0.3), 0.1, 0.7, 0.3, 200.0, 0.0);
    let back_transf =
        Matrix4::from_translation(Vector3::new(0.0, 1.0, 1.5)) * Matrix4::from_scale(0.55);
    let back_inv_transf = back_transf.inverse_transform().unwrap();
    let back = Shape::sphere(&back_transf, &back_inv_transf, false);

    let file = std::fs::File::open("cube_corner.stl").unwrap();
    let mut reader = std::io::BufReader::new(&file);
    let mesh_transf = Matrix4::from_translation(Vector3::new(-0.0, 1.0, 0.5))
        * Matrix4::from_angle_z(Rad(PI / 6.0))
        * Matrix4::from_angle_y(Rad(PI / 4.0));
    let mesh_inv_transf = middle_transf.inverse_transform().unwrap();
    let mesh = Mesh::from_stl(&mesh_transf, &mesh_inv_transf, false, &mut reader).unwrap();
    let triangle_material = Material::new(Rgb::new(1.0, 0.8, 0.1), 0.1, 0.7, 0.3, 100.0, 0.2);
    let triangle1 = Shape::triangle(&mesh, 0);
    let triangle2 = Shape::triangle(&mesh, 1);
    let triangle3 = Shape::triangle(&mesh, 2);
    let triangle4 = Shape::triangle(&mesh, 3);

    let light1 = PointLight::new(Rgb::new(1.0, 1.0, 1.0), Point3::new(-10.0, 10.0, -10.0));
    let light2 = PointLight::new(Rgb::new(0.2, 0.0, 0.4), Point3::new(10.0, 10.0, -10.0));

    let camera_transf = view_transform(
        Point3::new(0.0, 1.5, -5.0),
        Point3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    // let camera = Camera::new(400, 200, Rad(PI / 3.0), camera_transf);
    let camera = Camera::new(1200, 600, Rad(PI / 3.0), camera_transf);

    let world = World::new(
        Renderable::Vector(vec![
            Renderable::primitive(floor, &floor_material),
            Renderable::primitive(middle, &middle_material),
            Renderable::primitive(right, &right_material),
            Renderable::primitive(left, &left_material),
            Renderable::primitive(back, &back_material),
            Renderable::primitive(triangle1, &triangle_material),
            Renderable::primitive(triangle2, &triangle_material),
            Renderable::primitive(triangle3, &triangle_material),
            Renderable::primitive(triangle4, &triangle_material),
        ]),
        vec![light1, light2],
    );
    let img = world.render(&camera, 5);
    let _ = img.save("demo.png");
}
