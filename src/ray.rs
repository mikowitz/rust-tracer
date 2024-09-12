use crate::vec3::{Point, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let o = Point::new(0., -1., 3.5);
        let d = Vector::new(-1., 1., 0.);
        let r = Ray::new(o, d);

        assert_eq!(r.at(0.0), Point::new(0., -1., 3.5));
        assert_eq!(r.at(1.5), Point::new(-1.5, 0.5, 3.5));
        assert_eq!(r.at(-1.), Point::new(1., -2., 3.5));
    }
}
