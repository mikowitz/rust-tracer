use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point, Vector},
};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(p: Point, normal: Vector, t: f32, ray: &Ray, material: Material) -> Self {
        let front_face = ray.direction.dot(normal) < 0.0;
        Self {
            p,
            normal: if front_face { normal } else { -normal },
            t,
            front_face,
            material,
        }
    }
}
