mod bma;
mod utils;
mod types;

use std::{fs, ops::Div, time::Instant};

use bma::exhaustive::ExhaustiveBlockMatcher;
use image::{DynamicImage, GrayImage, ImageError};
use itertools::Itertools;
use types::{ExtractedBlock, PredictionResult};
use utils::extract_blocks;

use crate::{bma::{naive::NaiveBlockMatcher, BlockMatcher}, utils::{calculate_block_prediction_error, export_block}};

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
    let central_blocks = extract_blocks("central/original", &frames, 190, 80, mb_size);
    // let central_blocks = extract_blocks("central", &images, 190, 80, mb_size);

    println!(" --- Central block, naive predictor");
    let start_time = Instant::now();
    predict_with_matcher(
        &central_blocks,
        &frames,
        "central/naive",
        Box::new(NaiveBlockMatcher::new())
    );
    println!(" --- Execution time: {}s", start_time.elapsed().as_secs_f64());

    println!(" --- Central block, exhaustive predictor");
    let start_time = Instant::now();
    predict_with_matcher(
        &central_blocks,
        &frames,
        "central/exhaustive",
        Box::new(ExhaustiveBlockMatcher::new(25))
    );
    println!(" --- Execution time: {}s", start_time.elapsed().as_secs_f64());
}

fn predict_with_matcher(
    anchor_blocks: &Vec<ExtractedBlock>,
    frames: &Vec<(String, DynamicImage)>,
    parent_folder: &str,
    matcher: Box<dyn BlockMatcher>,
) {
    let anchor_frame_indices = 0..frames.len() - 1;
    // let target_frame_indices = 1..frames.len();

    let prediction_indices_pairs = anchor_frame_indices.flat_map(|anchor_frame_index| {
        (anchor_frame_index..anchor_frame_index+1).cartesian_product(anchor_frame_index+1..frames.len())
    });

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

    let prediction_errors = prediction_results.iter().map(|prediction| {
        let anchor_block = prediction.anchor_block;
        let target_block = &prediction.target_block;
        let error = calculate_block_prediction_error(&anchor_block.pixels, &target_block.pixels);

        println!(
            "Error between anchor {} and target {}: {}",
            prediction.anchor_frame_index, prediction.target_frame_index, error
        );

        error
    });

    println!("Average error: {}", prediction_errors
        .sum::<f64>()
        .div(prediction_results.len() as f64));
}
