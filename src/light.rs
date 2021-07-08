use cgmath::{InnerSpace, Point3, Vector3};

use crate::{color::Rgb, interaction::SurfaceInteraction, material::Material, math::vector};

pub struct PointLight {
    pub intensity: Rgb,
    pub position: Point3<f32>,
}

impl PointLight {
    pub fn new(intensity: Rgb, position: Point3<f32>) -> Self {
        Self {
            intensity,
            position,
        }
    }
}

pub fn phong_shading(
    material: &Material,
    light: &PointLight,
    position: &Point3<f32>,
    eye: &Vector3<f32>,
    normal: &Vector3<f32>,
    in_shadow: bool,
) -> Rgb {
    let effective_color = material.color * light.intensity;
    let to_light = (light.position - position).normalize();
    let ambient = effective_color * material.ambient;

    // light_dot_normal is the cosine of the angle between the light and normal.
    // If it's negative then the light is on the other side of the surface.
    let light_dot_normal = to_light.dot(*normal);

    let (diffuse, specular) = if in_shadow || light_dot_normal < 0.0 {
        (Rgb::black(), Rgb::black())
    } else {
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        // reflect_dot_eye is the cosine of the angle between the reflection and
        // the camera. If it's negative then the reflection is not visible.
        let reflect = vector::reflect(-1.0 * to_light, *normal);
        let reflect_dot_eye = reflect.dot(*eye);
        let specular = if reflect_dot_eye <= 0.0 {
            Rgb::black()
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            light.intensity * material.specular * factor
        };

        (diffuse, specular)
    };

    ambient + diffuse + specular
}

#[cfg(test)]
mod phong_shading_tests {
    use crate::color::Rgb;
    use crate::light::{phong_shading, PointLight};
    use crate::material::Material;
    use crate::test::ApproxEq;
    use cgmath::{Point3, Vector3};

    #[test]
    fn eye_between_light_and_surface() {
        let material = Material::new(Rgb::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0);
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 0.0, -1.0);

        let light = PointLight::new(Rgb::new(1.0, 1.0, 1.0), Point3::new(0.0, 0.0, -10.0));
        let eye = Vector3::new(0.0, 0.0, -1.0);
        let result = phong_shading(&material, &light, &position, &eye, &normal, false);
        assert!(result.approx_eq(&Rgb::new(1.9, 1.9, 1.9)));
    }

    #[test]
    fn surface_in_shadow() {
        let material = Material::new(Rgb::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0);
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 0.0, -1.0);

        let light = PointLight::new(Rgb::new(1.0, 1.0, 1.0), Point3::new(0.0, 0.0, -10.0));
        let eye = Vector3::new(0.0, 0.0, -1.0);
        let result = phong_shading(&material, &light, &position, &eye, &normal, true);
        assert!(result.approx_eq(&Rgb::new(0.1, 0.1, 0.1)));
    }

    #[test]
    fn eye_between_light_and_surface_and_offset_45_degrees() {
        let material = Material::new(Rgb::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0);
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 0.0, -1.0);

        let light = PointLight::new(Rgb::new(1.0, 1.0, 1.0), Point3::new(0.0, 0.0, -10.0));
        let eye = Vector3::new(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / -2.0);
        let result = phong_shading(&material, &light, &position, &eye, &normal, false);
        assert!(result.approx_eq(&Rgb::new(1.0, 1.0, 1.0)));
    }

    #[test]
    fn eye_opposite_surface_light_offset() {
        let material = Material::new(Rgb::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0);
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 0.0, -1.0);

        let light = PointLight::new(Rgb::new(1.0, 1.0, 1.0), Point3::new(0.0, 10.0, -10.0));
        let eye = Vector3::new(0.0, 0.0, -1.0);
        let result = phong_shading(&material, &light, &position, &eye, &normal, false);
        assert!(result.approx_eq(&Rgb::new(0.7364, 0.7364, 0.7364)));
    }

    #[test]
    fn eye_in_reflection_path() {
        let material = Material::new(Rgb::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0);
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 0.0, -1.0);

        let light = PointLight::new(Rgb::new(1.0, 1.0, 1.0), Point3::new(0.0, 10.0, -10.0));
        let eye = Vector3::new(0.0, f32::sqrt(2.0) / -2.0, f32::sqrt(2.0) / -2.0);
        let result = phong_shading(&material, &light, &position, &eye, &normal, false);
        assert!(result.approx_eq(&Rgb::new(1.6364, 1.6364, 1.6364)));
    }

    #[test]
    fn light_behind_surface() {
        let material = Material::new(Rgb::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0);
        let position = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(0.0, 0.0, -1.0);

        let light = PointLight::new(Rgb::new(1.0, 1.0, 1.0), Point3::new(0.0, 10.0, 10.0));
        let eye = Vector3::new(0.0, 0.0, -1.0);
        let result = phong_shading(&material, &light, &position, &eye, &normal, false);
        assert!(result.approx_eq(&Rgb::new(0.1, 0.1, 0.1)));
    }
}
