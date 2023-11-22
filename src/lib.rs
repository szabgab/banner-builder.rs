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

    #[serde(default = "default_white")]
    background_color: String,
}

fn default_white() -> String {
    "FFFFFF".to_string()
}

pub fn draw_image(banner: &Banner, path: &PathBuf) -> bool {
    let limit = 90;
    if banner.text.len() > limit {
        return false;
    }

    // create image
    let mut image = RgbImage::new(banner.width, banner.height);
    // set background color
    let red = u8::from_str_radix(&banner.background_color[0..=1], 16).unwrap();
    let green = u8::from_str_radix(&banner.background_color[2..=3], 16).unwrap();
    let blue = u8::from_str_radix(&banner.background_color[4..=5], 16).unwrap();

    for x in 0..banner.width {
        for y in 0..banner.height {
            *image.get_pixel_mut(x, y) = image::Rgb([red, green, blue]);
        }
    }

    //"/snap/cups/980/usr/share/fonts/truetype/freefont/FreeSans.ttf"
    let font =
        Vec::from(include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf") as &[u8]);
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
    let width = 30;
    let lines = textwrap::wrap(&banner.text, width);
    let padding: u32 = 10;
    let (_text_width, text_height) = text_size(scale, &font, &banner.text);
    let line_height = padding + text_height as u32;
    let start_row = (banner.height / 2) - line_height * (lines.len() as u32) / 2;
    //println!("start_row: {}", start_row);

    for (idx, line) in lines.iter().enumerate() {
        let (text_width, _text_height) = text_size(scale, &font, line);
        //println!("Text size: {}x{}", text_width, text_height);
        //println!("banner width: {}  text width: {}", banner.width, text_width);
        let text_start_x = (banner.width - text_width as u32) / 2;
        let text_start_y = start_row + (idx as u32) * line_height;

        draw_text_mut(
            &mut image,
            Rgb([red, green, blue]),
            text_start_x as i32,
            text_start_y as i32,
            scale,
            &font,
            line,
        );
    }

    image.save(path).unwrap();

    true
}
