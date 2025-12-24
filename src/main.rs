mod halftone;
use halftone::halftone;
use halftone::imgs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Конвертируем изображение...");

    let res = imgs()?;
    println!("{:#?}", res);
    let output = halftone(res, 5, 4, "sky", "pure-red"); //7-8, 4, black - base
    match output {
        Ok(img) => img,
        Err(e) => panic!("Error with output result: {}", e),
    };
    println!("Готово!");
    Ok(())
}
