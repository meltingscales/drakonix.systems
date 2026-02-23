use axum::{http::header, response::IntoResponse};
use image::{ImageBuffer, Rgba};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::io::Cursor;

// Drakonix fursona color palette:
//   deep navy body, dark navy scales, bright purple wings, lilac hair,
//   cyan/teal bg, light cyan, medium purple, pale lavender, off-white claws
const PALETTE: &[(u8, u8, u8)] = &[
    (13, 27, 42),    // #0d1b2a deep navy body
    (26, 26, 53),    // #1a1a35 dark navy scales
    (147, 51, 234),  // #9333ea bright purple wings
    (192, 132, 252), // #c084fc light lilac hair
    (6, 182, 212),   // #06b6d4 cyan/teal bg
    (34, 211, 238),  // #22d3ee light cyan
    (168, 85, 247),  // #a855f7 medium purple
    (216, 180, 254), // #d8b4fe pale lavender
    (240, 240, 240), // #f0f0f0 off-white claws
];

pub async fn favicon_ico() -> impl IntoResponse {
    let mut rng = rand::thread_rng();
    let size: u32 = 32;

    // Randomly pick stripe direction: true = horizontal bands, false = vertical bands
    let horizontal = rng.gen_bool(0.5);

    // 3–6 stripes
    let num_stripes: usize = rng.gen_range(3..=6);

    // Pre-pick one color per stripe so each band is uniform before noise
    let stripe_colors: Vec<(u8, u8, u8)> = (0..num_stripes)
        .map(|_| PALETTE[rng.gen_range(0..PALETTE.len())])
        .collect();

    // Perlin noise parameters — randomised each request
    let noise_scale: f64 = rng.gen_range(3.0..9.0);
    let noise_strength: f64 = rng.gen_range(25.0..70.0); // additive shift, out of 255
    let seed: u32 = rng.gen();
    let perlin = Perlin::new(seed);

    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(size, size);

    for y in 0..size {
        for x in 0..size {
            // Which stripe does this pixel fall in?
            let axis = if horizontal { y } else { x };
            let stripe_idx =
                ((axis as f64 / size as f64) * num_stripes as f64) as usize;
            let stripe_idx = stripe_idx.min(num_stripes - 1);
            let (r, g, b) = stripe_colors[stripe_idx];

            // Perlin noise sampled at this pixel — returns roughly [-1, 1]
            let n = perlin.get([x as f64 / noise_scale, y as f64 / noise_scale]);
            let shift = (n * noise_strength) as i32;

            let r = (r as i32 + shift).clamp(0, 255) as u8;
            let g = (g as i32 + shift).clamp(0, 255) as u8;
            let b = (b as i32 + shift).clamp(0, 255) as u8;

            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)
        .expect("PNG encode failed");
    let bytes = buf.into_inner();

    ([(header::CONTENT_TYPE, "image/png")], bytes)
}
