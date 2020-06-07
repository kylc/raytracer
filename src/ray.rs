use na::{Point3, Vector3};

pub static INDEX_OF_REFRACTION_AIR: f32 = 1.000293;

pub struct Ray {
    // The starting point of the ray.
    pub origin: Point3<f32>,

    // The normalized direction of the ray.
    pub direction: Vector3<f32>,

    // The index of refraction of the material from which this ray is cast.
    pub index_of_refraction: f32,
}

impl Ray {
    // Create a new ray from the given origin in the given direction where the
    // index of refraction is assumed to be that of air.
    pub fn new_from_air(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin,
            direction,
            index_of_refraction: INDEX_OF_REFRACTION_AIR,
        }
    }
}
