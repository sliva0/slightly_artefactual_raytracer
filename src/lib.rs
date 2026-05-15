mod basic;
pub use basic::*;

mod objects;
pub use objects::*;

mod camera;
pub use camera::Camera;

mod scene_objects;
pub use scene_objects::SceneObjects;

mod renderers;
pub use renderers::*;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn foo() {
    let renderer = SubsamplingRenderer {
        scene: Scene {
            objs: SceneObjects::new(
                vec![],
                vec![Sphere::new(
                    Point::new(65.0, 75.0, 75.0),
                    10.0,
                    Color::new(0, 50, 0),
                    Material {
                        ambient: 0.2,
                        diffuse: 1.0,
                        specular: 0.3,
                        shininess: 100,
                        m_type: MaterialType::Refractive {
                            surface_transparency: 1.0,
                            index: 1.5,
                        },
                    },
                )],
                vec![Room::new(
                    100.0,
                    20.0,
                    (Color::new(0, 0, 255), Color::new(255, 0, 0)),
                    Material {
                        ambient: 0.05,
                        diffuse: 1.0,
                        specular: 0.6,
                        shininess: 200,
                        m_type: MaterialType::Reflective { reflectance: 0.3 },
                    },
                )],
                vec![
                    Lamp::new(Point::new(60.0, 60.0, 70.0), Color::new(255, 255, 0), 800.0),
                    Lamp::new(
                        Point::new(80.0, 80.0, 60.0),
                        Color::new(255, 255, 255),
                        500.0,
                    ),
                ],
                2,
            ),
            cam: Camera::from_angles(Point::new(0.0, 70.0, 0.0), -150.0, 0.0),
            fov: 60.0,
            resolution: [480, 270], //[3840, 2160],
        },
        subsampling_limit: 0.005,
        supersampling_multiplier: 1,
    };

    let path = "image.png";
    renderer.render(subsampling_func(4)).save(path).unwrap();
}
