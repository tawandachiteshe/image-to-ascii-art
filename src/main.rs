use image::{GenericImageView, PixelWithColorType};

fn main() {
    let _img = image::open("chad.jpg").unwrap();

    let mut _img2 = _img.flipv().rotate90().grayscale();
    let img = _img2
        .resize(128, 128, image::imageops::FilterType::Gaussian)
        .into_luma8();

    // img.save("lisa.resize.jpg").unwrap();
    let mut string_builder = String::new();

    let ascii = " .,:;i1tfLCG08@";

    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);

            let index = (pixel[0] as f32 / 255.0 * (ascii.len() - 1) as f32).round() as usize;
            string_builder.push_str(&ascii[index..index + 1]);
        }

        string_builder.push_str("\n");
    }

    println!("{}", string_builder);
}
