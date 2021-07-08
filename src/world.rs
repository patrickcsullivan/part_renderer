use crate::{
    camera::Camera,
    color::Rgb,
    interaction::SurfaceInteraction,
    intersection::{Intersection, Intersections},
    light::{phong_shading, PointLight},
    material::Material,
    object::Object,
    primitive::Primitive,
    ray::Ray,
};
use cgmath::{InnerSpace, MetricSpace, Point3};
use image::ImageBuffer;

pub struct World<'shp, 'mtrx, 'mtrl> {
    pub primitives: Vec<Primitive<'shp, 'mtrx, 'mtrl>>,
    pub lights: Vec<PointLight>,
}

impl<'shp, 'mtrx, 'mtrl> World<'shp, 'mtrx, 'mtrl> {
    fn ray_intersections(&self, ray: &Ray) -> Intersections<'shp, 'mtrx, 'mtrl> {
        let mut inters = Intersections::empty();
        for primitive in &self.primitives {
            let values = primitive
                .object
                .ray_intersections(&ray)
                .into_iter()
                .map(|(t, interaction)| Intersection {
                    t,
                    interaction,
                    primitive: Primitive::new(primitive.object, primitive.material),
                })
                .collect();
            let mut new_inters = Intersections::new(values);
            inters.append(&mut new_inters);
        }
        inters
    }

    pub fn shade_surface_interaction(
        &self,
        interaction: &SurfaceInteraction,
        material: &Material,
        remaining: usize,
    ) -> Rgb {
        self.lights.iter().fold(Rgb::black(), |color, light| {
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

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Rgb {
        let intersections = self.ray_intersections(&ray);
        if let Some(hit) = intersections.hit() {
            self.shade_surface_interaction(&hit.interaction, &hit.primitive.material, remaining)
        } else {
            Rgb::black()
        }
    }

    pub fn render(
        &self,
        camera: &Camera,
        recursions: usize,
    ) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
        ImageBuffer::from_fn(camera.width as u32, camera.height as u32, |x, y| {
            let ray = camera.ray_for_pixel(x, y);
            let color = self.color_at(&ray, recursions);
            let pixel: image::Rgb<u8> = color.into();
            pixel
        })
    }

    /// Returns true if the specified point is occluded from the light.
    pub fn is_occluded(&self, p: Point3<f32>, light: &PointLight) -> bool {
        let to_light = light.position - p;
        let distance = to_light.magnitude();

        let ray = Ray::new(p, to_light.normalize());
        let intersections = self.ray_intersections(&ray);

        if let Some(hit) = intersections.hit() {
            hit.t < distance
        } else {
            false
        }
    }

    fn reflected_color(
        &self,
        material: &Material,
        interaction: &SurfaceInteraction,
        remaining: usize,
    ) -> Rgb {
        if remaining < 1 || material.reflective == 0.0 {
            Rgb::black()
        } else {
            let reflect_ray = Ray::new(interaction.over_point(), interaction.reflect());
            let color = self.color_at(&reflect_ray, remaining - 1);
            color * material.reflective
        }
    }
}

pub struct WorldBuilder<'shp, 'mtrx, 'mtrl> {
    pub primitives: Vec<Primitive<'shp, 'mtrx, 'mtrl>>,
    pub lights: Vec<PointLight>,
}

impl<'shp, 'mtrx, 'mtrl> WorldBuilder<'shp, 'mtrx, 'mtrl> {
    pub fn new() -> Self {
        Self {
            primitives: vec![],
            lights: vec![],
        }
    }

    pub fn build(self) -> World<'shp, 'mtrx, 'mtrl> {
        World {
            primitives: self.primitives,
            lights: self.lights,
        }
    }

    pub fn primitive(mut self, sphere: Primitive<'shp, 'mtrx, 'mtrl>) -> Self {
        self.primitives.push(sphere);
        self
    }

    pub fn point_light(mut self, point_light: PointLight) -> Self {
        self.lights.push(point_light);
        self
    }
}

#[cfg(test)]
mod ray_intersections_tests {
    use crate::{
        color::Rgb, light::PointLight, material::Material, matrix::identity4, object::Object,
        primitive::Primitive, ray::Ray, test::ApproxEq, world::WorldBuilder,
    };
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    #[test]
    fn ray_intersects_spheres() {
        let identity = identity4();
        let scale = Matrix4::from_scale(0.5);
        let inv_scale = scale.inverse_transform().unwrap();
        let material = Material::new(Rgb::new(0.8, 1.0, 0.6), 0.0, 0.7, 0.2, 0.0, 0.0);
        let sphere1 = Object::sphere(&identity, &identity, false);
        let sphere2 = Object::sphere(&scale, &inv_scale, false);
        let primitive1 = Primitive::new(&sphere1, &material);
        let primitive2 = Primitive::new(&sphere2, &material);
        let light = PointLight::new(Rgb::white(), Point3::new(-10.0, 10.0, -10.0));
        let world = WorldBuilder::new()
            .primitive(primitive1)
            .primitive(primitive2)
            .point_light(light)
            .build();
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let intersections = world.ray_intersections(&ray);
        assert_eq!(intersections.values().len(), 4);
        assert!(intersections.values()[0].t.approx_eq(&4.0));
        assert!(intersections.values()[1].t.approx_eq(&4.5));
        assert!(intersections.values()[2].t.approx_eq(&5.5));
        assert!(intersections.values()[3].t.approx_eq(&6.0));
    }
}

#[cfg(test)]
mod color_at_tests {
    use crate::{
        color::Rgb, light::PointLight, material::Material, matrix::identity4, object::Object,
        primitive::Primitive, ray::Ray, test::ApproxEq, world::WorldBuilder,
    };
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    #[test]
    fn color_at() {
        let identity = identity4();
        let scale = Matrix4::from_scale(0.5);
        let inv_scale = scale.inverse_transform().unwrap();
        let material = Material::new(Rgb::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, 0.0);
        let sphere1 = Object::sphere(&identity, &identity, false);
        let sphere2 = Object::sphere(&scale, &inv_scale, false);
        let primitive1 = Primitive::new(&sphere1, &material);
        let primitive2 = Primitive::new(&sphere2, &material);
        let light = PointLight::new(Rgb::white(), Point3::new(-10.0, 10.0, -10.0));
        let world = WorldBuilder::new()
            .primitive(primitive1)
            .primitive(primitive2)
            .point_light(light)
            .build();

        // When ray misses.
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 1.0, 0.0),
        };
        let color = world.color_at(&ray, 0);
        assert!(color.approx_eq(&Rgb::black()));

        // When ray hits.
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let color = world.color_at(&ray, 0);
        assert!(color.approx_eq(&Rgb::new(0.38066, 0.47583, 0.2855)));

        // When ray starts outer sphere and hits inner sphere.
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let color = world.color_at(&ray, 0);
        assert!(color.approx_eq(&Rgb::new(0.38066, 0.47583, 0.2855)));
    }
}

#[cfg(test)]
mod is_occluded_tests {
    use crate::{
        color::Rgb, light::PointLight, material::Material, matrix::identity4, object::Object,
        primitive::Primitive, ray::Ray, test::ApproxEq, world::WorldBuilder,
    };
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    #[test]
    fn is_occluded() {
        let identity = identity4();
        let material = Material::new(Rgb::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, 0.0);
        let sphere = Object::sphere(&identity, &identity, false);
        let primitive = Primitive::new(&sphere, &material);
        let light = PointLight::new(Rgb::white(), Point3::new(-10.0, 10.0, -10.0));
        let world = WorldBuilder::new()
            .primitive(primitive)
            .point_light(light)
            .build();

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
