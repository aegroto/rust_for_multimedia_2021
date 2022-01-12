use image::{DynamicImage};

use crate::ExtractedBlock;

pub mod naive;
pub mod exhaustive;

pub trait BlockMatcher {
    fn match_block(&self, block: &ExtractedBlock, frame: &DynamicImage) -> ExtractedBlock;
}

