use std::{fs, path::PathBuf, str::FromStr};

const STARTING_OFFSET_HORIZONTAL: usize = 0;
const STARTING_OFFSET_VERTICAL: usize = 0;
const RECT_WIDTH: usize = 1386;
const RECT_HEIGHT: usize = 1050;
const HORIZONTAL_RECT_COUNT: usize = 2;
const VERTICAL_RECT_COUNT: usize = 1;
const SOURCE_PATH: &'static str = "";

fn main() {
    let output_path: PathBuf = PathBuf::from_str(SOURCE_PATH).unwrap().join("output");
    fs::create_dir_all(&output_path).unwrap();
    let paths = fs::read_dir(SOURCE_PATH).unwrap();
    let mut counter = 0;

    for image_path in paths.filter(|entry| entry.as_ref().unwrap().metadata().unwrap().is_file()) {
        let image = image::open(&image_path.unwrap().path()).expect("Image not found");

        for x in (0..HORIZONTAL_RECT_COUNT)
            .into_iter()
            .map(|i| i * RECT_WIDTH + STARTING_OFFSET_HORIZONTAL)
        {
            for y in (0..VERTICAL_RECT_COUNT)
                .into_iter()
                .map(|i| i * RECT_HEIGHT + STARTING_OFFSET_VERTICAL)
            {
                let rect =
                    image.crop_imm(x as u32, y as u32, RECT_WIDTH as u32, RECT_HEIGHT as u32);
                let rect_name = format!("{}.png", counter);
                rect.save_with_format(output_path.join(rect_name), image::ImageFormat::Png)
                    .unwrap();
                counter += 1;
            }
        }
    }
}
