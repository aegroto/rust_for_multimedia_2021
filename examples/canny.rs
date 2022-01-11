use image::{GrayImage, ImageError};

use convolve2d::*;
use rust_for_multimedia::{
    drog::perform_drog_convolution, edge::Edge, nonmax::perform_nonmax_suppression,
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

    Ok(())
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
