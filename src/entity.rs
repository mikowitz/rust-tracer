use crate::{
    hit_record::HitRecord, hittable::Hittable, interval::Interval, material::Material, ray::Ray,
    vec3::Point,
};

pub enum Entity {
    Sphere(Point, f32, Material),
}

impl Hittable for Entity {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        match self {
            Entity::Sphere(center, radius, mat) => hit_sphere(ray, interval, center, *radius, *mat),
        }
    }
}

fn hit_sphere(
    ray: &Ray,
    interval: &Interval,
    center: &Point,
    radius: f32,
    material: Material,
) -> Option<HitRecord> {
    let oc = *center - ray.origin;
    let a = ray.direction.length_squared();
    let h = ray.direction.dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrtd = discriminant.sqrt();

    let mut root = (h - sqrtd) / a;

    if !interval.contains(root) {
        root = (h + sqrtd) / a;
        if !interval.contains(root) {
            return None;
        }
    }

    let t = root;
    let p = ray.at(t);
    let normal = (p - *center) / radius;
    Some(HitRecord::new(p, normal, t, ray, material))
}
