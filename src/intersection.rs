use na::{Point3, Vector3};
use ray::Ray;
use surface::Surface;

#[derive(Clone, Copy)]
pub struct Intersection {
    // The distance from the ray origin of the intersection.
    pub distance: f32,

    // The point at which the intersection occured.
    pub position: Point3<f32>,

    // The normal on the surface at which the intersection occured.
    pub normal: Vector3<f32>,
}

impl Intersection {
    // Create a new ray from a given intersection distance, ray, and surface.
    pub fn new_from_distance(distance: f32, ray: &Ray, surface: &dyn Surface) -> Intersection {
        let position = ray.origin + ray.direction * distance;
        let normal = surface.normal_towards(position);

        Intersection {
            distance,
            position,
            normal,
        }
    }
}
