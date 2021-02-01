use crate::{
    image::Colour,
    object::{Hit, Ray},
    scene::Scene,
};

#[derive(serde::Deserialize)]
pub enum Material {
    Solid(Colour),
    Diffuse(Colour),
}

impl Material {
    pub fn get_colour(&self, _scene: &Scene, ray: &Ray, hit: Hit) -> Colour {
        match self {
            &Material::Solid(colour) => colour,
            &Material::Diffuse(colour) => colour * ray.direction.dot(-hit.normal),
        }
    }
}
