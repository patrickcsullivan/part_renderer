mod error;

use cgmath::{
    point2, point3, vec2, vec3, Deg, InnerSpace, Matrix, Matrix4, Point2, Point3, Rad, Transform,
    Vector2, Vector3,
};
use error::{Error, Result};
use image::{imageops, ImageBuffer, Rgba};
use mesh::{Mesh, MeshBuilder};
use renderer::color::RgbaSpectrum;
use renderer::filter::MitchellFilter;
use renderer::integrator::{render, WhittedRayTracer};
use renderer::light::{self, Light};
use renderer::material::MatteMaterial;
use renderer::primitive::PrimitiveAggregate;
use renderer::sampler::ConstantSampler;
use renderer::scene::Scene;
use renderer::{camera::OrthographicCamera, film::Film};
use std::cmp;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use std::io::BufReader;
use typed_arena::Arena;

fn main() -> Result<()> {
    let matches = clap::App::new("Part Viewer")
        .arg(
            clap::Arg::with_name("INPUT")
                .help("The input STL file to use")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("OUTPUT")
                .help("The output destination")
                .required(true)
                .index(2),
        )
        .arg(
            clap::Arg::with_name("WIDTH")
                .help("Width of the output image in pixels")
                .required(true)
                .index(3),
        )
        .arg(
            clap::Arg::with_name("HEIGHT")
                .help("Height of the output image in pixels")
                .required(true)
                .index(4),
        )
        .arg(
            clap::Arg::with_name("CAMERA VERTICAL FOV")
                .help("The camera's vertical field of view in degrees. Default is 45.")
                .required(false)
                .index(5),
        )
        .arg(
            clap::Arg::with_name("CAMERA POSITION POLAR ANGLE")
                .help("The camera's spherical position theta component. This is the angle between the camera and the z axis. Default is 90.")
                .required(false)
                .index(6),
        )
        .arg(
            clap::Arg::with_name("CAMERA POSITION AZIMUTHAL ANGLE")
                .help("The camera's spherical position phi component. This is the angle between the camera and the x axis in the xy plane. Default is 0.")
                .required(false)
                .index(7),
        )
        .arg(
            clap::Arg::with_name("LIGHT POSITION POLAR ANGLE")
                .help("The light's spherical position theta component. This is the angle between the light and the z axis. Default is 0.")
                .required(false)
                .index(8),
        )
        .arg(
            clap::Arg::with_name("LIGHT POSITION AZIMUTHAL ANGLE")
                .help("The light's spherical position phi component. This is the angle between the light and the x axis in the xy plane. Default is 0.")
                .required(false)
                .index(9),
        )
        .arg(
            clap::Arg::with_name("LIGHT INTENSITY")
                .help("The light's intensity. Default is 1.0.")
                .required(false)
                .index(10),
        )
        .arg(
            clap::Arg::with_name("CROP")
                .short("c")
                .help("Enables cropping"),
        )
        .get_matches();

    // The first four arguments are required by Clap, so unwrapping them is ok.
    let src_path = matches.value_of("INPUT").unwrap();
    let dst_path = matches.value_of("OUTPUT").unwrap();
    let width = matches.value_of("WIDTH").unwrap().parse::<u32>()?;
    let height = matches.value_of("HEIGHT").unwrap().parse::<u32>()?;

    let camera_fovy = Deg(matches
        .value_of("CAMERA VERTICAL FOV")
        .unwrap_or("45")
        .parse::<f32>()?);
    let camera_theta = Deg(matches
        .value_of("CAMERA POSITION POLAR ANGLE")
        .unwrap_or("90")
        .parse::<f32>()?);
    let camera_phi = Deg(matches
        .value_of("CAMERA POSITION AZIMUTHAL ANGLE")
        .unwrap_or("0")
        .parse::<f32>()?);
    let light_theta = Deg(matches
        .value_of("LIGHT POSITION POLAR ANGLE")
        .unwrap_or("0")
        .parse::<f32>()?);
    let light_phi = Deg(matches
        .value_of("LIGHT POSITION AZIMUTHAL ANGLE")
        .unwrap_or("0")
        .parse::<f32>()?);
    let point_light_intensity = matches
        .value_of("LIGHT INTENSITY")
        .unwrap_or("1.0")
        .parse::<f32>()?;
    let is_crop_on = matches.is_present("CROP");

    let mesh_arena = Arena::new();
    let file = std::fs::File::open(&src_path)?;
    let mut reader = std::io::BufReader::new(&file);
    let mesh = mesh_arena.alloc(MeshBuilder::from_stl(&mut reader)?.build());
    let (bounds_min, bounds_max) = mesh.bounding_box().ok_or(Error::EmptyMesh)?;
    let center = bounds_min + (bounds_max - bounds_min) / 2.0;
    let center_to_origin = Matrix4::from_translation(Point3::new(0.0f32, 0.0f32, 0.0f32) - center);
    mesh.transform(center_to_origin);
    let bounding_sphere_radius = max_distance_from_origin(mesh);
    println!("RADIUS: {}", bounding_sphere_radius);
    let scale = Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0)
        * Matrix4::from_scale(1.0 / bounding_sphere_radius);
    mesh.transform_swapping_handedness(scale);

    let material_arena = Arena::new();
    let material = material_arena.alloc(MatteMaterial::new(
        RgbaSpectrum::from_rgb(0.4, 0.4, 0.4),
        0.3,
    ));

    let light_position =
        origin_to_spherical(1.0, light_theta, light_phi).transform_point(point3(0.0, 0.0, 0.0));
    let light = Light::point_light(
        light_position,
        RgbaSpectrum::from_rgb(1.0, 1.0, 1.0) * point_light_intensity,
    );
    let scene = Scene::new(
        PrimitiveAggregate::Vector(vec![
            // PrimitiveAggregate::from_mesh(plane_mesh, material),
            PrimitiveAggregate::from_mesh(mesh, material),
        ]),
        vec![light],
    );

    let camera_to_world = origin_to_spherical(1.0, camera_theta, camera_phi);
    let resolution = Vector2::new(width as usize, height as usize);
    let mut film = Film::new(resolution);
    let camera = OrthographicCamera::new(
        camera_to_world,
        0.0,
        100.0,
        Vector2::new(2.0, 2.0),
        resolution,
    );

    // let filter = BoxFilter::new(0.5, 0.5);
    let filter = MitchellFilter::new(2.0, 2.0, 1.0 / 3.0, 1.0 / 3.0);
    // let sampler = StratifiedSampler::new(2, 2, 5, 0, true);
    let sampler = ConstantSampler {};

    render(
        &scene,
        &camera,
        &mut film,
        &filter,
        &sampler,
        &WhittedRayTracer {},
        5,
    );
    let mut image = film.write_image();

    if is_crop_on {
        image = crop_to_non_transparent(&image)?;
    }

    image.save(dst_path)?;
    Ok(())
}

