use std::fs;

use image::{DynamicImage, GrayImage, ImageError};

use crate::types::ExtractedBlock;

pub fn extract_blocks(
    parent_folder: &str,
    images: &Vec<(String, DynamicImage)>,
    x_offset: u32,
    y_offset: u32,
    mb_size: u32,
) -> Vec<ExtractedBlock> {
    println!(
        "Extracting '{}' blocks with offset ({}, {})...",
        parent_folder, x_offset, y_offset
    );

    images
        .iter()
        .map(|(file_name, img)| {
            let pixels = crop_block_from_image(&img, x_offset, y_offset, mb_size).unwrap();
            export_block(parent_folder, &pixels, &file_name).unwrap();

            ExtractedBlock {
                x_offset,
                y_offset,
                pixels: DynamicImage::ImageLuma8(pixels),
            }
        })
        .collect()
}

pub fn crop_block_from_image(
    img: &image::DynamicImage,
    x_offset: u32,
    y_offset: u32,
    mb_size: u32,
) -> Result<GrayImage, ImageError> {
    let block = img.crop_imm(x_offset, y_offset, mb_size, mb_size);
    let block = block.to_luma8();
    Ok(block)
}

pub fn export_block(
    parent_folder: &str,
    block_img: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    file_name: &str,
) -> Result<(), ImageError> {
    let output_folder = format!("output/{}", parent_folder);
    fs::create_dir_all(&output_folder)?;
    block_img.save(format!("{}/{}", output_folder, file_name))?;
    Ok(())
}

pub fn calculate_block_prediction_error(anchor_block: &DynamicImage, target_block: &DynamicImage) -> f64 {
    let anchor_pixels = anchor_block.to_luma8().into_raw();
    let target_pixels = target_block.to_luma8().into_raw();
    let error_weight: f64 = 1.0 / anchor_pixels.len() as f64;
    let error: f64 = anchor_pixels
        .into_iter()
        .zip(target_pixels.into_iter())
        .map(|(anchor_pixel, target_pixel)| (anchor_pixel as i16, target_pixel as i16))
        .map(|(anchor_pixel, target_pixel)| {
            let pixel_error = (anchor_pixel - target_pixel).abs() as f64;
            pixel_error * error_weight
        })
        .sum();
    error
}
