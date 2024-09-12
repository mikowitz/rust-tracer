use crate::{
    entity::Entity, hit_record::HitRecord, hittable::Hittable, interval::Interval, ray::Ray,
};

#[derive(Default)]
pub struct World {
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

impl Hittable for World {
    fn hit(&self, r: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = interval.max;

        for entity in &self.entities {
            match entity.hit(r, &Interval::new(interval.min, closest)) {
                None => {}
                Some(rec) => {
                    closest = rec.t;
                    hit_record = Some(rec);
                }
            }
        }
        hit_record
    }
}
