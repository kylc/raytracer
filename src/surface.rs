use std::num::Float;
use std::f32::consts::{PI_2, PI};
use rand::random;
use na::{Dot, Norm, Pnt3, Vec3};
use intersection::Intersection;
use ray::Ray;

pub trait Surface {
    // Check if a ray intersects with the surface.
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Sphere {
    pub center: Pnt3<f32>,
    pub radius: f32
}

impl Sphere {
    fn get_normal(&self, point: Pnt3<f32>) -> Vec3<f32> {
        (point - self.center).normalize()
    }
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

            // Choose the intersection with the minimum distance.
            let dist = (-b + det).min(-b - det);

            Some(Intersection {
                distance: dist,
                position: ray.origin + ray.direction * dist
            })
        } else {
            None
        }
    }
}
