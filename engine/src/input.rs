use crate::{camera::Camera, config::Config, scene::Scene};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Input {
    pub camera: Camera,
    pub config: Config,
    pub scene: Scene,
}
