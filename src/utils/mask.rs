use image::RgbaImage;

pub fn apply_mask(img: &mut RgbaImage, mask: &RgbaImage) {
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let alpha = mask.get_pixel(x, y)[3];
        pixel.0[3] = alpha;
    }
}
