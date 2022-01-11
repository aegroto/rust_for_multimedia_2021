use std::fs;

use image::ImageError;

fn main() -> Result<(), ImageError> {
    let mb_size = 16;

    for file in fs::read_dir("assets/meatthezoo_frames").unwrap() {
        let path = file.unwrap().path();
        println!("Loading frame {:?}...", path);

        let img = image::io::Reader::open(path.clone())?.decode()?;
        let file_name = path.file_name().unwrap().to_str().unwrap();
        export_block_from_image("topleft", &img, 0, 0, mb_size, file_name)?;
        export_block_from_image("central", &img, 190, 80, mb_size, file_name)?;
    }

    Ok(())
}

fn export_block_from_image(
    block_id: &str,
    img: &image::DynamicImage,
    x_offset: u32,
    y_offset: u32,
    mb_size: u32,
    file_name: &str,
) -> Result<(), ImageError> {
    let topleft_corner = img.crop_imm(x_offset, y_offset, mb_size, mb_size);
    let output_folder = format!("output/{}", block_id);
    fs::create_dir_all(&output_folder)?;
    topleft_corner.save(format!("{}/{}", output_folder, file_name))?;
    Ok(())
}