/// Returns a transformation matrix that translates a point at the origin to the
/// given spherical coordinates.
///
/// If this transformation matrix is applied to a camera coordinate system that
/// looks towards positive z and has positive y as its "up" direction, then the
/// resulting coordinate system will look at the origin and "up" will be in
/// roughly the positive y direction.
fn origin_to_spherical(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> Matrix4<f32> {
    Matrix4::from_angle_z(Rad::from(phi) - Rad(FRAC_PI_2))
        * Matrix4::from_angle_x(Rad(PI) - Rad::from(theta))
        * Matrix4::from_translation(Vector3::new(0.0, 0.0, -1.0 * r))
}

/// Return the maximum distance between any vertex and the origin.
fn max_distance_from_origin(mesh: &Mesh) -> f32 {
    mesh.positions
        .iter()
        .fold(0.0f32, |acc, p| {
            let dist2 = (p - point3(0.0, 0.0, 0.0)).magnitude2();
            acc.max(dist2)
        })
        .sqrt()
}

/// Crop transparent edges from the image.
fn crop_to_non_transparent(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let (crop_bounds_min, crop_bounds_max) =
        non_transparent_bounds(image).ok_or(Error::ZeroAreaImage)?;
    let crop_bounds_diag = crop_bounds_max - crop_bounds_min;
    let cropped = imageops::crop_imm(
        image,
        crop_bounds_min.x,
        crop_bounds_min.y,
        crop_bounds_diag.x + 1,
        crop_bounds_diag.y + 1,
    )
    .to_image();
    Ok(cropped)
}

/// Return the min and max (inclusive) pixels of a 2D bounding box around any
/// non-transparent content in the image.
fn non_transparent_bounds(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Option<(Point2<u32>, Point2<u32>)> {
    let mut min_max = None;

    for (x, y, rgba) in image.enumerate_pixels() {
        if rgba.0[3] == 0 {
            continue;
        }

        min_max = match min_max {
            None => Some((point2(x, y), point2(x, y))),
            Some((min, max)) => {
                let new_min = point2(cmp::min(min.x, x), cmp::min(min.y, y));
                let new_max = point2(cmp::max(max.x, x), cmp::max(max.y, y));
                Some((new_min, new_max))
            }
        };
    }

    min_max
}
