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
            vec![],
            vec![
                Arc::new(Room {
                    size: 100.0,
                    square_size: 20.0,
                    colors: (Color::new(80, 80, 80), Color::new(200, 200, 200)),
                    material: Material {
                        ambient: 0.05,
                        smoothness: 200,
                        flare_intensity: 0.7,
                        specularity: 0.12,
                    },
                }),
                Arc::new(Cuboid::new(
                    Point {
                        x: -10.0,
                        y: -50.0,
                        z: -80.0,
                    },
                    Point {
                        x: 10.0,
                        y: 10.0,
                        z: 20.0,
                    },
                    Color::new(80, 80, 80),
                    Material {
                        ambient: 0.05,
                        smoothness: 100,
                        flare_intensity: 0.7,
                        specularity: 0.12,
                    },
                )),
            ],
            vec![
                Arc::new(Lamp {
                    pos: Point {
                        x: 10.0,
                        y: -30.0,
                        z: -52.0,
                    },
                    color: Color::new(255, 127, 0),
                    brightness: 3000.0,
                }),
                Arc::new(Lamp {
                    pos: Point {
                        x: -30.0,
                        y: -30.0,
                        z: -52.0,
                    },
                    color: Color::new(0, 0, 255),
                    brightness: 3000.0,
                }),
                Arc::new(Lamp {
                    pos: Point {
                        x: -10.0,
                        y: -80.0,
                        z: -75.0,
                    },
                    color: Color::new(180, 120, 255),
                    brightness: 700.0,
                }),
            ],
            3,
        ),
        cam: Camera::from_angles(
            Point {
                x: -35.0,
                y: -80.0,
                z: 60.0,
            },
            -15.0,
            15.0,
        ),
        fov: 60.0,
        resolution: (3840, 2160), //(800, 450),
    };

    let path = "image.png";
    renderer.render_and_save(path);
    open_image(path);
}
