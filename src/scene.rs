use crate::camera::Camera;
use crate::config::Config;
use crate::image::{Colour, Image};
use crate::object::{Hit, Object, Ray};
use glam::Vec3A as Vec3;
use rayon::prelude::*;
use serde_derive::Deserialize;
use std::ops::Mul;

#[derive(Deserialize)]
pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn render(&self, camera: &Camera, config: &Config) -> Image {
        // Build the lower left point and the basis vectors for the viewport
        let (horizontal, vertical, lower_left) = {
            let camera_matrix = glam::Mat4::look_at_rh(
                glam::Vec3::from(camera.position),
                glam::Vec3::from(camera.target),
                glam::Vec3::from(camera.up),
            )
            .inverse();

            // Build basis vectors for the viewport. For the basis vectors, we assume the camera
            // is at (0, 0, 0), and looks along the -z axis. Assume the "viewport" is a rectangle
            // at z=-1.0 with horizontal and vertical along the x and y axes.
            let viewport_low_y = -1.0 * (camera.vertical_fov.to_radians() * 0.5).atan();
            let viewport_low_x =
                viewport_low_y * config.image_width as f32 / config.image_height as f32;

            // Build the Vec4s to be multiplied (transformed) by the camera matrix.
            // "Directions" have w=0.0; positions have w=1.0.
            let horizontal = glam::Vec4::new(-2.0 * viewport_low_x, 0.0, 0.0, 0.0);
            let vertical = glam::Vec4::new(0.0, -2.0 * viewport_low_y, 0.0, 0.0);
            let lower_left = glam::Vec4::new(viewport_low_x, viewport_low_y, -1.0, 1.0);

            // Transform each of the above and convert back into a Vec3A
            (
                Vec3::from(camera_matrix.mul_vec4(horizontal)),
                Vec3::from(camera_matrix.mul_vec4(vertical)),
                Vec3::from(camera_matrix.mul_vec4(lower_left)),
            )
        };

        let u_mult = 1.0 / (config.image_width - 1) as f32;
        let v_mult = 1.0 / (config.image_height - 1) as f32;

        let make_ray = |row: usize, col: usize| Ray {
            origin: camera.position,
            direction: (lower_left
                + horizontal.mul(col as f32 * u_mult)
                + vertical.mul(row as f32 * v_mult)
                - camera.position)
                .normalize(),
        };

        // Parallelise at the per-row level so that each work unit has enough to chew on,
        // so that the threading overhead is somewhat minimised.
        let rows = (0..config.image_height)
            .into_par_iter()
            .map(|row| {
                (0..config.image_width)
                    .map(|col| {
                        self.get_colour(&make_ray(row, col))
                            .unwrap_or(config.background_colour)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Image::from_rows(config.image_width, config.image_height, rows)
    }

    fn get_colour(&self, ray: &Ray) -> Option<Colour> {
        let mut closest: Option<(Hit, &Object)> = None;

        for object in self.objects.iter() {
            if let Some(hit) = object.shape.get_hit(ray) {
                match &closest {
                    Some((prev_closest_hit, _)) => {
                        if hit.distance < prev_closest_hit.distance {
                            closest = Some((hit, object));
                        }
                    }
                    None => {
                        closest = Some((hit, object));
                    }
                }
            }
        }

        match closest {
            Some((closest_hit, closest_object)) => {
                Some(closest_object.material.get_colour(self, ray, closest_hit))
            }
            None => None,
        }
    }
}

pub enum ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error")
    }
}
