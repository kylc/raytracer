extern crate image;
extern crate nalgebra as na;
extern crate renderer;

use na::{DMatrix, Point3, Vector3};
use renderer::*;

fn main() {
    let sphere1 = surface::Sphere {
        center: Point3::new(-0.75, 0.25, 0.75),
        radius: 0.25,
    };
    let sphere2 = surface::Sphere {
        center: Point3::new(0.75, 0.25, 0.75),
        radius: 0.25,
    };
    let cool_light = surface::Sphere {
        center: Point3::new(-0.35, 1.0, -0.2),
        radius: 0.1,
    };
    let sphere3 = surface::Sphere {
        center: Point3::new(0.0, 0.45, 0.0),
        radius: 0.25,
    };
    let back = surface::Plane {
        normal: Vector3::new(0.0, 0.0, -1.0),
        offset: 1.0,
    };
    let front = surface::Plane {
        normal: Vector3::new(0.0, 0.0, 1.0),
        offset: -2.0,
    };
    let floor = surface::Plane {
        normal: Vector3::new(0.0, 1.0, 0.0),
        offset: 0.0,
    };
    let left = surface::Plane {
        normal: Vector3::new(1.0, 0.0, 0.0),
        offset: -1.0,
    };
    let right = surface::Plane {
        normal: Vector3::new(-1.0, 0.0, 0.0),
        offset: 1.0,
    };
    let light = surface::Plane {
        normal: Vector3::new(0.0, -1.5, 0.0),
        offset: 2.0,
    };

    let scene = scene::Scene {
        objects: vec![
            // object::Object {
            //     surface: Box::new(sphere1),
            //     material: material::MaterialBox::Reflective(Box::new(material::PerfectDiffuseMaterial {
            //         color: Vector3::new(1.0, 1.0, 1.0)
            //     }))
            // },
            // object::Object {
            //     surface: Box::new(sphere2),
            //     material: material::MaterialBox::Reflective(Box::new(material::PerfectSpecularMaterial))
            // },
            object::Object {
                surface: Box::new(sphere3),
                material: material::MaterialBox::Reflective(Box::new(
                    material::PerfectRefractiveMaterial {
                        index_of_refraction: 1.440,
                        reflect_prob: 0.1,
                    },
                )),
            },
            object::Object {
                surface: Box::new(back),
                material: material::MaterialBox::Reflective(Box::new(
                    material::PerfectDiffuseMaterial {
                        color: Vector3::new(1.0, 1.0, 1.0),
                    },
                )),
            },
            object::Object {
                surface: Box::new(front),
                material: material::MaterialBox::Reflective(Box::new(
                    material::PerfectDiffuseMaterial {
                        color: Vector3::new(1.0, 1.0, 1.0),
                    },
                )),
            },
            object::Object {
                surface: Box::new(floor),
                material: material::MaterialBox::Reflective(Box::new(
                    material::PerfectDiffuseMaterial {
                        color: Vector3::new(1.0, 1.0, 1.0),
                    },
                )),
            },
            object::Object {
                surface: Box::new(left),
                material: material::MaterialBox::Reflective(Box::new(
                    material::PerfectDiffuseMaterial {
                        color: Vector3::new(0.5, 0.5, 0.9),
                    },
                )),
            },
            object::Object {
                surface: Box::new(right),
                material: material::MaterialBox::Reflective(Box::new(
                    material::PerfectDiffuseMaterial {
                        color: Vector3::new(0.5, 0.9, 0.5),
                    },
                )),
            },
            object::Object {
                surface: Box::new(cool_light),
                material: material::MaterialBox::Emissive(Box::new(material::EmissiveMaterial {
                    emissivity: 50.0,
                })),
            },
            // object::Object {
            //     surface: Box::new(light),
            //     material: material::MaterialBox::Emissive(Box::new(material::EmissiveMaterial {
            //         emissivity: 0.5
            //     }))
            // },
        ],
    };

    let camera = camera::Camera {
        position: Point3::new(0.0, 0.3, -1.0),
        direction: Vector3::new(0.0, 0.0, 1.0),
    };

    let props = render::RenderProperties {
        width: 500,
        height: 500,
    };

    let integrator = integrator::MonteCarloIntegrator {
        camera: &camera,
        scene: &scene,
        width: props.width,
        height: props.height,
        samples_per_pixel: 1000,
        max_bounces: 5,
    };

    let screen = render::render(&props, &integrator);
    write_image("test.ppm", &screen);
}

fn write_image(file_name: &str, screen: &DMatrix<Vector3<f32>>) {
    let size = (screen.ncols() as u32, screen.nrows() as u32);

    let mut imbuf = image::ImageBuffer::new(size.0, size.1);
    for x in 0..size.0 {
        for y in 0..size.1 {
            let position = (x as usize, y as usize);

            let pixel = image::Rgb([
                (screen[position].x.min(1.0) * 255.0) as u8,
                (screen[position].y.min(1.0) * 255.0) as u8,
                (screen[position].z.min(1.0) * 255.0) as u8,
            ]);
            imbuf.put_pixel(x, y, pixel);
        }
    }

    let _ = imbuf.save(file_name);
}
