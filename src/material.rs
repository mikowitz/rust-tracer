use crate::{color::Color, hit_record::HitRecord, ray::Ray, vec3::Vec3};
use rand::prelude::*;

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f32),
    Dielectric(f32),
}

impl Material {
    pub fn scatter(self, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        match self {
            Material::Lambertian(albedo) => scatter_lambertian(albedo, ray, hr),
            Material::Metal(albedo, fuzz) => scatter_metal(albedo, fuzz, ray, hr),
            Material::Dielectric(refraction_index) => scatter_dielectric(refraction_index, ray, hr),
        }
    }
}

fn scatter_lambertian(attenuation: Color, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
    let mut scatter_direction = hr.normal + Vec3::random_normalized();
    if scatter_direction.is_near_zero() {
        scatter_direction = hr.normal;
    }
    let scattered = Ray::new(hr.p, scatter_direction, ray.time);
    Some(Scatter {
        attenuation,
        scattered,
    })
}

fn scatter_metal(attenuation: Color, fuzz: f32, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
    let mut reflected = ray.direction.reflect(hr.normal);
    reflected = reflected.normalize() + Vec3::random_normalized() * fuzz;
    let scattered = Ray::new(hr.p, reflected, ray.time);
    Some(Scatter {
        attenuation,
        scattered,
    })
}

fn scatter_dielectric(refraction_index: f32, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
    let mut rng = rand::thread_rng();
    let attenuation = Color::white();
    let ri = if hr.front_face {
        1.0 / refraction_index
    } else {
        refraction_index
    };
    let unit_direction = ray.direction.normalize();
    let cosθ = -unit_direction.dot(hr.normal).min(1.0);
    let sinθ = (1.0 - cosθ * cosθ).sqrt();

    let cannot_refract = ri * sinθ > 1.0;

    let direction = if cannot_refract || (reflectance(cosθ, ri) > rng.gen::<f32>()) {
        unit_direction.reflect(hr.normal)
    } else {
        unit_direction.refract(hr.normal, ri)
    };

    let scattered = Ray::new(hr.p, direction, ray.time);
    Some(Scatter {
        attenuation,
        scattered,
    })
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
