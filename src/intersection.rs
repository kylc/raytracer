use na::Pnt3;

pub struct Intersection {
    // The distance from the ray origin of the intersection.
    pub distance: f32,

    // The point at which the intersection occured.
    pub point: Pnt3<f32>,
}
