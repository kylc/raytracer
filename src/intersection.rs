use na::{Pnt3, Vec3};
use ray::Ray;
use surface::Surface;

pub struct Intersection {
    // The distance from the ray origin of the intersection.
    pub distance: f32,

    // The point at which the intersection occured.
    pub position: Pnt3<f32>,

    // The normal on the surface at which the intersection occured.
    pub normal: Vec3<f32>
}

impl Intersection {
    // Create a new ray from a given intersection distance, ray, and surface.
    pub fn new_from_distance(distance: f32, ray: &Ray, surface: &Surface) -> Intersection {
        let position = ray.origin + ray.direction * distance;
        let normal = surface.normal_towards(position);

        Intersection {
            distance: distance,
            position: position,
            normal: normal
        }
    }
}
