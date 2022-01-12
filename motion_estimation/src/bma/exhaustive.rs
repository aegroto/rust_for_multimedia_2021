use image::{DynamicImage, GenericImageView};

use crate::ExtractedBlock;

use super::BlockMatcher;

pub struct ExhaustiveBlockMatcher {
    search_region_size: usize
}

impl ExhaustiveBlockMatcher {
    pub fn new(search_region_size: usize) -> Self {
        Self { 
            search_region_size
        }
    }
}

impl BlockMatcher for ExhaustiveBlockMatcher {
    fn match_block(&self, block: &ExtractedBlock, frame: &DynamicImage) -> ExtractedBlock {
        let mb_size = block.pixels.width();
        let predicted_pixels = frame.crop_imm(block.x_offset, block.y_offset, mb_size, mb_size);

        ExtractedBlock {
            x_offset: 0,
            y_offset: 0,
            pixels: predicted_pixels,
        }
    }
}
