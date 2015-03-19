use std::f32::consts::PI;
use std::num::Float;
use rand::random;
use na::{Dot, Norm, Vec3};
use intersection::Intersection;
use ray::{INDEX_OF_REFRACTION_AIR, Ray};

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

    // TODO: Are there some materials where the color depends on the incoming
    // ray or the intersection? Subsurface scattering perhaps?
    fn color(&self) -> Vec3<f32>;
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
    let v = Vec3::new(x, y, z).normalize();

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

    fn color(&self) -> Vec3<f32> {
        self.color
    }
}

pub struct PerfectSpecularMaterial;

impl ReflectiveMaterial for PerfectSpecularMaterial {
    fn bounce(&self, incoming: &Ray, intersection: &Intersection) -> Ray {
        let direction = (incoming.direction - intersection.normal
            * 2.0 * incoming.direction.dot(&intersection.normal)).normalize();

        Ray::new_from_air(incoming.origin, direction)
    }

    fn color(&self) -> Vec3<f32> {
        // TODO: Correct color?
        Vec3::new(1.0, 1.0, 1.0)
    }
}

pub struct PerfectRefractiveMaterial {
    pub index_of_refraction: f32,

    // Probability that a ray will be reflected rather than refracted.
    pub reflect_prob: f32
}

impl ReflectiveMaterial for PerfectRefractiveMaterial {
    fn bounce(&self, incoming: &Ray, intersection: &Intersection) -> Ray {
        let refract_or_reflect = random::<f32>();
        if refract_or_reflect > self.reflect_prob {
            // Refract

            // Equations from http://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf

            // TODO: Index of refraction varies with the light's wavelength.

            // Compare the direction of the incoming ray to the direction of the
            // normal to see if the ray is entering or exiting the material.
            let dir = incoming.direction.dot(&intersection.normal);

            let (n1, n2) = if dir >= 0.0 {
                // Normal and incoming ray are in the same direction, so the ray is
                // exiting the refractive material.

                // TODO: Entering... air?
                (self.index_of_refraction, INDEX_OF_REFRACTION_AIR)
            } else {
                // Entering refractive material.
                (incoming.index_of_refraction, self.index_of_refraction)
            };

            // Apply Snell's law to determine the direction of the new ray.
            let n_ratio = n1 / n2;
            let cost = -incoming.direction.dot(&intersection.normal);
            let sin2t = n_ratio.powi(2) * (1.0 - cost.powi(2));

            let t1 = incoming.direction * n_ratio;
            let t2 = intersection.normal * (n_ratio * cost - (1.0 - sin2t).sqrt());

            // The final direction is the sum of the vector in the original ray
            // direction (t1) and the vector in the direction of the surface normal
            // (t2).
            let direction = (t1 + t2).normalize();

            Ray {
                origin: intersection.position,
                direction: direction,
                index_of_refraction: n2
            }
        } else {
            // Reflect
            let reflect_mat = PerfectSpecularMaterial;
            reflect_mat.bounce(incoming, intersection)
        }
    }

    fn color(&self) -> Vec3<f32> {
        // TODO: Correct color?
        Vec3::new(1.0, 1.0, 1.0)
    }
}
