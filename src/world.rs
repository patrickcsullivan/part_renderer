use crate::{
    color::Rgb,
    interaction::SurfaceInteraction,
    intersection::{Intersection, Intersections},
    light::{phong_shading, PointLight},
    ray::Ray,
    shape::{Shape, Sphere},
};

pub struct World<'shp, 'mtrx, 'mtrl> {
    pub shapes: Vec<&'shp Sphere<'mtrx, 'mtrl>>,
    pub lights: Vec<PointLight>,
}

impl<'shp, 'mtrx, 'mtrl> World<'shp, 'mtrx, 'mtrl> {
    fn ray_intersections(&self, ray: &Ray) -> Intersections<'shp, 'mtrx, 'mtrl> {
        let mut inters = Intersections::empty();
        for &shape in &self.shapes {
            let mut new_inters = shape.ray_intersections(&ray);
            inters.append(&mut new_inters);
        }
        inters
    }

    pub fn shade_surface_interaction(
        &self,
        interaction: &SurfaceInteraction<'shp, 'mtrx, 'mtrl>,
    ) -> Rgb {
        // FIXME: Assumes there is exactly one light. Loop over lights, adding
        // to color to support multiple lights.
        let light = self.lights.first().unwrap();

        phong_shading(
            interaction.shape.material(),
            light,
            &interaction.point,
            &interaction.neg_ray_direction,
            &interaction.normal,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Rgb {
        let intersections = self.ray_intersections(&ray);
        if let Some(hit) = intersections.hit() {
            self.shade_surface_interaction(&hit.interaction)
        } else {
            Rgb::black()
        }
    }
}

pub struct WorldBuilder<'shp, 'mtrx, 'mtrl> {
    pub shapes: Vec<&'shp Sphere<'mtrx, 'mtrl>>,
    pub lights: Vec<PointLight>,
}

impl<'shp, 'mtrx, 'mtrl> WorldBuilder<'shp, 'mtrx, 'mtrl> {
    pub fn new() -> Self {
        Self {
            shapes: vec![],
            lights: vec![],
        }
    }

    pub fn build(self) -> World<'shp, 'mtrx, 'mtrl> {
        World {
            shapes: self.shapes,
            lights: self.lights,
        }
    }

    pub fn sphere(mut self, sphere: &'shp Sphere<'mtrx, 'mtrl>) -> Self {
        self.shapes.push(sphere);
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
        color::Rgb, light::PointLight, material::Material, matrix::identity4, ray::Ray,
        shape::Sphere, test::ApproxEq, world::WorldBuilder,
    };
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    #[test]
    fn ray_intersects_spheres() {
        let identity = identity4();
        let scale = Matrix4::from_scale(0.5);
        let inv_scale = scale.inverse_transform().unwrap();
        let material = Material::new(Rgb::new(0.8, 1.0, 0.6), 0.0, 0.7, 0.2, 0.0);
        let sphere1 = Sphere::new(&identity, &identity, false, &material);
        let sphere2 = Sphere::new(&scale, &inv_scale, false, &material);
        let light = PointLight::new(Rgb::white(), Point3::new(-10.0, 10.0, -10.0));
        let world = WorldBuilder::new()
            .sphere(&sphere1)
            .sphere(&sphere2)
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
mod color_at {
    use crate::{
        color::Rgb, light::PointLight, material::Material, matrix::identity4, ray::Ray,
        shape::Sphere, test::ApproxEq, world::WorldBuilder,
    };
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    // #[test]
    // fn color_at() {
    //     let identity = identity4();
    //     let scale = Matrix4::from_scale(0.5);
    //     let inv_scale = scale.inverse_transform().unwrap();
    //     let material = Material::new(Rgb::new(0.8, 1.0, 0.6), 0.0, 0.7, 0.2, 0.0);
    //     let sphere1 = Sphere::new(&identity, &identity, false, &material);
    //     let sphere2 = Sphere::new(&scale, &inv_scale, false, &material);
    //     let light = PointLight::new(Rgb::white(), Point3::new(-10.0, 10.0, -10.0));
    //     let world = WorldBuilder::new()
    //         .sphere(&sphere1)
    //         .sphere(&sphere2)
    //         .point_light(light)
    //         .build();

    //     // When ray misses.
    //     let ray = Ray {
    //         origin: Point3::new(0.0, 0.0, -5.0),
    //         direction: Vector3::new(0.0, 1.0, 0.0),
    //     };
    //     let color = world.color_at(&ray);
    //     assert!(color.approx_eq(&Rgb::black()));

    //     // When ray hits.
    //     let ray = Ray {
    //         origin: Point3::new(0.0, 0.0, -5.0),
    //         direction: Vector3::new(0.0, 0.0, 1.0),
    //     };
    //     let color = world.color_at(&ray);
    //     assert!(color.approx_eq(&Rgb::new(0.38066, 0.47583, 0.2855)));

    //     // // When ray starts outer sphere and hits inner sphere.
    //     // let ray = Ray {
    //     //     origin: Point3::new(0.0, 0.0, -5.0),
    //     //     direction: Vector3::new(0.0, 0.0, 1.0),
    //     // };
    //     // let color = world.color_at(&ray);
    //     // assert!(color.approx_eq(&Rgb::new(0.38066, 0.47583, 0.2855)));
    // }
}
