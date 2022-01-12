use image::DynamicImage;

pub struct ExtractedBlock {
    pub x_offset: u32,
    pub y_offset: u32,
    pub pixels: DynamicImage,
}

pub struct PredictionResult<'a> {
    pub anchor_frame_index: usize,
    pub target_frame_index: usize,
    pub anchor_block: &'a ExtractedBlock,
    pub target_block: ExtractedBlock,
}