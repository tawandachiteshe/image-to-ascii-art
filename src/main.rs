type Error = Box<dyn std::error::Error + Send + Sync>;
fn main() -> Result<(), Error> {
    let (_args, argv) = argmap::new().parse(std::env::args());

    if !argv.contains_key("i")
        || !argv.contains_key("w")
        || !argv.contains_key("h")
        || !argv.contains_key("o")
    {
        println!("Usage: chad-ascii -i <path> -w <width> -h <height> -o <txt_path>");
        return Ok(());
    }

    let is_dev = argv.contains_key("d");

    let image_path = argv.get("i").to_owned().unwrap().first().unwrap();

    let output_path = argv
        .get("o")
        .to_owned()
        .unwrap()
        .first()
        .unwrap()
        .to_string();

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

    if is_dev {
        string_builder.push_str("## Image to ASCII\n\n");
        string_builder.push_str("### How to build \n \n");

        string_builder.push_str("```bash\n cargo buld \n ```\n\n");

        string_builder.push_str("### How to run \n \n");
        string_builder.push_str(
            "```bash\n cargo run -- -i <path> -w <width> -h <height> -o <txt_path> \n ```\n\n",
        );

        string_builder.push_str("### Output \n \n");

        string_builder.push_str("```bash \n");
    }

    let ascii = " .,:;i1tfLCG08@";
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);

            let index = (pixel[0] as f32 / 255.0 * (ascii.len() - 1) as f32).round() as usize;
            string_builder.push_str(&ascii[index..index + 1]);
        }

        string_builder.push_str("\n");
    }

    if is_dev {
        string_builder.push_str("```");
    }

    println!("{}", string_builder);

    std::fs::write(output_path, string_builder).unwrap();

    Ok(())
}
