use crate::{color::Color, hit_record::HitRecord, ray::Ray, vec3::Vec3};

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f32),
}

impl Material {
    pub fn scatter(self, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        match self {
            Material::Lambertian(albedo) => scatter_lambertian(albedo, hr),
            Material::Metal(albedo, fuzz) => scatter_metal(albedo, fuzz, ray, hr),
        }
    }
}

fn scatter_lambertian(attenuation: Color, hr: &HitRecord) -> Option<Scatter> {
    let mut scatter_direction = hr.normal + Vec3::random_normalized();
    if scatter_direction.is_near_zero() {
        scatter_direction = hr.normal;
    }
    let scattered = Ray::new(hr.p, scatter_direction);
    Some(Scatter {
        attenuation,
        scattered,
    })
}

fn scatter_metal(attenuation: Color, fuzz: f32, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
    let mut reflected = ray.direction.reflect(hr.normal);
    reflected = reflected.normalize() + Vec3::random_normalized() * fuzz;
    let scattered = Ray::new(hr.p, reflected);
    Some(Scatter {
        attenuation,
        scattered,
    })
}
