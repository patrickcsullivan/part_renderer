use crate::{
    camera::{Camera, OrthographicCamera},
    color::RgbSpectrum,
    film::Film,
    filter::BoxFilter,
    geometry::matrix::identity4,
    integrator::{render, OriginalRayTracer},
    light::PointLight,
    light_v1::LightSource,
    material_v1::MaterialV1,
    primitive::PrimitiveAggregate,
    sampler::{ConstantSampler, IncrementalSampler},
    scene::{self, Scene},
    shape::{Mesh, Shape},
};
use cgmath::{Matrix4, Point3, Rad, Transform, Vector2, Vector3};
use std::f32::consts::PI;
use typed_arena::Arena;

pub fn circles_ortho() {
    let mut matrix_arena = Arena::new();
    let mut material_arena = Arena::new();
    let scene = circles_scene(&mut matrix_arena, &mut material_arena);

    let camera_to_world = Matrix4::from_angle_x(Rad(3.0 * PI / 4.0))
        * Matrix4::from_translation(Vector3::new(0.0, 0.0, -4.0));
    let resolution = Vector2::new(600, 400);
    let mut film = Film::new(resolution);
    let camera = OrthographicCamera::new(
        camera_to_world,
        0.0,
        100.0,
        Vector2::new(6.0, 4.0),
        resolution,
    );

    let filter = BoxFilter::new(1.5, 1.5);
    render::<ConstantSampler>(
        &scene,
        &camera,
        &mut film,
        &filter,
        &ConstantSampler {},
        &OriginalRayTracer {},
        5,
    );
    let img = film.write_image();

    let _ = img.save("circles_ortho.png");
}

pub fn bunny_orth() {
    let mut mesh_arena = Arena::new();
    let mut matrix_arena = Arena::new();
    let mut material_arena = Arena::new();
    let scene = bunny_scene(&mut mesh_arena, &mut matrix_arena, &mut material_arena);

    let camera_to_world = Matrix4::from_translation(Vector3::new(0.0, 1.0, 0.0))
        * Matrix4::from_angle_x(Rad(PI / 8.0))
        * Matrix4::from_translation(Vector3::new(0.0, 0.0, -4.0));
    let resolution = Vector2::new(1200, 800);
    let mut film = Film::new(resolution);
    let camera = OrthographicCamera::new(
        camera_to_world,
        0.0,
        100.0,
        Vector2::new(6.0, 4.0),
        resolution,
    );

    let filter = BoxFilter::new(0.5, 0.5);
    render::<ConstantSampler>(
        &scene,
        &camera,
        &mut film,
        &filter,
        &ConstantSampler {},
        &OriginalRayTracer {},
        5,
    );
    let img = film.write_image();

    let _ = img.save("bunny_orth.png");
}

fn circles_scene<'msh, 'mtrx, 'mtrl>(
    matrix_arena: &'mtrx mut Arena<Matrix4<f32>>,
    material_arena: &'mtrl mut Arena<MaterialV1>,
) -> Scene<'msh, 'mtrx, 'mtrl> {
    let right_transf = matrix_arena.alloc(Matrix4::from_translation(Vector3::new(2.0, 0.0, 0.0)));
    let left_transf = matrix_arena.alloc(Matrix4::from_translation(Vector3::new(-2.0, 0.0, 0.0)));
    let identity = matrix_arena.alloc(identity4());

    let material = material_arena.alloc(MaterialV1::new(
        RgbSpectrum::from_rgb(1.0, 0.2, 1.0),
        0.1,
        0.9,
        0.9,
        200.0,
        0.0,
    ));

    let sphere1 = Shape::sphere(identity, identity, false);
    let sphere2 = Shape::sphere(right_transf, left_transf, false);
    let sphere3 = Shape::sphere(left_transf, right_transf, false);
    let light = PointLight::new(Point3::new(-10.0, 10.0, -10.0), RgbSpectrum::constant(1.0));

    Scene::new(
        PrimitiveAggregate::Vector(vec![
            PrimitiveAggregate::primitive(sphere1, material),
            PrimitiveAggregate::primitive(sphere2, material),
            PrimitiveAggregate::primitive(sphere3, material),
        ]),
        vec![Box::new(light)],
    )
}

