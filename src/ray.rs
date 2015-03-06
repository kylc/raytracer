use na::{Pnt3, Vec3};

pub static INDEX_OF_REFRACTION_AIR: f32 = 1.000293;

pub struct Ray {
    // The starting point of the ray.
    pub origin: Pnt3<f32>,

    // The normalized direction of the ray.
    pub direction: Vec3<f32>,

    // The index of refraction of the material from which this ray is cast.
    pub index_of_refraction: f32
}

impl Ray {
    // Create a new ray from the given origin in the given direction where the
    // index of refraction is assumed to be that of air.
    pub fn new_from_air(origin: Pnt3<f32>, direction: Vec3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            index_of_refraction: INDEX_OF_REFRACTION_AIR
        }
    }
}
