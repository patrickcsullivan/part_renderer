use crate::{
    camera::{view_transform, Camera, Film, OrthographicCamera},
    light::LightSource,
    material::Material,
    primitive::PrimitiveAggregate,
    scene::Scene,
    shape::Mesh,
};
use cgmath::{Matrix4, Transform, Vector2};

pub fn simple() {
    use crate::color::RgbSpectrum;
    use crate::geometry::matrix::identity4;
    use crate::shape::Shape;
    use cgmath::{Point3, Rad, Vector3};
    use std::f32::consts::PI;

    let right_transf = Matrix4::from_translation(Vector3::new(2.5, 0.0, 0.0));
    let left_transf = Matrix4::from_translation(Vector3::new(-2.5, 0.0, 0.0));

    let identity = identity4();
    let material = Material::new(
        RgbSpectrum::from_rgb(1.0, 0.2, 1.0),
        0.1,
        0.9,
        0.9,
        200.0,
        0.0,
    );
    let sphere1 = Shape::sphere(&identity, &identity, false);
    let sphere2 = Shape::sphere(&right_transf, &left_transf, false);
    let sphere3 = Shape::sphere(&left_transf, &right_transf, false);
    let light =
        LightSource::point_light(RgbSpectrum::constant(1.0), Point3::new(-10.0, 10.0, -10.0));

    let world_to_camera = view_transform(
        Point3::new(0.0, 0.0, -4.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let camera_to_world = world_to_camera.inverse_transform().unwrap();
    let film = Film::new(400, 400);
    let camera = OrthographicCamera::new(film, camera_to_world, 0.0, 100.0, Vector2::new(4.0, 4.0));
    // let camera = Camera::new(film, Rad(PI / 2.0), camera_to_world);

    let world = Scene::new(
        PrimitiveAggregate::Vector(vec![
            PrimitiveAggregate::primitive(sphere1, &material),
            PrimitiveAggregate::primitive(sphere2, &material),
            PrimitiveAggregate::primitive(sphere3, &material),
        ]),
        vec![light],
    );
    let img = world.render(Box::new(camera), 5);
    let _ = img.save("demo_simple.png");
}

pub fn complex() {
    use crate::color::RgbSpectrum;
    use crate::geometry::matrix::identity4;
    use crate::shape::Shape;
    use cgmath::{Point3, Rad, Vector3};
    use std::f32::consts::PI;

    let identity = identity4();

    let floor_material = Material::new(
        RgbSpectrum::from_rgb(1.0, 0.9, 0.9),
        0.1,
        0.9,
        0.0,
        200.0,
        0.1,
    );
    let floor = Shape::plane(&identity, &identity, false);

    let right_material = Material::new(
        RgbSpectrum::from_rgb(0.5, 1.0, 0.1),
        0.1,
        0.7,
        0.3,
        200.0,
        0.25,
    );
    let right_transf =
        Matrix4::from_translation(Vector3::new(1.5, 0.5, -0.5)) * Matrix4::from_scale(0.5);
    let right_inv_transf = right_transf.inverse_transform().unwrap();
    let right = Shape::sphere(&right_transf, &right_inv_transf, false);

    let left_material = Material::new(
        RgbSpectrum::from_rgb(1.0, 0.1, 0.3),
        0.1,
        0.7,
        0.3,
        200.0,
        0.0,
    );
    let left_transf =
        Matrix4::from_translation(Vector3::new(-1.5, 0.33, -0.75)) * Matrix4::from_scale(0.33);
    let left_inv_transf = left_transf.inverse_transform().unwrap();
    let left = Shape::sphere(&left_transf, &left_inv_transf, false);

    let back_material = Material::new(
        RgbSpectrum::from_rgb(0.1, 1.0, 0.5),
        0.1,
        0.7,
        0.3,
        200.0,
        0.5,
    );
    let back_transf =
        Matrix4::from_translation(Vector3::new(0.0, 1.0, 1.5)) * Matrix4::from_scale(0.55);
    let back_inv_transf = back_transf.inverse_transform().unwrap();
    let back = Shape::sphere(&back_transf, &back_inv_transf, false);

    let triangle_material = Material::new(
        RgbSpectrum::from_rgb(1.0, 0.8, 0.1),
        0.1,
        0.7,
        0.3,
        100.0,
        0.2,
    );
    let file = std::fs::File::open("teapot.stl").unwrap();
    let mut reader = std::io::BufReader::new(&file);
    let teapot_transf = Matrix4::from_angle_x(Rad(PI / -2.0)) * Matrix4::from_scale(0.1);
    let inv_teapot_transf = teapot_transf.inverse_transform().unwrap();
    let teapot_mesh =
        Mesh::from_stl(&teapot_transf, &inv_teapot_transf, false, &mut reader).unwrap();
    let teapot = PrimitiveAggregate::from_mesh(&teapot_mesh, &triangle_material);

    let light1 = LightSource::point_light(
        RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
        Point3::new(-10.0, 10.0, -10.0),
    );
    let light2 = LightSource::point_light(
        RgbSpectrum::from_rgb(0.2, 0.0, 0.4),
        Point3::new(10.0, 10.0, -10.0),
    );

    let world_to_camera = view_transform(
        Point3::new(0.0, 1.5, -5.0),
        Point3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let camera_to_world = world_to_camera.inverse_transform().unwrap();
    // let camera = Camera::new(400, 200, Rad(PI / 3.0), camera_transf);
    let film = Film::new(600, 300);
    let camera = OrthographicCamera::new(film, camera_to_world, 0.0, 100.0, Vector2::new(4.0, 4.0));
    // let camera = Camera::new(film, Rad(PI / 3.0), camera_to_world);

    let world = Scene::new(
        PrimitiveAggregate::Vector(vec![
            PrimitiveAggregate::primitive(floor, &floor_material),
            PrimitiveAggregate::primitive(right, &right_material),
            PrimitiveAggregate::primitive(left, &left_material),
            PrimitiveAggregate::primitive(back, &back_material),
            teapot,
        ]),
        vec![light1, light2],
    );
    let img = world.render(Box::new(camera), 5);
    let _ = img.save("demo.png");
}
