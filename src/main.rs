use std::process::Command;

mod types;
use types::*;

fn open_image(path: &str) {
    match {
        if cfg!(windows) {
            Some("C:/Windows/explorer.exe")
        } else if cfg!(unix) {
            Some("xdg-open")
        } else {
            None
        }
    } {
        Some(opener) => {
            Command::new(opener).arg(path).spawn().unwrap();
        }
        None => (),
    }
}

fn main() {
    let renderer = Renderer {
        scene: Scene::new(
            vec![],
            vec![],
            vec![Arc::new(objects::TracingRoom {
                size: 100.0,
                square_size: 20.0,
                colors: (Color::new(0, 0, 255), Color::new(255, 0, 0)),
            })],
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
        resolution: (640, 360),
    };

    let path = "image.png";
    renderer.render_and_save(path);
    open_image(path);
}
