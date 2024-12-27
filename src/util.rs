use image::imageops::overlay;
use image::{DynamicImage, GenericImageView, GrayImage, LumaA, Rgb, RgbImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use mihomo4::Client;
use rusttype::{Font, PositionedGlyph, Scale, point};
use std::error::Error;
use std::io::Cursor;
use imageproc::drawing::draw_text_mut;

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

pub fn resize_to_fill_and_stick_image_top_to_top(
    image: &DynamicImage,
    width: u32,
    height: u32,
    filter: image::imageops::FilterType,
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

pub fn create_png_outline(
    img: &RgbaImage,
    outline_color: Rgba<u8>,
    outline_width: u32,
) -> RgbaImage {
    let width = img.width();
    let height = img.height();

    let mut outlined_img = RgbaImage::new(width + outline_width * 2, height + outline_width * 2);

    for y in 0..outlined_img.height() {
        for x in 0..outlined_img.width() {
            outlined_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if pixel[3] != 0 {
                for offset_y in -(outline_width as i32)..=(outline_width as i32) {
                    for offset_x in -(outline_width as i32)..=(outline_width as i32) {
                        let new_x = (x as i32 + offset_x + outline_width as i32) as u32;
                        let new_y = (y as i32 + offset_y + outline_width as i32) as u32;

                        if new_x < outlined_img.width() && new_y < outlined_img.height() {
                            outlined_img.put_pixel(new_x, new_y, outline_color);
                        }
                    }
                }
            }
        }
    }

    outlined_img
}

pub fn draw_text_with_outline(
    img: &mut RgbaImage,
    text: &str,
    position: (i32, i32),
    font: &impl ab_glyph::Font,
    scale: ab_glyph::PxScale,
    text_color: Rgba<u8>,
    outline_color: Rgba<u8>,
    outline_thickness: i32,
) {
    let (x, y) = position;

    for dx in -outline_thickness..=outline_thickness {
        for dy in -outline_thickness..=outline_thickness {
            if dx * dx + dy * dy <= outline_thickness * outline_thickness {
                draw_text_mut(
                    img,
                    outline_color,
                    x + dx,
                    y + dy,
                    scale,
                    font,
                    text,
                );
            }
        }
    }

    draw_text_mut(
        img,
        text_color,
        x,
        y,
        scale,
        font,
        text,
    );
}

pub fn draw_text(
    img: &mut RgbaImage,
    text: &str,
    position: (i32, i32),
    font: &rusttype::Font,
    scale: f32,
    color: Rgba<u8>,
    stroke_color: Rgba<u8>,
    stroke_width: f32,
) {
    let scale = Scale { x: scale, y: scale };
    let offset = point(position.0 as f32, position.1 as f32);

    let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, offset).collect();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            if stroke_width > 0.0 {
                glyph.draw(|x, y, v| {
                    let x2 = (x as i32 + bounding_box.min.x) as u32;
                    let y2 = (y as i32 + bounding_box.min.y) as u32;
                    if v > 0.0 {
                        if x2 < img.width() && y2 < img.height() {
                            let pixel = img.get_pixel_mut(x2, y2);
                            let blended = blend_text_pixel(*pixel, stroke_color);
                            *pixel = blended;
                        }
                    }
                });
            }

            glyph.draw(|x, y, v| {
                let x2 = (x as i32 + bounding_box.min.x) as u32;
                let y2 = (y as i32 + bounding_box.min.y) as u32;
                if v > 0.0 {
                    if x2 < img.width() && y2 < img.height() {
                        let pixel = img.get_pixel_mut(x2, y2);
                        let blended = blend_text_pixel(*pixel, color);
                        *pixel = blended;
                    }
                }
            });
        }
    }
}

fn blend_text_pixel(bottom: Rgba<u8>, top: Rgba<u8>) -> Rgba<u8> {
    let alpha = top[3] as f32 / 255.0;
    let inv_alpha = 1.0 - alpha;

    Rgba([
        (top[0] as f32 * alpha + bottom[0] as f32 * inv_alpha) as u8,
        (top[1] as f32 * alpha + bottom[1] as f32 * inv_alpha) as u8,
        (top[2] as f32 * alpha + bottom[2] as f32 * inv_alpha) as u8,
        255,
    ])
}
