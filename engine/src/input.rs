use crate::camera::Camera;
use crate::config::Config;
use crate::scene::Scene;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Input {
    pub camera: Camera,
    pub config: Config,
    pub scene: Scene,
}
