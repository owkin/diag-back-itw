use image::DynamicImage;

pub fn blur_image(image: &DynamicImage, sigma: f32) -> DynamicImage {
    image.blur(sigma)
}

pub fn unsharpen_image(image: &DynamicImage, sigma: f32, threshold: i32) -> DynamicImage {
    image.unsharpen(sigma, threshold)
}