fn bunny_scene<'msh, 'mtrx, 'mtrl>(
    mesh_arena: &'msh mut Arena<Mesh<'mtrx>>,
    matrix_arena: &'mtrx mut Arena<Matrix4<f32>>,
    material_arena: &'mtrl mut Arena<MaterialV1>,
) -> Scene<'msh, 'mtrx, 'mtrl> {
    let identity = matrix_arena.alloc(identity4());
    let right_transf = matrix_arena
        .alloc(Matrix4::from_translation(Vector3::new(1.5, 0.5, -0.5)) * Matrix4::from_scale(0.5));
    let right_inv_transf = matrix_arena.alloc(right_transf.inverse_transform().unwrap());
    let left_transf = matrix_arena.alloc(
        Matrix4::from_translation(Vector3::new(-1.5, 0.33, -0.75)) * Matrix4::from_scale(0.33),
    );
    let left_inv_transf = matrix_arena.alloc(left_transf.inverse_transform().unwrap());
    let back_transf = matrix_arena
        .alloc(Matrix4::from_translation(Vector3::new(0.0, 1.0, 1.5)) * Matrix4::from_scale(0.55));
    let back_inv_transf = matrix_arena.alloc(back_transf.inverse_transform().unwrap());
    let bunny_transf =
        matrix_arena.alloc(Matrix4::from_angle_x(Rad(PI / -2.0)) * Matrix4::from_scale(0.02));
    let inv_bunny_transf = matrix_arena.alloc(bunny_transf.inverse_transform().unwrap());

    let floor_material = material_arena.alloc(MaterialV1::new(
        RgbSpectrum::from_rgb(1.0, 0.9, 0.9),
        0.1,
        0.9,
        0.0,
        200.0,
        0.1,
    ));
    let right_material = material_arena.alloc(MaterialV1::new(
        RgbSpectrum::from_rgb(0.5, 1.0, 0.1),
        0.1,
        0.7,
        0.3,
        200.0,
        0.25,
    ));
    let left_material = material_arena.alloc(MaterialV1::new(
        RgbSpectrum::from_rgb(1.0, 0.1, 0.3),
        0.1,
        0.7,
        0.3,
        200.0,
        0.0,
    ));
    let back_material = material_arena.alloc(MaterialV1::new(
        RgbSpectrum::from_rgb(0.1, 1.0, 0.5),
        0.1,
        0.7,
        0.3,
        200.0,
        0.5,
    ));
    let triangle_material = material_arena.alloc(MaterialV1::new(
        RgbSpectrum::from_rgb(1.0, 0.8, 0.1),
        0.1,
        0.7,
        0.3,
        100.0,
        0.2,
    ));

    let floor = Shape::plane(identity, identity, false);
    let right = Shape::sphere(right_transf, right_inv_transf, false);
    let left = Shape::sphere(left_transf, left_inv_transf, false);
    let back = Shape::sphere(back_transf, back_inv_transf, false);

    let file = std::fs::File::open("bunny.stl").unwrap();
    let mut reader = std::io::BufReader::new(&file);
    let bunny_mesh = mesh_arena
        .alloc(Mesh::from_stl(bunny_transf, inv_bunny_transf, false, &mut reader).unwrap());
    let bunny = PrimitiveAggregate::from_mesh(bunny_mesh, triangle_material);

    let light1 = PointLight::new(
        Point3::new(-10.0, 10.0, -10.0),
        RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
    );
    let light2 = PointLight::new(
        Point3::new(10.0, 10.0, -10.0),
        RgbSpectrum::from_rgb(0.2, 0.0, 0.4),
    );

    Scene::new(
        PrimitiveAggregate::Vector(vec![
            PrimitiveAggregate::primitive(floor, floor_material),
            PrimitiveAggregate::primitive(right, right_material),
            PrimitiveAggregate::primitive(left, left_material),
            PrimitiveAggregate::primitive(back, back_material),
            bunny,
        ]),
        vec![Box::new(light1), Box::new(light2)],
    )
}
