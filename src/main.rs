use std::{process::Command, sync::Arc};

mod types;
use types::{objects::*, *};

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
            vec![],
            vec![Arc::new(TracingRoom {
                size: 100.0,
                square_size: 20.0,
                colors: (Color::new(0, 0, 255), Color::new(255, 0, 0)),
                material: Material {
                    ambient: 0.05,
                    smoothness: 200,
                    flare_intensity: 0.5,
                    specularity: 0.2,
                },
            })],
            vec![
                Arc::new(Lamp {
                    pos: Point {
                        x: -70.0,
                        y: 60.0,
                        z: -60.0,
                    },
                    color: Color::new(255, 255, 0),
                    brightness: 700.0,
                }),
                Arc::new(Lamp {
                    pos: Point {
                        x: -60.0,
                        y: 80.0,
                        z: -80.0,
                    },
                    color: Color::new(255, 255, 255),
                    brightness: 700.0,
                }),
            ],
            0,
        ),
        cam: Camera::from_angles(
            Point {
                x: 0.0,
                y: 70.0,
                z: 0.0,
            },
            -30.0,
            0.0,
        ),
        fov: 60.0,
        resolution: (800, 450),
    };

    let path = "image.png";
    renderer.render_and_save(path);
    open_image(path);
}
