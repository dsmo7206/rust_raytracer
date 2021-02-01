use crate::image::Colour;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub image_width: usize,
    pub image_height: usize,
    pub background_colour: Colour,
    pub rays_per_pixel: usize,
}
