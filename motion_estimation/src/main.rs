mod bma;

use std::fs;

use image::{DynamicImage, GrayImage, ImageError};
use itertools::Itertools;

use crate::bma::{naive::NaiveBlockMatcher, BlockMatcher};

pub struct ExtractedBlock {
    pub x_offset: u32,
    pub y_offset: u32,
    pub pixels: DynamicImage,
}

struct PredictionResult<'a> {
    anchor_frame_index: usize,
    target_frame_index: usize,
    anchor_block: &'a ExtractedBlock,
    target_block: ExtractedBlock,
}

fn main() {
    let mb_size = 16;

    let mut frames: Vec<(String, DynamicImage)> = fs::read_dir("assets/meatthezoo_frames")
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
                .unwrap()
                .to_luma8();

            let img = DynamicImage::ImageLuma8(img);

            (file_name, img)
        })
        .collect();

    frames.sort_unstable_by_key(|(file_name, _)| {
        let (id, _) = file_name.split_once(".").unwrap();
        id.parse::<i32>().unwrap()
    });

    let topleft_blocks = extract_blocks("topleft/original", &frames, 0, 0, mb_size);
    // let central_blocks = extract_blocks("central", &images, 190, 80, mb_size);

    predict_with_matcher(
        &topleft_blocks,
        &frames,
        "topleft/naive",
        NaiveBlockMatcher::new(),
    );
}

fn predict_with_matcher(
    anchor_blocks: &Vec<ExtractedBlock>,
    frames: &Vec<(String, DynamicImage)>,
    parent_folder: &str,
    matcher: NaiveBlockMatcher,
) {
    let anchor_frame_indices = 0..frames.len() - 1;
    let target_frame_indices = 1..frames.len();

    let prediction_indices_pairs = anchor_frame_indices.cartesian_product(target_frame_indices);

    let prediction_results: Vec<PredictionResult> = prediction_indices_pairs
        .map(|(anchor_frame_index, target_frame_index)| {
            let anchor_block = &anchor_blocks[anchor_frame_index];
            let anchor_frame_id = anchor_frame_index + 1;
            let (target_frame_id, target_frame) = &frames[target_frame_index];

            println!(
                "Matching block from anchor {} to target frame {}",
                anchor_frame_id, target_frame_id
            );

            let target_block = matcher.match_block(anchor_block, target_frame);

            export_block(
                parent_folder,
                target_block.pixels.as_luma8().unwrap(),
                &format!("{}_{}", anchor_frame_id, target_frame_id),
            )
            .unwrap();

            PredictionResult {
                anchor_frame_index,
                target_frame_index,
                anchor_block,
                target_block,
            }
        })
        .collect();

    prediction_results.iter().for_each(|prediction| {
        let anchor_pixels = &prediction.anchor_block.pixels.to_luma8().into_raw();
        let target_pixels = &prediction.target_block.pixels.to_luma8().into_raw();

        let error_weight: f64 = 1.0 / anchor_pixels.len() as f64;

        let error: f64 = anchor_pixels
            .into_iter()
            .zip(target_pixels.into_iter())
            .map(|(anchor_pixel, target_pixel)| (*anchor_pixel as i16, *target_pixel as i16))
            .map(|(anchor_pixel, target_pixel)| {
                let pixel_error = (anchor_pixel - target_pixel).pow(2) as f64;
                pixel_error * error_weight
            })
            .sum();

        println!(
            "Error between anchor {} and target {}: {}",
            prediction.anchor_frame_index, prediction.target_frame_index, error
        );
    });
}

fn extract_blocks(
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
    parent_folder: &str,
    block_img: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    file_name: &str,
) -> Result<(), ImageError> {
    let output_folder = format!("output/{}", parent_folder);
    fs::create_dir_all(&output_folder)?;
    block_img.save(format!("{}/{}", output_folder, file_name))?;
    Ok(())
}
