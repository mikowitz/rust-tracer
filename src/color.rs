use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn to_ppm(self) -> String {
        let r = (255.999 * self.x) as u32;
        let g = (255.999 * self.y) as u32;
        let b = (255.999 * self.z) as u32;

        format!("{} {} {}", r, g, b)
    }
}
