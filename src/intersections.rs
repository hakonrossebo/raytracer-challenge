use crate::spheres::Sphere;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
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
    v.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));
    v
}

pub fn hit<'a>(xs: &[&'a Intersection]) -> Option<&'a Intersection<'a>> {
    // let y:Option<&'a Intersection> = xs.iter().filter(|x| x.t >= 0.0).collect().pop();
    for x in xs {
        if x.t >= 0.0 {
            return Some(x);
        }
    }
    None
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

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&vec![&i2, &i1]);
        let i = hit(&xs);
        assert_eq!(Some(&i1), i);
    }
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = intersections(&vec![&i2, &i1]);
        let i = hit(&xs);
        assert_eq!(Some(&i2), i);
    }
    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = intersections(&vec![&i2, &i1]);
        let i = hit(&xs);
        assert_eq!(None, i);
    }
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = intersections(&vec![&i1, &i2, &i3, &i4]);
        let i = hit(&xs);
        assert_eq!(Some(&i4), i);
    }
}
