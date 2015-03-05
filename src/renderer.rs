use rand::random;
use na::{DMat, Vec3};

pub struct RenderProperties {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32,
    pub max_bounces: u32
}

pub fn render(properties: &RenderProperties) -> DMat<Vec3<f32>> {
    let mut screen = DMat::new_zeros(properties.width, properties.height);

    for x in 0..properties.width {
        for y in 0..properties.height {
            // Scale the axes to be on the range of [-1, 1]. Also invert the y
            // axis, as positive y needs to be towards the top of the screen.
            let normalized_position = ( 1.0 * (x as f32 / properties.width as f32 * 2.0 - 1.0),
                                       -1.0 * (y as f32 / properties.height as f32 * 2.0 - 1.0));

            // Perturb the ray for this sample by a small amount.
            let jitter = ((random::<f32>() - 0.5) / properties.width as f32,
                          (random::<f32>() - 0.5) / properties.height as f32);

            // Generate a ray from the camera origin through the current
            // position on the screen (plus some jitter).
            // let ray = camera.get_ray(normalized_position.0 + jitter.0,
            //                          normalized_position.1 + jitter.1);
        }
    }

    screen
}
