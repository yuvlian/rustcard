use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};

pub fn resize_to_fit(
    image: &DynamicImage,
    max_width: u32,
    max_height: u32,
    filter: FilterType,
) -> DynamicImage {
    let (width, height) = image.dimensions();
    let aspect_ratio = width as f32 / height as f32;

    let new_dimensions = if width > height {
        let new_width = max_width;
        let new_height = (max_width as f32 / aspect_ratio).round() as u32;
        if new_height > max_height {
            let new_height = max_height;
            let new_width = (max_height as f32 * aspect_ratio).round() as u32;
            (new_width, new_height)
        } else {
            (new_width, new_height)
        }
    } else {
        let new_height = max_height;
        let new_width = (max_height as f32 * aspect_ratio).round() as u32;
        if new_width > max_width {
            let new_width = max_width;
            let new_height = (max_width as f32 / aspect_ratio).round() as u32;
            (new_width, new_height)
        } else {
            (new_width, new_height)
        }
    };

    image.resize(new_dimensions.0, new_dimensions.1, filter)
}

pub fn resize_to_fill_and_stick_image_top_to_top(
    image: &DynamicImage,
    width: u32,
    height: u32,
    filter: FilterType,
) -> DynamicImage {
    let aspect_width = width as f32 / image.width() as f32;
    let aspect_height = height as f32 / image.height() as f32;

    let scale_factor = aspect_width.max(aspect_height);
    let scaled_width = (image.width() as f32 * scale_factor).ceil() as u32;
    let scaled_height = (image.height() as f32 * scale_factor).ceil() as u32;

    let resized = image.resize(scaled_width, scaled_height, filter);

    let crop_x = ((scaled_width - width) / 2).min(scaled_width - width);
    let crop_y = 0;
    let crop_width = width.min(scaled_width);
    let crop_height = height.min(scaled_height);

    resized.crop_imm(crop_x, crop_y, crop_width, crop_height)
}
