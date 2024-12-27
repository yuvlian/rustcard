use image::Rgba;

pub fn blend_image_pixel(bg: Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
    let alpha_fg = fg[3] as f32 / 255.0;
    let alpha_bg = bg[3] as f32 / 255.0;

    let out_alpha = alpha_fg + alpha_bg * (1.0 - alpha_fg);
    
    let out_r = if out_alpha > 0.0 {
        ((fg[0] as f32 * alpha_fg) + (bg[0] as f32 * alpha_bg * (1.0 - alpha_fg))) / out_alpha
    } else {
        0.0
    };

    let out_g = if out_alpha > 0.0 {
        ((fg[1] as f32 * alpha_fg) + (bg[1] as f32 * alpha_bg * (1.0 - alpha_fg))) / out_alpha
    } else {
        0.0
    };

    let out_b = if out_alpha > 0.0 {
        ((fg[2] as f32 * alpha_fg) + (bg[2] as f32 * alpha_bg * (1.0 - alpha_fg))) / out_alpha
    } else {
        0.0
    };

    Rgba([
        out_r.min(255.0) as u8,
        out_g.min(255.0) as u8,
        out_b.min(255.0) as u8,
        (out_alpha * 255.0).min(255.0) as u8,
    ])
}
