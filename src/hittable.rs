use crate::{hit_record::HitRecord, interval::Interval, ray::Ray};

pub trait Hittable {
    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord>;
}
