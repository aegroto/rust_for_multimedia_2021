use image::{GrayImage, ImageError};

use convolve2d::*;
use rust_for_multimedia_canny::{
    drog::perform_drog_convolution,
    edge::{Edge, ThresholdedEdge},
    hysteresis::perform_hysteresis_thresholding,
    nonmax::perform_nonmax_suppression,
};

fn main() -> Result<(), ImageError> {
    // Params
    let sigma = 2.0;
    let kernel_size = 10;

    // Load image and convert to Luma8
    let image_reader = image::io::Reader::open("test_assets/myownlena.jpg")?;
    let image = image_reader.decode()?;
    let image_luma = image.into_luma8();
    image_luma.save("test_outputs/myownlena_luma8.jpg").unwrap();

    let image_matrix: DynamicMatrix<SubPixels<u8, 1>> = image_luma.into();

    let normalized_image_matrix = image_matrix.map_subpixels(|x| (x as f64) / 255.0);

    // DroG convolution
    let drog_edges = perform_drog_convolution(&normalized_image_matrix, kernel_size, sigma);
    GrayImage::from(
        drog_edges
            .clone()
            .map(|edge| SubPixels([edge.get_magnitude()]))
            .map_subpixels(|v| f64::round(v * 255.0) as u8),
    )
    .save("test_outputs/myownlena_drog_magnitude.png")
    .unwrap();
    count_nonzero_edges(&drog_edges);

    // Non-maximum suppression
    println!("Applying non-maximum suppression...");
    let (width, height) = (
        normalized_image_matrix.get_width(),
        normalized_image_matrix.get_height(),
    );

    let nonmax_edges = perform_nonmax_suppression(width, height, &drog_edges, 25);
    count_nonzero_edges(&nonmax_edges);

    GrayImage::from(
        nonmax_edges
            .clone()
            .map(|edge| SubPixels([edge.get_magnitude()]))
            .map_subpixels(|v| f64::round(v * 255.0) as u8),
    )
    .save("test_outputs/myownlena_nonmax.png")
    .unwrap();

    let thresholded_edges = perform_hysteresis_thresholding(width, height,
         &nonmax_edges, 0.05, 0.1, 2);
    count_edge_types(&thresholded_edges);

    GrayImage::from(thresholded_edges.clone().map(thresholded_edge_to_subpixels))
        .save("test_outputs/myownlena_hysteresis.png")
        .unwrap();

    Ok(())
}

fn thresholded_edge_to_subpixels(edge: ThresholdedEdge) -> SubPixels<u8, 1> {
    SubPixels([match edge {
        ThresholdedEdge::STRONG => 255,
        ThresholdedEdge::WEAK => 32,
        ThresholdedEdge::NULL => 0,
    }])
}

fn count_nonzero_edges(edges: &DynamicMatrix<Edge>) {
    println!(
        "Non-zero magnitudes: {}",
        edges
            .get_data()
            .iter()
            .filter(|edge| edge.get_magnitude() > 0.0)
            .count()
    );
}

fn count_edge_types(edges: &DynamicMatrix<ThresholdedEdge>) {
    println!(
        "Strong edges: {}",
        edges
            .get_data()
            .iter()
            .filter(|edge| matches!(edge, ThresholdedEdge::STRONG))
            .count()
    );

    println!(
        "Weak edges: {}",
        edges
            .get_data()
            .iter()
            .filter(|edge| matches!(edge, ThresholdedEdge::WEAK))
            .count()
    );
}
