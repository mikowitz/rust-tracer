use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    pub fn black() -> Self {
        Color::new(0., 0., 0.)
    }

    pub fn white() -> Self {
        Color::new(1., 1., 1.)
    }

    pub fn to_ppm(self) -> String {
        let i = Interval::new(0.000, 0.999);
        let r = (256.0 * i.clamp(self.x)) as u32;
        let g = (256.0 * i.clamp(self.y)) as u32;
        let b = (256.0 * i.clamp(self.z)) as u32;

        format!("{} {} {}", r, g, b)
    }
}
