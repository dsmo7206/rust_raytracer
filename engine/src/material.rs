use crate::{
    image::Colour,
    object::{Hit, Ray},
    scene::Scene,
};
use std::ops::{Add, Mul, Sub};

#[derive(serde::Deserialize)]
pub enum Material {
    Solid(Colour),
    Diffuse(Colour),
}

impl Material {
    /// This function is complete rubbish
    pub fn get_colour(&self, scene: &Scene, _ray: &Ray, hit: Hit) -> Colour {
        match self {
            &Material::Solid(colour) => colour,
            &Material::Diffuse(colour) => {
                // Assume there's only one light for now
                let light = &scene.lights[0];

                let frac = light.visible_fraction(hit.position.add(hit.normal.mul(0.01)), scene);

                let light_position = match light {
                    crate::light::Light::PointLight(inner) => inner.position,
                };

                let position_to_light = light_position.sub(hit.position).normalize();

                let x = hit.normal.dot(position_to_light).max(0.0);
                //colour * ray.direction.dot(-hit.normal)
                colour * x * frac
            }
        }
    }
}
