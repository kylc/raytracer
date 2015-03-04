use na::{Pnt3, Vec3};

pub struct Intersection {
    // The distance from the ray origin of the intersection.
    distance: f32,

    // The point at which the intersection occured.
    point: Vec3<f32>,
}
