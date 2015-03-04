use na::{Pnt3, Vec3};

pub struct Ray {
    // The starting point of the ray.
    pub origin: Pnt3<f32>,

    // The normalized direction of the ray.
    pub direction: Vec3<f32>,

    // The index of refraction of the material from which this ray is cast.
    pub index_of_refraction: f32
}

impl Ray {
    fn new_from_air(origin: Pnt3<f32>, direction: Vec3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            index_of_refraction: 1.000293
        }
    }
}
