use image::{self, imageops::FilterType, io::Reader};
use log::debug;
use std::fs::File;
use std::io::BufReader;

pub fn generate_thumbnail(path: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    // Create Image Reader and turn off size limits
    let mut image_reader = Reader::new(reader).with_guessed_format()?;

    image_reader.no_limits();

    // Decode the image
    let image = image_reader.decode()?;

    // Get the aspect ratio of the image
    let aspect_ratio = image.width() as f32 / image.height() as f32;

    // Calculate the thumbnail size
    let thumbnail_size = if aspect_ratio > 1.0 {
        (512, (512.0 / aspect_ratio) as u32)
    } else {
        (((512.0 * aspect_ratio) as u32), 512)
    };

    // Create the thumbnail
    let thumbnail = image.resize_exact(thumbnail_size.0, thumbnail_size.1, FilterType::Lanczos3);

    // Convert the thumbnail to a binary Vec<u8>
    let thumbnail_bin = thumbnail.to_rgb8().into_vec();

    Ok(thumbnail_bin)
}

pub fn generate_display(path: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path).expect("Failed to open file");

    let reader = std::io::BufReader::new(file);

    let mut image_reader = image::io::Reader::new(reader)
        .with_guessed_format()
        .expect("Failed to read micrograph");

    image_reader.no_limits();

    let image = image_reader.decode().unwrap();

    // get aspect ratio of micrograph
    let aspect_ratio = image.width() as f32 / image.height() as f32;

    // calculate display image size
    let desired_size = 2048;
    let display_size = if aspect_ratio > 1.0 {
        (desired_size, (desired_size as f32 / aspect_ratio) as u32)
    } else {
        (((desired_size as f32 * aspect_ratio) as u32), desired_size)
    };

    // create display image
    let display_image = image
        .resize(
            display_size.0,
            display_size.1,
            image::imageops::FilterType::Lanczos3,
        )
        .to_rgb16();

    // convert display image to binary vec<u8>
    let display_bin = convert_image_to_binary(display_image)?;

    Ok(display_bin)
}

fn convert_image_to_binary(
    image: image::ImageBuffer<image::Rgb<u16>, Vec<u16>>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut buffer),
        image::ImageOutputFormat::Png,
    )?;

    Ok(buffer)
}
