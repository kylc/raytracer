use std::num::Float;
use na::{Cross, Dot, Norm, Pnt3, Vec3};
use intersection::Intersection;
use ray::Ray;

pub trait Surface {
    // Check if a ray intersects with the surface.
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
    fn normal_towards(&self, point: Pnt3<f32>) -> Vec3<f32>;
}

pub struct Sphere {
    pub center: Pnt3<f32>,
    pub radius: f32
}

impl Surface for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Equations taken from http://en.wikipedia.org/wiki/Line–sphere_intersection
        let oc = ray.origin - self.center;

        // Find quadratic equation coefficients.
        let a = ray.direction.sqnorm();
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.sqnorm() - self.radius.powi(2);

        let det2 = b.powi(2) - 4.0 * a * c;

        // Three options based on the value of det^2:
        // 1) det^2 < 0.0: no solutions
        // 2) det^2 = 0.0: one solution
        // 3) det^2 > 0.0: two solutions (pick the closest)
        if det2 >= 0.0 {
            // Only sqrt det once we know we have to.
            let det = det2.sqrt();

            // Calculate the two solutions to the quadratic equation.
            let t1 = (-b + det) / (2.0 * a);
            let t2 = (-b - det) / (2.0 * a);

            // Choose the positive intersection with the minimum distance.
            let dist = if t1 > 0.0 && t1 < t2 {
                Some(t1)
            } else if t2 > 0.0 && t2 < t1 {
                Some(t2)
            } else {
                None
            };

            dist.map(|d| Intersection::new_from_distance(d, ray, self))
        } else {
            None
        }
    }

    fn normal_towards(&self, point: Pnt3<f32>) -> Vec3<f32> {
        // The normal is a ray traced from the center of the sphere to the given
        // point, normalized.
        (point - self.center).normalize()
    }
}
