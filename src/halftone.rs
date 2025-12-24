use image::{ImageError, Rgba, RgbaImage, open};
use imageproc::drawing::draw_filled_circle_mut;
use std::env;
use std::fs;
use std::path::Path;

pub fn halftone(vec: Vec<String>, step: i32, detailize: i32, color_bg: &str, color: &str) -> Result<(), ImageError> {
    let bg = match color_bg {
        "sky" => [127, 199, 255, 255],
        "purple" => [139, 0, 255, 255],
        "red" => [152, 0, 2, 255],
        "gold" => [255, 215, 0, 255],
        "pink" => [255, 192, 203, 255],
        "yellow" => [255, 255, 122, 255],
        "dark-blue" => [5, 14, 60, 255],
        "pure-red" => [220, 0, 0, 255],
        "dark-green" => [13, 71, 21, 255],
        "orange" => [255, 162, 64, 255],
        "black" => [0, 0, 0, 255],
        _ => [255, 255, 255, 255],
    };
    let color_p = match color {
        "sky" => [127, 199, 255, 255],
        "purple" => [139, 0, 255, 255],
        "red" => [152, 0, 2, 255],
        "gold" => [255, 215, 0, 255],
        "pink" => [255, 192, 203, 255],
        "yellow" => [255, 255, 122, 255],
        "dark-blue" => [5, 14, 60, 255],
        "pure-red" => [220, 0, 0, 255],
        "dark-green" => [13, 71, 21, 255],
        "orange" => [255, 162, 64, 255],
        "white" => [255, 255, 255, 255],
        _ => [0, 0, 0, 255],
    };
    let output_dir = "./output";

    fs::create_dir_all(output_dir)?;
    let mut m = 0;
    for i in vec {
        let img = open(&i)?; //открываем изображение

        let gray_img = img.to_luma8(); //конвертируем в чернобелые тона

        let (width, height) = gray_img.dimensions(); //получаем измерения длина ширина

        let mut result = RgbaImage::from_pixel(width, height, Rgba(bg)); //новое пустое белое изображение

        //чем меньше значения max_radius,
        //КОТОРОЕ в свою очередь зависит от step, чем больше точек -> детализированнее картинка
        //(radius / 2) as f32;
        let max_radius = match detailize {
            1 => step as f32 * 0.3,
            2 => step as f32 * 0.4,
            3 => step as f32 * 0.45,
            4 => step as f32 * 0.5,
            _ => 100.0,
        };

        if max_radius <= step as f32 * 0.5 {
            for y in (0..height).step_by(step as usize) {
                for x in (0..width).step_by(step as usize) {
                    let pixels = gray_img.get_pixel(x, y).0[0];
                    let brightness = pixels as f32 / 255.0;
                    if brightness < 0.8 {
                        let radius = ((1.0 - brightness) * max_radius) as i32;
                        // println!("{brightness}, {max_radius}, {radius}");
                        if radius > 0 {
                            let center = (x as i32, y as i32); //меняет цвет точки
                            draw_filled_circle_mut(&mut result, center, radius, Rgba(color_p));
                        }
                    }
                }
            }
        } else {
            println!("Please,write correct detailize, in case it 1,2,3,4. NO BIGGER OR SMALLER");
            println!("Also, step can't be bigger than 15-20, because circle have be very big")
        }
        let output_path = format!("{}/{}_halftone.png", output_dir, m);
        result.save(&output_path)?;
        println!("Обработано: {} -> {}", i, output_path);
        m += 1;
    }

    Ok(())
}

pub fn imgs() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let img_dir = "./img";

    let mut imgs = Vec::new();

    for img in fs::read_dir(img_dir)? {
        let img = img?;
        let path = img.path();
        if path.is_file() {
            match path.file_name() {
                Some(name) => {
                    if let Some(nm) = name.to_str() {
                        let f_name = format!("{}/{}", img_dir, nm);
                        imgs.push(f_name);
                    }
                }
                None => imgs.push("None".to_string()),
            }
        }
    }

    Ok(imgs)
}
