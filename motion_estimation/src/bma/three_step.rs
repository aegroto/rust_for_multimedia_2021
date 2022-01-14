use image::{DynamicImage, GenericImageView};
use log::debug;

use crate::{utils::calculate_block_prediction_error, ExtractedBlock};

use super::BlockMatcher;

pub struct ThreeStepBlockMatcher {
    search_region_size: i32,
}

impl ThreeStepBlockMatcher {
    pub fn new(search_region_size: u16) -> Self {
        Self {
            search_region_size: search_region_size as i32,
        }
    }
}

impl BlockMatcher for ThreeStepBlockMatcher {
    fn match_block(&self, block: &ExtractedBlock, frame: &DynamicImage) -> ExtractedBlock {
        let mb_size = block.pixels.width();

        let mut r = self.search_region_size as i32;
        let mut anchor_pos = (block.x_offset as i32, block.y_offset as i32);

        let predicted_block = loop {
            let prediction_offsets = vec![
                (-r, -r),
                (0, -r),
                (r, -r),
                (-r, 0),
                (0, 0),
                (r, 0),
                (-r, r),
                (0, r),
                (r, r),
            ];

            let new_anchor =
                get_best_prediction_in_offsets(&block, &frame, anchor_pos, prediction_offsets, mb_size);

            r /= 2;

            if r <= 1 {
                break new_anchor;
            } else {
                anchor_pos = (new_anchor.x_offset as i32, new_anchor.y_offset as i32)
            }
        };

        debug!(
            "Prediction: ({}, {})",
            predicted_block.x_offset, predicted_block.y_offset
        );

        predicted_block
    }
}

fn get_best_prediction_in_offsets(
    block: &ExtractedBlock,
    frame: &DynamicImage,
    anchor_pos: (i32, i32),
    prediction_offsets: Vec<(i32, i32)>,
    mb_size: u32,
) -> ExtractedBlock {
    let anchor_block = &block.pixels;

    let predicted_block = prediction_offsets
        .iter()
        .map(|(x_offset, y_offset)| {
            debug!(
                "Extracting block from offset ({}, {})...",
                x_offset, y_offset
            );
            extract_block_from_offset(anchor_pos, (*x_offset, *y_offset), frame, mb_size)
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(|extracted_block| {
            let error = calculate_block_prediction_error(anchor_block, &extracted_block.pixels);
            (extracted_block, error)
        })
        .min_by(|(_, first_error), (_, second_error)| {
            first_error
                .partial_cmp(second_error)
                .expect("Comparing NaN errors")
        })
        .map(|(predicted_block, _)| predicted_block)
        .unwrap();

    predicted_block
}

fn extract_block_from_offset(
    anchor_pos: (i32, i32),
    block_offset: (i32, i32),
    frame: &DynamicImage,
    mb_size: u32,
) -> Option<ExtractedBlock> {
    let (x_offset, y_offset) = (anchor_pos.0 + block_offset.0, anchor_pos.1 + block_offset.1);

    if is_pos_in_frame(x_offset, y_offset, frame) {
        None
    } else {
        let x_offset = x_offset as u32;
        let y_offset = y_offset as u32;

        Some(ExtractedBlock {
            x_offset,
            y_offset,
            pixels: frame.crop_imm(x_offset, y_offset, mb_size, mb_size),
        })
    }
}

fn is_pos_in_frame(x: i32, y: i32, frame: &DynamicImage) -> bool {
    x < 0 || x >= frame.width() as i32 || y < 0 || y >= frame.height() as i32
}
