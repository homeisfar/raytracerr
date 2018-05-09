extern crate image;

use std::fs::File;
use image::{DynamicImage, GenericImage, Rgba};

mod scene_structs;
use scene_structs::*;

fn main() {
    println!("Hello, world!");

    // scene: Scene = {
    //     width: 640,
    //     height: 480,
    //     fov: 90.0,
    //     sphere: 
    // }
    // render()

    let img = render(&create_simple_scene());

    // let ref mut fout = File::create("output.png").unwrap();
    // img.save(fout).unwrap();
    img.save("c:\\Users\\Alih\\Desktop\\Output.png").unwrap();
}

fn create_simple_scene() -> Scene {
    let scene = Scene {
        width: 800,
        height: 200,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                r: 0.4,
                g: 1.0,
                b: 0.4,
            },
        },
    };
    scene
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba([0,0,0,255]);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime_ray(x, y, scene);

            if scene.sphere.intersects(&ray) {
                image.put_pixel(x, y, Rgba([255,0,0,255]));
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

#[test]
fn test_scene_render() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                r: 0.4,
                g: 1.0,
                b: 0.4,
            },
        },
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}