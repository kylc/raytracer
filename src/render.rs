use rand::random;
use na::{DMat, Indexable, Vec3};
use camera::Camera;
use integrator::{Integrator, MonteCarloIntegrator};
use scene::Scene;

pub struct RenderProperties {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32, // TODO: This shouldn't be in properties
    pub max_bounces: u32
}

pub fn render(properties: &RenderProperties, camera: &Camera, scene: &Scene) -> DMat<Vec3<f32>> {
    let mut screen = DMat::new_zeros(properties.width, properties.height);
    let integrator = MonteCarloIntegrator {
        camera: &camera,
        width: properties.width,
        height: properties.height,
        samples_per_pixel: properties.samples_per_pixel,
        max_boundes: properties.max_bounces,
        scene: scene
    };

    for x in 0..properties.width {
        for y in 0..properties.height {
            // Scale the axes to be on the range of [-1, 1]. Also invert the y
            // axis, as positive y needs to be towards the top of the screen.
            let normalized_position = ( 1.0 * (x as f32 / properties.width as f32 * 2.0 - 1.0),
                                       -1.0 * (y as f32 / properties.height as f32 * 2.0 - 1.0));

            let color = integrator.integrate((normalized_position.0, normalized_position.1));

            screen.set((x, y), color);
        }
    }

    screen
}
