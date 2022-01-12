use image::{DynamicImage};

use crate::ExtractedBlock;

pub mod naive;
pub mod exhaustive;
pub mod three_step;

pub trait BlockMatcher {
    fn match_block(&self, block: &ExtractedBlock, frame: &DynamicImage) -> ExtractedBlock;
}

