use crate::app::database::Database;
use crate::app::model::{EncodedImage, JobRequest, Status};
use crate::image_proc::{blur_image, unsharpen_image};
use image::{DynamicImage, ImageEncoder};
use std::io::Cursor;
use image::codecs::png;
use uuid::Uuid;

pub fn process(id: Uuid, request: &JobRequest, database: Database) {
    let id = database.write().unwrap().create_job(id, request.filter);

    let result = image_processing(request);

    match result {
        Ok(result) => {
            database
                .write()
                .unwrap()
                .update_result(id, Status::Done, Some(result));
            println!("Image processed successfully")
        }
        Err(e) => {
            database
                .write()
                .unwrap()
                .update_result(id, Status::Error, None);
            eprintln!("Error processing image: {:?}", e);
        }
    }
}

fn image_processing(request: &JobRequest) -> Result<DynamicImage, anyhow::Error> {
    let image = image::load_from_memory(&request.image)?;

    let result = match request.filter {
        crate::app::model::FilterType::Blurring => blur_image(&image, request.sigma),
        crate::app::model::FilterType::UnSharpening => unsharpen_image(&image, request.sigma, 1),
    };
    Ok(result)
}

pub fn encode_buffer_rgb(image: &DynamicImage) -> Result<EncodedImage, anyhow::Error> {
    let mut data = vec![];

    let cursor = &mut Cursor::new(&mut data);

    let encoder = png::PngEncoder::new_with_quality(
        cursor,
        png::CompressionType::Fast,
        png::FilterType::NoFilter,
    );
    encoder.write_image(
        image.as_bytes(),
        image.width(),
        image.height(),
        image.color(),
    )?;

    Ok(EncodedImage { data })
}
