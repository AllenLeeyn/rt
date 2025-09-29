use rt::color::Color;
use rt::cube::Cube;
use rt::cylinder::Cylinder;
use rt::image::Image;
use rt::light::Light;
use rt::plane::Plane;
use rt::sphere::Sphere;
use rt::scene::*;
use rt::texture::*;
use rt::vec3::*;

fn main() -> std::io::Result<()> {
    let mut scene = Scene::new();

    scene.camera_mut().set(
        Vec3::new(5.0, 4.0, 3.0),
        Vec3::ZERO,
        Point3::new(0.0, 1.0, 0.2),
        70.0,
        1.0,
        (800, 600));

    scene.set_background(Texture::Gradient(
        Color::PASTEL_BLUE, 
        Color::PASTEL_YELLOW,
        3.142/2.0
    ));
    
    scene.add_object(Cube::new(
        Point3::new(0.0, 1.0, -1.0), // center (1.0 y lifts it above plane)
        2.0, // size (width, height, depth)
        Texture::Checkerboard(Color::MAGENTA, Color::PASTEL_CYAN, 2.0),
    ));

    scene.add_object(Cylinder::new(
        Point3::new(3.0, 0.0, 1.0), // base center
        0.5,
        1.0,
        Texture::Gradient(Color::YELLOW, Color::BLUE, 3.142),
    ));

    scene.add_object(Sphere::new(
        Point3::new(-1.0, 1.0, 2.0),
        1.0,
        Texture::Gradient(Color::PASTEL_PURPLE, Color::RED, 0.0),
    ));

    // Add plane to the scene
    let image = Image::load("assets/bg.png")?;
    scene.add_object(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(20.0, 0.0, 20.0),
        Texture::Image(image),
    ));

    scene.add_light(Light::new(
        Point3::new(3.0, 5.0, 5.0),
        Color::WHITE,
        50.0,
        16,
        0.1,
        100.0,
    ));

    scene.render("output2.ppm")?;

    Ok(())
}
