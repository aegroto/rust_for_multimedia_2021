use image::{DynamicImage, GenericImageView};
use itertools::Itertools;

use crate::{utils::calculate_block_prediction_error, ExtractedBlock};

use super::BlockMatcher;

pub struct ExhaustiveBlockMatcher {
    search_region_size: i32,
}

impl ExhaustiveBlockMatcher {
    pub fn new(search_region_size: u16) -> Self {
        Self {
            search_region_size: search_region_size as i32,
        }
    }
}

impl BlockMatcher for ExhaustiveBlockMatcher {
    fn match_block(&self, block: &ExtractedBlock, frame: &DynamicImage) -> ExtractedBlock {
        let mb_size = block.pixels.width();
        let anchor_block = &block.pixels;

        let prediction_offsets = (-self.search_region_size..self.search_region_size)
            .cartesian_product(-self.search_region_size..self.search_region_size);

        let predicted_block = prediction_offsets
            .map(|(x_offset, y_offset)| {
                extract_block_from_offset(block, x_offset, y_offset, frame, mb_size)
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

        println!(
            "Prediction: ({}, {})",
            predicted_block.x_offset, predicted_block.y_offset
        );

        predicted_block
    }
}

fn extract_block_from_offset(
    block: &ExtractedBlock,
    block_x_offset: i32,
    block_y_offset: i32,
    frame: &DynamicImage,
    mb_size: u32,
) -> Option<ExtractedBlock> {
    let x_offset = block.x_offset as i32 + block_x_offset;
    let y_offset = block.y_offset as i32 + block_y_offset;
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
    x < 0
        || x >= frame.width() as i32
        || y < 0
        || y >= frame.height() as i32
}
