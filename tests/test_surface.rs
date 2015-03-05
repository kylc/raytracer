#![feature(core)]

// TODO: Why do I need these here if they're already in tests/lib.rs?
extern crate rand;
extern crate "nalgebra" as na;
extern crate renderer;

use std::num::Float;
use std::f32::consts::{PI, PI_2};
use rand::random;
use na::{Pnt3};
use renderer::surface::{Surface, Sphere};
use renderer::ray::Ray;

#[test]
fn sphere_test() {
    // Generate a sphere
    let sphere1 = Sphere {
        center: Pnt3::new(5.0, 10.0, 0.0),
        radius: 1.0
    };

    // Find a bunch of random points in the sphere
    for _ in 0..100 {
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

    // TODO: Need to pick a point outside the sphere (done), but also choose a
    // vantage point intelligently.
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
