extern crate three;
extern crate rand;
extern crate palette;

const SCENERY_S: f32 = 0.25;
const SCENERY_L: f32 = 0.1;
const SCENERY_H: f32 = 100.;

use color::palette::{Pixel, FromColor};

pub fn get_building_color<R: rand::Rng>(rng: &mut R) -> three::color::Color {
    let color = palette::Hsl::new(
        SCENERY_H + rng.gen_range(-25., 25.),
        SCENERY_S,
        SCENERY_L);

    let pixel: [u8; 3] = palette::Srgb::from_hsl(color)
        .into_format()
        .into_raw();
    let r = pixel[0] as u32;
    let g = pixel[1] as u32;
    let b = pixel[2] as u32;
    (r<<4) + (g<<2) + b
}
