use std::f32::consts::PI;
use std::num::Float;
use rand::random;
use na::Vec3;
use intersection::Intersection;
use ray::Ray;

pub enum Material {
    Emissive(EmissiveMaterial),
    Reflective(ReflectiveMaterial)
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

    let x = r * theta.cos();
    let y = r * theta.sin();

    // TODO: Should this be another random variable, or u1?
    let z = (1.0 - random::<f32>()).sqrt();

    Vec3::new(x, y, z)
}

impl ReflectiveMaterial for PerfectDiffuseMaterial {
    fn bounce(&self, incoming: &Ray, intersection: &Intersection) -> Ray {
        let origin = intersection.position;
        let direction = random_vec_on_hemnisphere(intersection.normal);

        Ray::new_from_air(origin, direction)
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
