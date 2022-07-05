use std::{fs, path::PathBuf, str::FromStr};

const RECT_WIDTH: usize = 1410;
const RECT_HEIGHT: usize = 1050;
const HORIZONTAL_RECT_COUNT: usize = 2;
const VERTICAL_RECT_COUNT: usize = 2;
const SOURCE_PATH: &'static str = "change_me";

fn main() {
    let paths = fs::read_dir(SOURCE_PATH).unwrap();
    let mut counter = 0;

    for image_path in paths {
        let image = image::open(&image_path.unwrap().path()).expect("Image not found");

        for x in (0..HORIZONTAL_RECT_COUNT)
            .into_iter()
            .map(|i| i * RECT_WIDTH)
        {
            for y in (0..VERTICAL_RECT_COUNT)
                .into_iter()
                .map(|i| i * RECT_HEIGHT)
            {
                let rect =
                    image.crop_imm(x as u32, y as u32, RECT_WIDTH as u32, RECT_HEIGHT as u32);
                let rect_name = format!("{}.png", counter);
                let output_path: PathBuf = PathBuf::from_str(SOURCE_PATH)
                    .unwrap()
                    .join("output")
                    .join(rect_name);
                rect.save_with_format(&output_path, image::ImageFormat::Png)
                    .unwrap();
                counter += 1;
            }
        }
    }
}
