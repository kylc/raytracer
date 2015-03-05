use na::Vec3;
use camera::Camera;
use ray::Ray;

// A rendering equation solver.
pub trait Integrator {
    fn integrate(&self, position: (f32, f32)) -> Vec3<f32>;
}

// A rendering equation solver that uses path tracing, a Monte Carlo method, to
// solve the rendering equation.
pub struct MonteCarloIntegrator<'a> {
    pub camera: &'a Camera,

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
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl<'a> Integrator for MonteCarloIntegrator<'a> {
    fn integrate(&self, (x, y): (f32, f32)) -> Vec3<f32> {
        // Generate a ray from the camera origin through the current position on
        // the screen.
        let ray = self.camera.get_ray(x, y);

        let mut color = Vec3::new(0.0, 0.0, 0.0); 
        for sample in 0..self.samples_per_pixel {
        }

        color
    }
}
