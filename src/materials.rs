use crate::lights::PointLight;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        let c = Tuple::color(1.0, 1.0, 1.0);
        Material {
            color: c,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Tuple {
        let black = Tuple::color(0.0, 0.0, 0.0);
        let ambient = Tuple::color(0.0, 0.0, 0.0);
        let mut diffuse = Tuple::color(0.0, 0.0, 0.0);
        let mut specular = Tuple::color(0.0, 0.0, 0.0);

        // Combine the surface color with the light's color/intensity
        let effective_color: Tuple = self.color * light.intensity;

        // Find the direction to the light source
        let lightv = (light.position - point).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // Light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means
        // the light is on the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            diffuse = black;
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle betwen the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0.0 {
                specular = black;
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lights::PointLight;

    #[test]
    fn the_default_material() {
        let m = Material::new();
        let c = Tuple::color(1.0, 1.0, 1.0);
        assert_eq!(c, m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200.0, m.shininess);
    }
    #[test]
    fn lightning_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Tuple::color(1.9, 1.9, 1.9);
        assert_eq!(expected, result);
    }
    #[test]
    fn lightning_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Tuple::color(1.0, 1.0, 1.0);
        assert_eq!(expected, result);
    }
    #[test]
    fn lightning_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Tuple::color(0.7364, 0.7364, 0.7364);
        assert_eq!(expected, result);
    }
    #[test]
    fn lightning_with_the_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Tuple::color(1.6364, 1.6364, 1.6364);
        assert_eq!(expected, result);
    }
    #[test]
    fn lightning_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Tuple::color(0.1, 0.1, 0.1);
        assert_eq!(expected, result);
    }
}
