use crate::spheres::Sphere;

pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(tt: f64, oo: &'a Sphere) -> Intersection<'a> {
        Intersection { t: tt, object: oo }
    }
}

pub fn intersections<'a>(xs: &[&'a Intersection]) -> Vec<&'a Intersection<'a>> {
    let mut v = xs.to_vec();
    v
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::tuple::Tuple;
    #[test]
    fn an_intersection_encapsulates_t_and_an_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(3.5, i.t);
        assert_eq!(&s, i.object);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&vec![&i1, &i2]);
        assert_eq!(1.0, i1.t);
        assert_eq!(2.0, i2.t);
    }
}
