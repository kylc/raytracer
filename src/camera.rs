use na::{Point3, Vector3};
use ray::Ray;

pub struct Camera {
    // Location of the camera within the scene.
    pub position: Point3<f32>,

    // Direction that the camera is facing.
    pub direction: Vector3<f32>,
}

impl Camera {
    // Returns a ray passing through the specified position, where the domain of
    // the position is from -1.0 to 1.0.
    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        // TODO: This makes assumptions about the field of view.
        let direction = (self.direction + Vector3::new(x, y, 0.0)).normalize();

        Ray::new_from_air(self.position, direction)
    }
}
