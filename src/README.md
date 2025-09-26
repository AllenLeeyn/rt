# rt
A simple ray tracer in Rust.


steps:
- objects so we have something to render
- camera so we can draw the scene
- lights so we can change the rendering


- clear documentation for scene set up
    - Explanation on the features of your ray tracer
    - Code examples and explanations on how to:
    - create an instance of each object (a sphere, a cube, a flat plane and a cylinder)
    - change the brightness
    - change the camera position and angle

rt/
├── Cargo.toml
├── src/
│   ├── lib.rs          <- Public API of the crate
│   ├── main.rs         <- Optional: binary for rendering scenes
│   ├── scene.rs        <- Scene struct and builder
│   ├── camera.rs       <- Camera logic
│   ├── object.rs       <- Object definitions and intersection logic
│   ├── material.rs     <- Material and texture logic
│   ├── light.rs        <- Lights and shading
│   ├── ray.rs          <- Ray struct and helpers
│   ├── vector.rs       <- Vec3 implementation
│   ├── color.rs        <- Color type and conversions
│   ├── render.rs       <- Renderer logic (ray casting, recursion)
│   ├── image.rs        <- Image struct and PPM export
│   └── utils.rs        <- Misc utilities (e.g., clamp, lerp, random)
└── README.md


struct Scene {
    objects: Vec<Object>,
    lights: Vec<Light>,
    background_color: Color,
    camera: Camera,
    max_ray_depth: u32,
}

struct Light {
    position: Vec3,
    color: Color,
    intensity: f32,
    // type (point, directional, spotlight)
    // direction (for directional and spotlight)
    // angle (for spotlight)\
}
    
pub struct Image {
    pub pixels: Vec<Color>, // row-major order
}

struct Camera {
    position: Vec3,
    look_at: Vec3,
    transform: Transform,
    up: Vec3,
    fov: f32,
    aspect_ratio: f32,
    resolution: (u32, u32),
}

enum TextureType {
    Color(Color),
    Checkerboard(Color, Color, f32), // two colors and scale
    // Image(String), // path to image file (not implemented)
    gradient(Color, Color), // two colors
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
}

struct Material {
    texture: Texture,
    brightness: f32,  // 0.0–1.0
}

struct Transform {
    translation: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

enum ObjectType {
    Sphere,
    Cube,
    Plane,
    Cylinder,
}

struct Object {
    obj_type: ObjectType,
    transform: Transform,
    material: Material,
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
    depth: u32,
}

struct Intersection {
    point: Vec3,
    normal: Vec3,
    distance: f32,
    object: Object,
}

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

- 4 .ppm (800x600)
    - A scene with a sphere;
    - A scene with a flat plane and a cube with lower brightness than in the sphere image;
    - A scene with one of each of all the objects (one cube, one sphere, one cylinder and one flat plane);
    - A scene like the previous one, but with the camera in another position (thus generating the same image from a different perspective).

