#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let i = Interval::new(1.0, 2.0);

        assert!(i.contains(1.0));
        assert!(i.contains(2.0));
        assert!(i.contains(1.0001));
        assert!(i.contains(1.9999));
        assert!(!i.contains(0.9999));
        assert!(!i.contains(2.0001));
    }

    #[test]
    fn surrounds() {
        let i = Interval::new(1.0, 2.0);

        assert!(!i.surrounds(1.0));
        assert!(!i.surrounds(2.0));
        assert!(i.surrounds(1.0001));
        assert!(i.surrounds(1.9999));
        assert!(!i.surrounds(0.9999));
        assert!(!i.surrounds(2.0001));
    }

    #[test]
    fn clamp() {
        let i = Interval::new(1.0, 2.0);

        assert_eq!(i.clamp(1.5), 1.5);
        assert_eq!(i.clamp(1.0), 1.0);
        assert_eq!(i.clamp(2.0), 2.0);
        assert_eq!(i.clamp(0.9999), 1.0);
        assert_eq!(i.clamp(2.0001), 2.0);
    }
}
