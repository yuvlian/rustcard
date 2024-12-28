use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;

pub fn apply_mask(img: &mut RgbaImage, mask: &RgbaImage) {
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let alpha = mask.get_pixel(x, y)[3];
        pixel.0[3] = alpha;
    }
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
