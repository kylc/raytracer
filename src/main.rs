#![feature(old_io, old_path)]

extern crate "nalgebra" as na;
extern crate image;
extern crate renderer;

use std::old_io::File;
use std::num::Float;
use na::{DMat, Indexable, Pnt3, Vec3};
use renderer::*;

fn main() {
    let props = render::RenderProperties {
        width: 200,
        height: 200,
        samples_per_pixel: 200,
        max_bounces: 5
    };

    let camera = camera::Camera {
        position: Pnt3::new(0.0, 0.0, -1.0),
        direction: Vec3::new(0.0, 0.0, 1.0)
    };

    let sphere1 = surface::Sphere {
        center: Pnt3::new(0.0, 0.0, 0.0),
        radius: 0.5
    };
    let light = surface::Sphere {
        center: Pnt3::new(1.0, 1.3, -1.0),
        radius: 0.5
    };

    let scene = scene::Scene {
        objects: vec![
            object::Object {
                surface: &sphere1,
                material: material::MaterialBox::Reflective(Box::new(material::PerfectDiffuseMaterial {
                    color: Vec3::new(1.0, 1.0, 1.0),
                }))
            },
            object::Object {
                surface: &light,
                material: material::MaterialBox::Emissive(Box::new(material::EmissiveMaterial {
                    emissivity: 1.0
                }))
            },
        ]
    };

    let screen = render::render(&props, &camera, &scene);
    write_image("test.ppm", &screen);
}

fn write_image(file_name: &str, screen: &DMat<Vec3<f32>>) {
    let size = (screen.ncols() as u32, screen.nrows() as u32);

    let mut imbuf = image::ImageBuffer::new(size.0, size.1);
    let mut fout = File::create(&Path::new(file_name)).unwrap();

    for x in (0..size.0) {
        for y in (0..size.1) {
            let position = (x as usize, y as usize);

            let pixel = image::Rgb([
                (screen.at(position).x.min(1.0) * 255.0) as u8,
                (screen.at(position).y.min(1.0) * 255.0) as u8,
                (screen.at(position).z.min(1.0) * 255.0) as u8
            ]);
            imbuf.put_pixel(x, y, pixel);
        }
    }

    let _ = image::ImageRgb8(imbuf).save(&mut fout, image::PPM);
}
