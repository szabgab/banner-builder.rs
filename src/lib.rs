use std::path::PathBuf;

use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Banner {
    width: u32,
    height: u32,
    text: String,
}

pub fn draw_image(banner: &Banner, path: &PathBuf) -> bool {
    let limit = 40;
    if banner.text.len() > limit {
        return false;
    }

    // create image
    let mut image = RgbImage::new(banner.width, banner.height);
    // set white background
    for x in 0..banner.width {
        for y in 0..banner.height {
            *image.get_pixel_mut(x, y) = image::Rgb([255, 255, 255]);
        }
    }

    //let font = Vec::from(include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf") as &[u8]);
    let font = Vec::from(include_bytes!(
        "/snap/cups/980/usr/share/fonts/truetype/freefont/FreeSans.ttf"
    ) as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let intended_text_height = 24.4;
    let scale = Scale {
        x: intended_text_height * 2.0,
        y: intended_text_height,
    };

    // color of the text
    let red = 50_u8;
    let green = 50;
    let blue = 0;

    // get the size of the text and calculate the x, y coordinate where to start to be center aligned
    // both horizontally and vertically
    let (text_width, text_height) = text_size(scale, &font, &banner.text);
    //println!("Text size: {}x{}", text_width, text_height);
    let text_start_x = ((banner.width - text_width as u32) / 2) as i32;
    let text_start_y = ((banner.height - text_height as u32) / 2) as i32;

    draw_text_mut(
        &mut image,
        Rgb([red, green, blue]),
        text_start_x,
        text_start_y,
        scale,
        &font,
        &banner.text,
    );

    image.save(path).unwrap();

    true
}
