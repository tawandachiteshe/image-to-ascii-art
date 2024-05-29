type Error = Box<dyn std::error::Error + Send + Sync>;
fn main() -> Result<(), Error> {
    let (_args, argv) = argmap::new().parse(std::env::args());

    if !argv.contains_key("i") || !argv.contains_key("w") || !argv.contains_key("h") {
        println!("Usage: chad-ascii -i <path> -w <width> -h <height>");
        return Ok(());
    }

    let image_path = argv.get("i").to_owned().unwrap().first().unwrap();
    let width: u32 = argv
        .get("w")
        .to_owned()
        .unwrap()
        .first()
        .unwrap()
        .parse()
        .expect("Invalid width");
    let height: u32 = argv
        .get("h")
        .to_owned()
        .unwrap()
        .first()
        .unwrap()
        .parse()
        .expect("Invalid height");

    let img = image::open(image_path)
        .unwrap()
        .flipv()
        .rotate90()
        .grayscale()
        .resize(width, height, image::imageops::FilterType::Gaussian)
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

    Ok(())
}
