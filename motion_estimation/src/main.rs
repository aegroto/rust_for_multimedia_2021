use std::fs;

use image::{DynamicImage, GrayImage, ImageError};

struct ExtractedBlock {
    pub x_offset: u32,
    pub y_offset: u32,
    pub pixels: GrayImage,
}

fn main() {
    let mb_size = 16;

    let images: Vec<(String, DynamicImage)> = fs::read_dir("assets/meatthezoo_frames")
        .unwrap()
        .map(|file| {
            let path = file.unwrap().path();
            println!("Loading frame {:?}...", path);

            let file_name = path
                .clone()
                .file_name()
                .unwrap()
                .to_os_string()
                .clone()
                .to_str()
                .unwrap()
                .to_owned();

            let img = image::io::Reader::open(path.clone())
                .unwrap()
                .decode()
                .unwrap();

            (file_name, img)
        })
        .collect();

    extract_blocks("topleft", &images, 0, 0, mb_size);
    extract_blocks("central", &images, 190, 80, mb_size);
}

fn extract_blocks(
    id: &str,
    images: &Vec<(String, DynamicImage)>,
    x_offset: u32,
    y_offset: u32,
    mb_size: u32,
) -> Vec<ExtractedBlock> {
    println!("Extracting '{}' blocks with offset ({}, {})...", id, x_offset, y_offset);

    images.iter().map(|(file_name, img)| {
        let pixels = crop_block_from_image(&img, x_offset, y_offset, mb_size).unwrap();
        export_block(id, &pixels, &file_name).unwrap();

        ExtractedBlock {
            x_offset,
            y_offset,
            pixels,
        }
    }).collect()
}

fn crop_block_from_image(
    img: &image::DynamicImage,
    x_offset: u32,
    y_offset: u32,
    mb_size: u32,
) -> Result<GrayImage, ImageError> {
    let block = img.crop_imm(x_offset, y_offset, mb_size, mb_size);
    let block = block.to_luma8();
    Ok(block)
}

fn export_block(
    block_id: &str,
    block_img: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    file_name: &str,
) -> Result<(), ImageError> {
    let output_folder = format!("output/{}", block_id);
    fs::create_dir_all(&output_folder)?;
    block_img.save(format!("{}/{}", output_folder, file_name))?;
    Ok(())
}
