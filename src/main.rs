use rt::image::*;
use rt::color::*;

fn main() -> std::io::Result<()> {
    let image_width = 255;
    let image_height = 255;

    let mut image = Image::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let r = (x as f64 / (image_width - 1) as f64 * 255.999) as u8;
            let g = (y as f64 / (image_height - 1) as f64 * 255.999) as u8;
            let b = 63;

            image.add_pixel(Color::new(r, g, b));
        }
    }

    image.save_ppm("output.ppm")?;

    Ok(())
}
