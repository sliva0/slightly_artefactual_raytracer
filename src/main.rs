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
    let renderer = Renderer {
        scene: Scene::new(
            vec![],
            vec![Arc::new(Sphere {
                pos: Point {
                    x: 75.0,
                    y: 75.0,
                    z: 75.0,
                },
                radius: 10.0,
                color: Color::new(0, 100, 0),
                material: Material {
                    ambient: 0.1,
                    smoothness: 100,
                    flare_intensity: 0.3,
                    specularity: 0.8,
                },
            })],
            vec![Arc::new(Room {
                size: 100.0,
                square_size: 20.0,
                colors: (Color::new(0, 0, 255), Color::new(255, 0, 0)),
                material: Material {
                    ambient: 0.05,
                    smoothness: 200,
                    flare_intensity: 0.4,
                    specularity: 0.3,
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
                    brightness: 500.0,
                }),
                Arc::new(Lamp {
                    pos: Point {
                        x: 80.0,
                        y: 80.0,
                        z: 60.0,
                    },
                    color: Color::new(255, 255, 255),
                    brightness: 700.0,
                }),
            ],
            1,
        ),
        cam: Camera::from_angles(
            Point {
                x: 0.0,
                y: 70.0,
                z: 0.0,
            },
            150.0,
            0.0,
        ),
        fov: 60.0,
        resolution: (800, 450),
    };

    let path = "image.png";
    renderer.render_and_save(path);
    open_image(path);
}
