use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct PointLight {
    pub intensity: Tuple,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(p: Tuple, i: Tuple) -> PointLight {
        PointLight {
            intensity: i,
            position: p,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Tuple::color(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert_eq!(intensity, light.intensity);
        assert_eq!(position, light.position);
    }
}
