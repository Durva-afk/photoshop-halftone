use image::{ImageError, Rgba, RgbaImage, open};
use imageproc::drawing::draw_filled_circle_mut;
use std::fs;
//imports
pub fn halftone(
    vec: Vec<String>,
    step: i32,
    detailize: i32,
    color_bg: &str,
    color: &str,
) -> Result<(), ImageError> {
    let bg = match color_bg {
        //background colors
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
        //colors of texture(circle in this version)
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
    let output_dir = "./output"; //output dir with final result

    fs::create_dir_all(output_dir)?; //if dir not exist -> create
    let mut m = 0; //counter for naming
    for i in vec {
        let img = open(&i)?; //open img

        let gray_img = img.to_luma8(); //convert to black white picture

        let (width, height) = gray_img.dimensions(); //dimensions of img(x,y)

        let mut result = RgbaImage::from_pixel(width, height, Rgba(bg)); //new empty img

        //if step is not big => img more detalize
        //more circles => more detalize
        //step = (radius / 2) as f32; by default halftone
        let max_radius = match detailize {
            1 => step as f32 * 0.3, //lvls of detalize
            2 => step as f32 * 0.4,
            3 => step as f32 * 0.45,
            4 => step as f32 * 0.5,
            _ => 100.0,
        };
        //generate img >
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
                            draw_filled_circle_mut(&mut result, center, radius, Rgba(color_p)); //draw circles
                        }
                    }
                }
            }
        } else {
            println!("Please,write correct detailize, in case it 1,2,3,4. NO BIGGER OR SMALLER");
            println!(
                "Also, step can't be bigger than 15-20 and less than 4-5, because img gonna be bad"
            )
        }
        let output_path = format!("{}/{}_halftone.png", output_dir, m); //generate name for output files
        result.save(&output_path)?; //save png
        println!("Обработано: {} -> {}", i, output_path);
        m += 1; //counter+=1
    }

    Ok(())
}

pub fn imgs() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let img_dir = "./img"; //dir for img, original photos before halftone

    let mut imgs = Vec::new();

    for img in fs::read_dir(img_dir)? {
        //reading this dir
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
