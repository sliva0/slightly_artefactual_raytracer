use std::process::Command;

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
            vec![],
            vec![
                Box::new(Room {
                    size: 100.0,
                    square_size: 20.0,
                    colors: (Color::new(80, 80, 80), Color::new(200, 200, 200)),
                    material: Material {
                        ambient: 0.05,
                        diffuse: 1.0,
                        specular: 0.7,
                        shininess: 200,
                        m_type: ReflectiveType { reflectance: 0.3 },
                    },
                }),
                Box::new(Cuboid::new(
                    Point::new(-10.0, -50.0, -80.0),
                    Point::new(10.0, 10.0, 20.0),
                    Color::new(80, 80, 80),
                    Material {
                        ambient: 0.05,
                        diffuse: 1.0,
                        specular: 0.7,
                        shininess: 100,
                        m_type: ReflectiveType { reflectance: 0.3 },
                    },
                )),
            ],
            vec![
                Box::new(Lamp {
                    pos: Point::new(10.0, -30.0, -52.0),
                    color: Color::new(255, 127, 0),
                    brightness: 3000.0,
                }),
                Box::new(Lamp {
                    pos: Point::new(-30.0, -30.0, -52.0),
                    color: Color::new(0, 0, 255),
                    brightness: 3000.0,
                }),
                Box::new(Lamp {
                    pos: Point::new(-10.0, -80.0, -75.0),
                    color: Color::new(180, 120, 255),
                    brightness: 700.0,
                }),
            ],
            1,
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
        resolution: (800, 450), //(3840, 2160),
        subsampling_limit: 0.05,
        supersampling_multiplier: 1,
    };

    let path = "image.png";
    renderer.render_and_save(path, subsampling_func(5));
    open_image(path);
}
