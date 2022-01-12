use image::{DynamicImage, GenericImageView};
use itertools::Itertools;

use crate::{ExtractedBlock, utils::calculate_block_prediction_error};

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

        let predicted_block = prediction_offsets.map(|(x_offset, y_offset)| {
            let x: i32 = block.x_offset as i32 + x_offset;
            let y: i32 = block.y_offset as i32 + y_offset;

            if x < 0 || x >= frame.width() as i32 || y < 0 || y >= frame.height() as i32 {
                None
            } else {
                let x = x as u32;
                let y = y as u32;
                Some(ExtractedBlock {
                    x_offset: x,
                    y_offset: y,
                    pixels: frame.crop_imm(x, y, mb_size, mb_size)
                })
            }
        })
        .filter(|crop_result| crop_result.is_some())
        .map(|crop_result| crop_result.unwrap())
        .map(|extracted_block| {
            let error = calculate_block_prediction_error(anchor_block, &extracted_block.pixels);
            (extracted_block, error)
        })
        .min_by(|(_, first_error), (_, second_error)| 
            first_error.partial_cmp(second_error).expect("Comparing NaN errors"))
        .map(|(predicted_block, _)| predicted_block)
        .unwrap();

        println!("Prediction: ({}, {})", predicted_block.x_offset, predicted_block.y_offset);

        predicted_block 
    }
}
