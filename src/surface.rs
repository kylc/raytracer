use std::num::Float;
use std::f32::consts::{PI_2, PI};
use rand::random;
use na::{Dot, Norm, Pnt3, Vec3};
use intersection::Intersection;
use ray::Ray;

trait Surface {
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
        let a = ray.direction.sqnorm();
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.sqnorm() - self.radius.powi(2);

        // x = -b +- sqrt(b^2 - 4ac) / 2a
        let det2 = b.powi(2) - 4.0 * a * c;
        if det2 >= 0.0 {
            let det = det2.sqrt();
            let dist = (-b + det).min(-b - det);

            Some(Intersection {
                distance: dist,
                point: ray.origin + ray.direction * dist
            })
        } else {
            None
        }
    }
}

#[test]
fn sphere_test() {
    // Generate a sphere
    let sphere1 = Sphere {
        center: Pnt3::new(5.0, 10.0, 0.0),
        radius: 1.0
    };

    // Find a bunch of random points in the sphere
    for i in 0..100 {
        // Generate random spherical coordinates
        let r = random::<f32>();
        let theta = random::<f32>() * PI;
        let phi = random::<f32>() * PI_2;

        // Convert to Cartesian
        let pnt = Pnt3::new(sphere1.center.x + r * theta.sin() * phi.cos(),
                            sphere1.center.y + r * theta.sin() * phi.sin(),
                            sphere1.center.z + r * theta.cos());
        
        // Cast a ray to this point from somewhere else
        let origin = Pnt3::new(0.0, 0.0, -100.0);
        let ray = Ray::new_from_air(origin, pnt - origin);

        // Check for intersection
        assert!(sphere1.intersects(&ray).is_some());
    }

    // // Find a bunch of random points outside the sphere
    // for i in 0..100 {
    //     // Generate random spherical coordinates
    //     let r = random::<f32>() + 1.1;
    //     let theta = random::<f32>() * PI;
    //     let phi = random::<f32>() * PI_2;
    //
    //     // Convert to Cartesian
    //     let pnt = Pnt3::new(sphere1.center.x + r * theta.sin() * phi.cos(),
    //                         sphere1.center.y + r * theta.sin() * phi.sin(),
    //                         sphere1.center.z + r * theta.cos());
    //     
    //     // Cast a ray to this point from somewhere else
    //     let origin = (sphere1.get_normal(pnt) * 100.0).to_pnt();
    //     let ray = Ray::new_from_air(origin, pnt - origin);
    //
    //     println!("Testing from {}, {}, {}", origin.x, origin.y, origin.z);
    //     println!("Testing to {}, {}, {}", pnt.x, pnt.y, pnt.z);
    //     // Check for intersection
    //     assert!(!sphere1.intersects(&ray));
    // }
}
