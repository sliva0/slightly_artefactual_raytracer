use std::{process::Command, sync::Arc};

mod types;
use types::*;

fn open_image(path: &str) {
    if let Some(opener) = {
        if cfg!(windows) {
            Some("C:/Windows/explorer.exe")
        } else if cfg!(unix) {
            Some("xdg-open")
        } else {
            None
        }
    } {
        Command::new(opener).arg(path).spawn().unwrap();
    }
}

fn main() {
    let renderer = SubsamplingRenderer {
        scene: Scene::new(
            vec![],
            vec![Arc::new(Sphere::new(
                Point {
                    x: 65.0,
                    y: 75.0,
                    z: 75.0,
                },
                10.0,
                Color::new(0, 50, 0),
                Material {
                    ambient: 0.2,
                    diffuse: 1.0,                    
                    specular: 0.3,
                    shininess: 100,
                    m_type: RefractiveType { index: 1.5 },
                },
            ))],
            vec![Arc::new(Room {
                size: 100.0,
                square_size: 20.0,
                colors: (Color::new(0, 0, 255), Color::new(255, 0, 0)),
                material: Material {
                    ambient: 0.05,
                    diffuse: 1.0,
                    specular: 0.6,
                    shininess: 200,
                    m_type: DefaultType,
                },
            })],
            vec![
                Arc::new(Lamp {
                    pos: Point {
                        x: 60.0,
                        y: 60.0,
                        z: 70.0,
                    },
                    color: Color::new(255, 255, 0),
                    brightness: 800.0,
                }),
                Arc::new(Lamp {
                    pos: Point {
                        x: 80.0,
                        y: 80.0,
                        z: 60.0,
                    },
                    color: Color::new(255, 255, 255),
                    brightness: 500.0,
                }),
            ],
            2,
        ),
        cam: Camera::from_angles(
            Point {
                x: 0.0,
                y: 70.0,
                z: 0.0,
            },
            -150.0,
            0.0,
        ),
        fov: 60.0,
        resolution:(480, 270), //(3840, 2160),
        subsampling_limit: 0.005,
        supersampling_multiplier: 1, 
    };

    let path = "image.png";
    renderer.render_and_save(path, subsampling_func(4));
    open_image(path);
}
