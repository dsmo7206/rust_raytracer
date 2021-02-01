use glam::Vec3A as Vec3;
use serde_derive::Deserialize;

const MIN_HIT_DISTANCE: f32 = 0.01;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, distance: f32) -> Vec3 {
        self.origin + self.direction * distance
    }
}

pub struct Hit {
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

#[derive(Deserialize)]
pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Object::Sphere(inner) => inner.get_hit(ray),
        }
    }
}

#[derive(Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;

            if root < MIN_HIT_DISTANCE {
                root = (-half_b + sqrtd) / a;
                if root < MIN_HIT_DISTANCE {
                    return None;
                }
            }

            let position = ray.at(root);

            Some(Hit {
                distance: root,
                position: position,
                normal: (position - self.center) / self.radius,
            })
        }
    }
}
