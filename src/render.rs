use integrator::Integrator;
use na::{DMatrix, Vector3};

pub struct RenderProperties {
    pub width: usize,
    pub height: usize,
}

pub fn render(properties: &RenderProperties, integrator: &dyn Integrator) -> DMatrix<Vector3<f32>> {
    let mut screen = DMatrix::zeros(properties.width, properties.height);

    for x in 0..properties.width {
        for y in 0..properties.height {
            // Scale the axes to be on the range of [-1, 1]. Also invert the y
            // axis, as positive y needs to be towards the top of the screen.
            let normalized_position = (
                1.0 * (x as f32 / properties.width as f32 * 2.0 - 1.0),
                -1.0 * (y as f32 / properties.height as f32 * 2.0 - 1.0),
            );

            let color = integrator.integrate((normalized_position.0, normalized_position.1));

            screen[(x, y)] = color;
        }

        let current_pixels = x;
        let total_pixels = properties.width;

        println!(
            "{}% finished",
            current_pixels as f32 / total_pixels as f32 * 100.0
        );
    }

    screen
}
