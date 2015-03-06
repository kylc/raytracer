use na::Vec3;
use rand::random;
use camera::Camera;
use material::MaterialBox;
use ray::Ray;
use scene::Scene;

// A rendering equation solver.
pub trait Integrator {
    fn integrate(&self, position: (f32, f32)) -> Vec3<f32>;
}

// A rendering equation solver that uses path tracing, a Monte Carlo method, to
// solve the rendering equation.
pub struct MonteCarloIntegrator<'a> {
    pub camera: &'a Camera,
    pub scene: &'a Scene<'a>,

    // Image dimensions.
    pub width: usize,
    pub height: usize,

    // How many samples should be collected for each pixel.
    pub samples_per_pixel: u32,

    // How many reflections to continue tracing before giving up.
    pub max_boundes: u32
}

impl<'a> MonteCarloIntegrator<'a> {
    fn trace_with_depth(&self, ray: &Ray, remaining_depth: u32) -> Vec3<f32> {
        // After the max recursive depth has been reached, don't bother
        // collecting any more bounces.
        if remaining_depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        match self.scene.intersects(&ray) {
            // If another object is hit, calculate the contribution of the ray.
            Some((intersection, object)) => {
                match object.material {
                    MaterialBox::Emissive(ref mat) => {
                        // TODO: What color?
                        Vec3::new(1.0, 1.0, 1.0) * mat.emissivity
                    },
                    // For a reflective surface, the contribution is calculated
                    // by indirect lighting.
                    MaterialBox::Reflective(ref mat) => {
                        // Bounce a ray off the object recursively to find the
                        // contribution.
                        let new_ray = mat.bounce(&ray, &intersection);

                        self.trace_with_depth(&new_ray, remaining_depth - 1)
                    }
                }
            },
            // Otherwise, this ray goes off into nothingness and we can stop
            // tracing.
            None => Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

impl<'a> Integrator for MonteCarloIntegrator<'a> {
    fn integrate(&self, (x, y): (f32, f32)) -> Vec3<f32> {
        let mut color = Vec3::new(0.0, 0.0, 0.0); 

        for sample in 0..self.samples_per_pixel {
            // Perturb the ray for this sample by a small amount, but keep it
            // within the pixel boundaries.
            let jitter = ((random::<f32>() - 0.5) / self.width as f32,
                          (random::<f32>() - 0.5) / self.height as f32);

            // Generate a ray from the camera origin through the current position on
            // the screen.
            let ray = self.camera.get_ray(x + jitter.0, y + jitter.1);

            let contribution = self.trace_with_depth(&ray, self.max_boundes);
            color = color + contribution / self.samples_per_pixel as f32;
        }

        color
    }
}
