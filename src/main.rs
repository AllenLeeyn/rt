use rt::color::Color;
use rt::scene::*;
use rt::texture::*;
use rt::vec3::*;

fn main() -> std::io::Result<()> {
    let mut scene = Scene::new();

    scene.camera_mut().set(
        Vec3::new(2.0, 10.0, 2.0),
        Vec3::zero(),
        Point3::new(0.0, 1.0, 0.0),
        60.0,
        1.0,
        (400, 300));

    scene.set_background(Texture::Gradient(
        Color::PASTEL_BLUE, 
        Color::PASTEL_YELLOW,
        3.142/2.0
    ));

    scene.render("output2.ppm")?;

    Ok(())
}
