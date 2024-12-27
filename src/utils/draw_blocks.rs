use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;

pub fn draw_dark_block(width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_pixel(width, height, Rgba([26, 26, 26, 255]))
}

pub fn draw_light_block(width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_pixel(width, height, Rgba([41, 41, 41, 255]))
}

pub fn draw_rounded_mask(size: (u32, u32), radius: u32) -> RgbaImage {
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

pub fn draw_linear_gradient_img(
    size: (u32, u32),
    color1: (u8, u8, u8),
    color2: (u8, u8, u8),
) -> RgbaImage {
    let (width, height) = size;
    let mut img = RgbaImage::new(width, height);

    for x in 0..width {
        let t = x as f32 / (width - 1) as f32;
        let r = (1.0 - t) * color1.0 as f32 + t * color2.0 as f32;
        let g = (1.0 - t) * color1.1 as f32 + t * color2.1 as f32;
        let b = (1.0 - t) * color1.2 as f32 + t * color2.2 as f32;

        for y in 0..height {
            img.put_pixel(x, y, Rgba([r as u8, g as u8, b as u8, 255]));
        }
    }

    img
}

pub fn draw_linear_gradient_l_img(size: (u32, u32), color1: u8, color2: u8) -> RgbaImage {
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

pub fn draw_relic_background(size: (u32, u32), rarity: u8) -> RgbaImage {
    let color = match rarity {
        3 => (92, 89, 255),
        4 => (189, 123, 255),
        5 => (255, 222, 89),
        _ => (92, 255, 89),
    };

    let gradient_mask = draw_linear_gradient_l_img(size, 255, 51); // 255 - 204 = 51
    let mut img = RgbaImage::from_pixel(size.0, size.1, Rgba([color.0, color.1, color.2, 255]));

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let alpha = gradient_mask.get_pixel(x, y)[0];
        pixel[3] = alpha;
    }

    img
}
