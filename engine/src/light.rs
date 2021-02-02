use crate::{image::Colour, scene::Scene};
use glam::Vec3A as Vec3;

#[derive(serde::Deserialize)]
pub enum Light {
    PointLight(PointLight),
}

impl Light {
    pub fn visible_fraction(&self, position: Vec3, scene: &Scene) -> f32 {
        match self {
            Light::PointLight(inner) => inner.visible_fraction(position, scene),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct PointLight {
    pub position: Vec3,
    pub colour: Colour,
}

impl PointLight {
    pub fn visible_fraction(&self, position: Vec3, scene: &Scene) -> f32 {
        use std::ops::Sub;
        let ray = crate::object::Ray {
            origin: position,
            direction: self.position.sub(position).normalize(),
        };

        match scene.get_hit(&ray) {
            Some(_) => 0.0,
            None => 1.0,
        }
    }
}
