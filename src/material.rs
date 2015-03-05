use std::f32::consts::PI;
use std::num::Float;
use rand::random;
use na::{Dot, Vec3};
use intersection::Intersection;
use ray::Ray;

// TODO: Boxing the enum rather than the individual components causes an ICE.
pub enum MaterialBox {
    Emissive(Box<EmissiveMaterial>),
    Reflective(Box<ReflectiveMaterial>)
}

pub struct EmissiveMaterial {
    pub emissivity: f32
}

pub trait ReflectiveMaterial {
    fn bounce(&self, incoming: &Ray, intersection: &Intersection) -> Ray;
}

pub struct PerfectDiffuseMaterial {
    pub color: Vec3<f32>
}

// Find a cosine-distributed random vector on the surface of the hemnisphere
// about the given normal.
fn random_vec_on_hemnisphere(normal: Vec3<f32>) -> Vec3<f32> {
    // Use a cosine instead of uniform distribution. This is because the diffuse
    // lighting term in the rendering equation looks like:
    //
    // L_i * cos(\theta)
    //
    // Rays that maximize the cosine term will be weighted more importantly, so
    // maybe we should just sample those rays more often anyway.
    //
    // Reference: http://www.rorydriscoll.com/2009/01/07/better-sampling/
    let r = random::<f32>().sqrt();
    let theta = 2.0 * PI * random::<f32>();

    // Convert polar to Cartesian.
    let x = r * theta.cos();
    let y = r * theta.sin();

    // TODO: Should this be another random variable, or u1?
    let z = (1.0 - random::<f32>()).sqrt();

    // Generate the vector about the xy-plane.
    let v = Vec3::new(x, y, z);

    // This vector is either within the hemnisphere or in the opposite
    // direction. Correct it if needed.
    // TODO: Does this preserve the distribution?
    if v.dot(&normal) > 0.0 {
        v
    } else {
        -v
    }
}

impl ReflectiveMaterial for PerfectDiffuseMaterial {
    fn bounce(&self, incoming: &Ray, intersection: &Intersection) -> Ray {
        let origin = intersection.position;
        let direction = random_vec_on_hemnisphere(intersection.normal);

        Ray::new_from_air(origin, direction)
    }
}

pub struct PerfectSpecularMaterial;

impl ReflectiveMaterial for PerfectSpecularMaterial {
    fn bounce(&self, incoming: &Ray, intersection: &Intersection) -> Ray {
        let direction = incoming.direction - intersection.normal
            * 2.0 * incoming.direction.dot(&intersection.normal);

        Ray::new_from_air(incoming.origin, direction)
    }
}

pub struct PerfectRefractiveMaterial {
    pub index_of_refraction: f32
}

impl ReflectiveMaterial for PerfectRefractiveMaterial {
    fn bounce(&self, incoming: &Ray, intersection: &Intersection) -> Ray {
        let origin = intersection.position;

        // TODO: Use the Fresnel equations and Snell's law to choose the right
        // refracted path.
        let direction = incoming.direction;

        Ray::new_from_air(origin, direction)
    }
}
