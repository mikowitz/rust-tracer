use crate::{
    hit_record::HitRecord,
    hittable::Hittable,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point, Vector},
};

pub enum Entity {
    Sphere(Ray, f32, Material),
}

impl Entity {
    pub fn sphere(center: Point, radius: f32, material: Material) -> Self {
        Entity::Sphere(
            Ray::new(center, Vector::new(0., 0., 0.), 0.),
            radius,
            material,
        )
    }

    pub fn moving_sphere(center: Point, center2: Point, radius: f32, material: Material) -> Self {
        Entity::Sphere(Ray::new(center, center2 - center, 0.), radius, material)
    }
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
    center: &Ray,
    radius: f32,
    material: Material,
) -> Option<HitRecord> {
    let current_center = center.at(ray.time);
    let oc = current_center - ray.origin;
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
    let normal = (p - current_center) / radius;
    Some(HitRecord::new(p, normal, t, ray, material))
}
