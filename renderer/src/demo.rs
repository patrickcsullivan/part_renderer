use crate::{
    camera::OrthographicCamera,
    color::RgbSpectrum,
    film::Film,
    filter::{BoxFilter, MitchellFilter},
    geometry::matrix::identity4,
    integrator::{render, WhittedRayTracer},
    light::Light,
    material::{Material, MatteMaterial},
    primitive::PrimitiveAggregate,
    sampler::{ConstantSampler, StratifiedSampler},
    scene::Scene,
    shape::{Mesh, Shape},
};
use cgmath::{Matrix4, Point3, Rad, Transform, Vector2, Vector3};
use std::f32::consts::{FRAC_PI_4, PI};
use typed_arena::Arena;

pub fn bunny_orth() {
    let mut mesh_arena = Arena::new();
    let mut matrix_arena = Arena::new();
    let mut material_arena = Arena::new();
    let scene = bunny_scene(&mut mesh_arena, &mut matrix_arena, &mut material_arena);

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
        &WhittedRayTracer {},
        5,
    );
    let img = film.write_image();

    let _ = img.save("bunny_orth.png");
}

fn bunny_scene<'msh, 'mtrx, 'mtrl>(
    mesh_arena: &'msh mut Arena<Mesh<'mtrx>>,
    matrix_arena: &'mtrx mut Arena<Matrix4<f32>>,
    material_arena: &'mtrl mut Arena<MatteMaterial>,
) -> Scene<'msh, 'mtrx, 'mtrl> {
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let identity = matrix_arena.alloc(Matrix4::from_scale(1.0));
    let file = std::fs::File::open("../renderer/plane.stl").unwrap();
    let mut reader = std::io::BufReader::new(&file);
    let plane_mesh =
        mesh_arena.alloc(Mesh::from_stl(identity, identity, false, &mut reader).unwrap());

    let bunny_transf = matrix_arena.alloc(
        Matrix4::from_angle_y(Rad(-0.8 * PI))
            * Matrix4::from_angle_x(Rad(PI / -2.0))
            * Matrix4::from_scale(0.02),
    );
    let inv_bunny_transf = matrix_arena.alloc(bunny_transf.inverse_transform().unwrap());
    let file = std::fs::File::open("../renderer/bunny.stl").unwrap();
    let mut reader = std::io::BufReader::new(&file);
    let bunny_mesh = mesh_arena
        .alloc(Mesh::from_stl(bunny_transf, inv_bunny_transf, false, &mut reader).unwrap());

    let material = material_arena.alloc(MatteMaterial::new(
        RgbSpectrum::from_rgb(0.4, 0.4, 0.4),
        0.3,
    ));

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
            PrimitiveAggregate::from_mesh(plane_mesh, material),
            PrimitiveAggregate::from_mesh(bunny_mesh, material),
        ]),
        vec![light1, light2],
    )
}
