use ab_glyph::{Font, PxScale};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

pub fn draw_png_with_outline(
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
    font: &impl Font,
    img: &mut RgbaImage,
    text: &str,
    position: (i32, i32),
    scale: PxScale,
    text_color: Rgba<u8>,
    outline_color: Rgba<u8>,
    outline_thickness: i32,
) {
    let (x, y) = position;

    for dx in -outline_thickness..=outline_thickness {
        for dy in -outline_thickness..=outline_thickness {
            if dx * dx + dy * dy <= outline_thickness * outline_thickness {
                draw_text_mut(img, outline_color, x + dx, y + dy, scale, &font, text);
            }
        }
    }

    draw_text_mut(img, text_color, x, y, scale, &font, text);
}
