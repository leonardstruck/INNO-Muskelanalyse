pub fn generate_tumbnail(path: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // generate thumbnail
    let thumbnail_buffer = {
        let file = std::fs::File::open(path).expect("Failed to open file");

        let reader = std::io::BufReader::new(file);

        let mut image_reader = image::io::Reader::new(reader)
            .with_guessed_format()
            .expect("Failed to read micrograph");

        image_reader.no_limits();

        let image = image_reader.decode().unwrap();

        // get aspect ratio of micrograph
        let aspect_ratio = image.width() as f32 / image.height() as f32;

        // calculate thumbnail size
        let thumbnail_size = if aspect_ratio > 1.0 {
            (512, (512.0 / aspect_ratio) as u32)
        } else {
            (((512.0 * aspect_ratio) as u32), 512)
        };

        // create thumbnail
        let thumbnail = image
            .resize(
                thumbnail_size.0,
                thumbnail_size.1,
                image::imageops::FilterType::Lanczos3,
            )
            .to_rgb16();

        thumbnail
    };

    // convert thumbnail to binary vec<u8>
    let thumbnail_bin = {
        let mut buffer = Vec::new();
        thumbnail_buffer
            .write_to(
                &mut std::io::Cursor::new(&mut buffer),
                image::ImageOutputFormat::Png,
            )
            .expect("Failed to write thumbnail to buffer");

        buffer
    };

    Ok(thumbnail_bin)
}
