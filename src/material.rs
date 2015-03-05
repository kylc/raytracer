use na::Vec3;
use intersection::Intersection;
use ray::Ray;

pub enum Material {
    Emissive(EmissiveMaterial),
    Reflective(ReflectiveMaterial)
}

pub struct EmissiveMaterial {
    emissivity: f32
}

pub trait ReflectiveMaterial {
    fn bounce(&self, incoming: &Ray, intersection: &Intersection) -> Ray;
}

pub struct PerfectDiffuseMaterial {
    pub color: Vec3<f32>
}

impl ReflectiveMaterial for PerfectDiffuseMaterial {
    fn bounce(&self, incoming: &Ray, intersection: &Intersection) -> Ray {
        let origin = intersection.position;
        let direction = incoming.direction; // TODO: Random in hemnisphere of normal

        Ray::new_from_air(origin, direction)
    }
}
