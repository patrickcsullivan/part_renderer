use crate::{
    camera::OrthographicCamera,
    color::RgbSpectrum,
    film::Film,
    filter::{BoxFilter, MitchellFilter},
    geometry::matrix::identity4,
    integrator::{render, OriginalRayTracer},
    light::Light,
    material::{Material, MatteMaterial},
    primitive::PrimitiveAggregate,
    sampler::{ConstantSampler, StratifiedSampler},
    scene::Scene,
    shape::{Mesh, Shape},
};
use cgmath::{Matrix4, Point3, Rad, Transform, Vector2, Vector3};
use std::f32::consts::PI;
use typed_arena::Arena;

pub fn teapot_orth() {
    let mut mesh_arena = Arena::new();
    let mut matrix_arena = Arena::new();
    let mut material_arena = Arena::new();
    let scene = teapot_scene(&mut mesh_arena, &mut matrix_arena, &mut material_arena);

    let camera_to_world = Matrix4::from_translation(Vector3::new(0.0, 1.0, 0.0))
        * Matrix4::from_angle_x(Rad(PI / 8.0))
        * Matrix4::from_translation(Vector3::new(0.0, 0.0, -4.0));
    let resolution = Vector2::new(800, 800);
    let mut film = Film::new(resolution);
    let camera = OrthographicCamera::new(
        camera_to_world,
        0.0,
        100.0,
        Vector2::new(4.0, 4.0),
        resolution,
    );

    // let filter = BoxFilter::new(0.5, 0.5);
    let filter = MitchellFilter::new(2.0, 2.0, 1.0 / 3.0, 1.0 / 3.0);
    let sampler = StratifiedSampler::new(2, 2, 5, 0, true);
    // let sampler = ConstantSampler {};

    render(
        &scene,
        &camera,
        &mut film,
        &filter,
        &sampler,
        &OriginalRayTracer {},
        5,
    );
    let img = film.write_image();

    let _ = img.save("teapot_orth.png");
}

fn teapot_scene<'msh, 'mtrx, 'mtrl>(
    mesh_arena: &'msh mut Arena<Mesh<'mtrx>>,
    matrix_arena: &'mtrx mut Arena<Matrix4<f32>>,
    material_arena: &'mtrl mut Arena<MatteMaterial>,
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
    let teapot_transf =
        matrix_arena.alloc(Matrix4::from_angle_x(Rad(PI / -2.0)) * Matrix4::from_scale(0.1));
    let inv_teapot_transf = matrix_arena.alloc(teapot_transf.inverse_transform().unwrap());

    let material = material_arena.alloc(MatteMaterial::new(
        RgbSpectrum::from_rgb(0.4, 0.4, 0.4),
        0.0,
    ));
    // let floor_material = material_arena.alloc(MaterialV1::new(
    //     RgbSpectrum::from_rgb(1.0, 0.9, 0.9),
    //     0.1,
    //     0.9,
    //     0.0,
    //     200.0,
    //     0.1,
    // ));
    // let right_material = material_arena.alloc(MaterialV1::new(
    //     RgbSpectrum::from_rgb(0.5, 1.0, 0.1),
    //     0.1,
    //     0.7,
    //     0.3,
    //     200.0,
    //     0.25,
    // ));
    // let left_material = material_arena.alloc(MaterialV1::new(
    //     RgbSpectrum::from_rgb(1.0, 0.1, 0.3),
    //     0.1,
    //     0.7,
    //     0.3,
    //     200.0,
    //     0.0,
    // ));
    // let back_material = material_arena.alloc(MaterialV1::new(
    //     RgbSpectrum::from_rgb(0.1, 1.0, 0.5),
    //     0.1,
    //     0.7,
    //     0.3,
    //     200.0,
    //     0.5,
    // ));
    // let triangle_material = material_arena.alloc(MaterialV1::new(
    //     RgbSpectrum::from_rgb(1.0, 0.8, 0.1),
    //     0.1,
    //     0.7,
    //     0.3,
    //     100.0,
    //     0.2,
    // ));

    let floor = Shape::plane(identity, identity, false);
    let right = Shape::sphere(right_transf, right_inv_transf, false);
    let left = Shape::sphere(left_transf, left_inv_transf, false);
    let back = Shape::sphere(back_transf, back_inv_transf, false);

    let file = std::fs::File::open("teapot.stl").unwrap();
    let mut reader = std::io::BufReader::new(&file);
    let teapot_mesh = mesh_arena
        .alloc(Mesh::from_stl(teapot_transf, inv_teapot_transf, false, &mut reader).unwrap());
    let teapot = PrimitiveAggregate::from_mesh(teapot_mesh, material);

    let light1 = Light::point_light(
        Point3::new(-10.0, 10.0, -10.0),
        RgbSpectrum::from_rgb(1.0, 1.0, 1.0) * 2000.0,
    );
    let light2 = Light::point_light(
        Point3::new(10.0, 10.0, -10.0),
        RgbSpectrum::from_rgb(0.2, 0.0, 0.4) * 1000.0,
    );

    Scene::new(
        PrimitiveAggregate::Vector(vec![
            PrimitiveAggregate::primitive(floor, material),
            PrimitiveAggregate::primitive(right, material),
            PrimitiveAggregate::primitive(left, material),
            PrimitiveAggregate::primitive(back, material),
            teapot,
        ]),
        vec![light1, light2],
    )
}
