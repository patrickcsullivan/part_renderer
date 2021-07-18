use crate::{
    camera::{Camera, CameraSample},
    color::RgbSpectrum,
    interaction::SurfaceInteraction,
    light::{phong_shading, PointLight},
    material::Material,
    primitive::PrimitiveAggregate,
    ray::Ray,
};
use cgmath::{InnerSpace, Point2, Point3};
use image::ImageBuffer;

pub struct Scene<'msh, 'mtrx, 'mtrl> {
    pub primitives: PrimitiveAggregate<'msh, 'mtrx, 'mtrl>,
    pub lights: Vec<PointLight>,
}

impl<'msh, 'mtrx, 'mtrl> Scene<'msh, 'mtrx, 'mtrl> {
    pub fn new(
        renderable: PrimitiveAggregate<'msh, 'mtrx, 'mtrl>,
        lights: Vec<PointLight>,
    ) -> Self {
        Self {
            primitives: renderable,
            lights,
        }
    }

    pub fn shade_surface_interaction(
        &self,
        interaction: &SurfaceInteraction,
        material: &Material,
        remaining: usize,
    ) -> RgbSpectrum {
        self.lights
            .iter()
            .fold(RgbSpectrum::constant(0.0), |color, light| {
                // Shift the interaction point away from the surface slightly, so that
                // the occlusion check doesn't accidentally intersect the surface.
                let in_shadow = self.is_occluded(interaction.over_point(), light);

                let surface = phong_shading(
                    material,
                    light,
                    &interaction.point,
                    &interaction.neg_ray_direction,
                    &interaction.normal,
                    in_shadow,
                );

                let reflected = self.reflected_color(material, interaction, remaining);

                color + surface + reflected
            })
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> RgbSpectrum {
        if let Some((_t, primitive, interaction)) = self.primitives.ray_intersection(&ray) {
            self.shade_surface_interaction(&interaction, primitive.material, remaining)
        } else {
            RgbSpectrum::constant(0.0)
        }
    }

    pub fn render(
        &self,
        camera: &Camera,
        recursions: usize,
    ) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
        ImageBuffer::from_fn(
            camera.film.resolution.x as u32,
            camera.film.resolution.y as u32,
            |x, y| {
                println!("At ({}, {})", x, y);
                let sample = CameraSample::at_pixel_center(Point2::new(x as usize, y as usize));
                let (ray, _) = camera.generate_ray(&sample);
                let color = self.color_at(&ray, recursions);
                let pixel: image::Rgb<u8> = color.into();
                pixel
            },
        )
    }

    /// Returns true if the specified point is occluded from the light.
    pub fn is_occluded(&self, p: Point3<f32>, light: &PointLight) -> bool {
        let to_light = light.position - p;
        let distance = to_light.magnitude();

        let ray = Ray::new(p, to_light.normalize());
        if let Some((t, _, _)) = self.primitives.ray_intersection(&ray) {
            t < distance
        } else {
            false
        }
    }

    fn reflected_color(
        &self,
        material: &Material,
        interaction: &SurfaceInteraction,
        remaining: usize,
    ) -> RgbSpectrum {
        if remaining < 1 || material.reflective == 0.0 {
            RgbSpectrum::constant(0.0)
        } else {
            let reflect_ray = Ray::new(interaction.over_point(), interaction.reflect());
            let color = self.color_at(&reflect_ray, remaining - 1);
            color * material.reflective
        }
    }
}

#[cfg(test)]
mod color_at_tests {
    use crate::{
        color::RgbSpectrum, geometry::matrix::identity4, light::PointLight, material::Material,
        primitive::PrimitiveAggregate, ray::Ray, scene::Scene, shape::Shape, test::ApproxEq,
    };
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    #[test]
    fn color_at() {
        let identity = identity4();
        let scale = Matrix4::from_scale(0.5);
        let inv_scale = scale.inverse_transform().unwrap();
        let material = Material::new(
            RgbSpectrum::from_rgb(0.8, 1.0, 0.6),
            0.1,
            0.7,
            0.2,
            200.0,
            0.0,
        );
        let sphere1 = Shape::sphere(&identity, &identity, false);
        let sphere2 = Shape::sphere(&scale, &inv_scale, false);
        let primitive1 = PrimitiveAggregate::primitive(sphere1, &material);
        let primitive2 = PrimitiveAggregate::primitive(sphere2, &material);
        let light = PointLight::new(RgbSpectrum::constant(1.0), Point3::new(-10.0, 10.0, -10.0));
        let world = Scene::new(
            PrimitiveAggregate::Vector(vec![primitive1, primitive2]),
            vec![light],
        );

        // When ray misses.
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        let color = world.color_at(&ray, 0);
        assert!(color.approx_eq(&RgbSpectrum::constant(0.0)));

        // When ray hits.
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        let color = world.color_at(&ray, 0);
        assert!(color.approx_eq(&RgbSpectrum::from_rgb(0.38066, 0.47583, 0.2855)));

        // When ray starts outer sphere and hits inner sphere.
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        let color = world.color_at(&ray, 0);
        assert!(color.approx_eq(&RgbSpectrum::from_rgb(0.38066, 0.47583, 0.2855)));
    }
}

#[cfg(test)]
mod is_occluded_tests {
    use crate::{
        color::RgbSpectrum, geometry::matrix::identity4, light::PointLight, material::Material,
        primitive::PrimitiveAggregate, scene::Scene, shape::Shape, test::ApproxEq,
    };
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    #[test]
    fn is_occluded() {
        let identity = identity4();
        let material = Material::new(
            RgbSpectrum::from_rgb(0.8, 1.0, 0.6),
            0.1,
            0.7,
            0.2,
            200.0,
            0.0,
        );
        let sphere = Shape::sphere(&identity, &identity, false);
        let primitive = PrimitiveAggregate::primitive(sphere, &material);
        let light = PointLight::new(RgbSpectrum::constant(1.0), Point3::new(-10.0, 10.0, -10.0));
        let world = Scene::new(primitive, vec![light]);

        // The point is above the sphere and collinear with the light, so
        // the sphere does not block light from the point.
        let p = Point3::new(0.0, 10.0, 0.0);
        assert!(!world.is_occluded(p, &world.lights[0]));

        // The sphere is between the point and the light, so it blocks light.
        let p = Point3::new(10.0, -10.0, 10.0);
        assert!(world.is_occluded(p, &world.lights[0]));

        // The light is between the point and the sphere, so it is unblocked.
        let p = Point3::new(-20.0, 20.0, -20.0);
        assert!(!world.is_occluded(p, &world.lights[0]));

        // The point is between the light and the sphere, so the light in
        // ublocked..
        let p = Point3::new(-2.0, 2.0, -2.0);
        assert!(!world.is_occluded(p, &world.lights[0]));
    }
}
