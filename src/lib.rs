#![allow(mixed_script_confusables)]

pub mod camera;
pub mod color;
pub mod entity;
pub mod hit_record;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ray;
pub mod vec3;
pub mod world;

pub mod prelude {
    pub use super::camera::Camera;
    pub use super::color::Color;
    pub use super::entity::Entity;
    pub use super::entity::Entity::Sphere;
    pub use super::material::Material;
    pub use super::vec3::{Point, Vector};
    pub use super::world::World;
}
