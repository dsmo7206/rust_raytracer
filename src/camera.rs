use glam::Vec3A as Vec3;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3, // Unit vector
    pub vertical_fov: f32,
}
