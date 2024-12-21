use image::GenericImageView;
use image::{DynamicImage, GrayImage, Rgb, RgbImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use mihomo4::Client;
use std::error::Error;
use std::io::Cursor;

const ASSET_BASE_URL: &str = "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/";

pub fn get_asset_url(path: &str) -> String {
    println!("{}", path);
    format!("{}{}", ASSET_BASE_URL, path)
}

pub async fn get_img_from_url(url: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    println!("{}", url);
    let response = Client::new().get(url).send().await?;
    let bytes = response.bytes().await?;

    let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)?;

    Ok(img)
}

pub fn create_dark_block(width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_pixel(width, height, Rgba([26, 26, 26, 255]))
}

pub fn create_light_block(width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_pixel(width, height, Rgba([41, 41, 41, 255]))
}

pub fn create_rounded_mask(size: (u32, u32), radius: u32) -> RgbaImage {
    let (width, height) = size;
    let mut mask = RgbaImage::from_pixel(width, height, Rgba([0, 0, 0, 0]));

    let rect_width = width - 2 * radius;
    let rect_height = height - 2 * radius;

    draw_filled_rect_mut(
        &mut mask,
        Rect::at(radius as i32, radius as i32).of_size(rect_width, rect_height),
        Rgba([0, 0, 0, 255]),
    );

    draw_filled_rect_mut(
        &mut mask,
        Rect::at(radius as i32, 0).of_size(rect_width, radius),
        Rgba([0, 0, 0, 255]),
    );

    draw_filled_rect_mut(
        &mut mask,
        Rect::at(radius as i32, (radius + rect_height) as i32).of_size(rect_width, radius),
        Rgba([0, 0, 0, 255]),
    );

    draw_filled_rect_mut(
        &mut mask,
        Rect::at(0, radius as i32).of_size(radius, rect_height),
        Rgba([0, 0, 0, 255]),
    );

    draw_filled_rect_mut(
        &mut mask,
        Rect::at((radius + rect_width) as i32, radius as i32).of_size(radius, rect_height),
        Rgba([0, 0, 0, 255]),
    );

    draw_filled_circle_mut(
        &mut mask,
        (radius as i32, radius as i32),
        radius as i32,
        Rgba([0, 0, 0, 255]),
    );

    draw_filled_circle_mut(
        &mut mask,
        ((radius + rect_width) as i32, radius as i32),
        radius as i32,
        Rgba([0, 0, 0, 255]),
    );

    draw_filled_circle_mut(
        &mut mask,
        (radius as i32, (radius + rect_height) as i32),
        radius as i32,
        Rgba([0, 0, 0, 255]),
    );

    draw_filled_circle_mut(
        &mut mask,
        ((radius + rect_width) as i32, (radius + rect_height) as i32),
        radius as i32,
        Rgba([0, 0, 0, 255]),
    );

    mask
}

pub fn linear_gradient_img(
    size: (u32, u32),
    color1: (u8, u8, u8),
    color2: (u8, u8, u8),
) -> RgbImage {
    let (width, height) = size;
    let mut img = RgbImage::new(width, height);

    for x in 0..width {
        let t = x as f32 / (width - 1) as f32;
        let r = (1.0 - t) * color1.0 as f32 + t * color2.0 as f32;
        let g = (1.0 - t) * color1.1 as f32 + t * color2.1 as f32;
        let b = (1.0 - t) * color1.2 as f32 + t * color2.2 as f32;

        for y in 0..height {
            img.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
        }
    }

    img
}

pub fn linear_gradient_l_img(size: (u32, u32), color1: u8, color2: u8) -> RgbaImage {
    let (width, height) = size;
    let mut img = RgbaImage::new(width, height);

    for x in 0..width {
        let t = x as f32 / (width - 1) as f32;
        let intensity = (1.0 - t) * color1 as f32 + t * color2 as f32;

        for y in 0..height {
            img.put_pixel(
                x,
                y,
                Rgba([intensity as u8, intensity as u8, intensity as u8, 255]),
            );
        }
    }

    img
}

pub fn create_relic_background(size: (u32, u32), rarity: u8) -> RgbaImage {
    let color = match rarity {
        3 => (92, 89, 255),
        4 => (189, 123, 255),
        5 => (255, 222, 89),
        _ => (92, 255, 89),
    };

    let gradient_mask = linear_gradient_l_img(size, 255, 51); // 255 - 204 = 51
    let mut img = RgbaImage::from_pixel(size.0, size.1, Rgba([color.0, color.1, color.2, 255]));

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let alpha = gradient_mask.get_pixel(x, y)[0];
        pixel[3] = alpha;
    }

    img
}

pub fn resize_to_fit(
    image: &DynamicImage,
    max_width: u32,
    max_height: u32,
    filter: image::imageops::FilterType,
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

pub fn resize_to_fill_top(
    image: &DynamicImage,
    width: u32,
    height: u32,
    filter: image::imageops::FilterType,
) -> DynamicImage {
    let resized = image.resize_to_fill(width, height, filter);
    let (img_width, img_height) = resized.dimensions();

    let crop_y = 0;
    let crop_height = height.min(img_height);

    resized.crop_imm(0, crop_y, width, crop_height)
}
