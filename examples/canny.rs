use image::{GrayImage, ImageError};

use convolve2d::*;
use rust_for_multimedia::{drog::perform_drog_convolution, edge::Edge};

fn main() -> Result<(), ImageError> {
    // Params
    let sigma = 2.0;
    let kernel_size = 10;

    let weak_edge_threshold = 0.0;
    let strong_edge_threshold = 0.5;

    // Load image and convert to Luma8
    let image_reader = image::io::Reader::open("test_assets/myownlena.jpg")?;
    let image = image_reader.decode()?;
    let image_luma = image.into_luma8();
    image_luma.save("test_outputs/myownlena_luma8.jpg").unwrap();

    let image_matrix: DynamicMatrix<SubPixels<u8, 1>> = image_luma.into();

    let normalized_image_matrix = image_matrix.map_subpixels(|x| (x as f64) / 255.0);

    // DroG convolution
    let drog_edges = perform_drog_convolution(&normalized_image_matrix, kernel_size, sigma);

    // Non-maximum suppression
    println!("Applying non-maximum suppression...");
    let (width, height) = (
        normalized_image_matrix.get_width(),
        normalized_image_matrix.get_height(),
    );

    let image_size = width*height;
    let edges_indices = 0..image_size;

    let nonmax_edges: DynamicMatrix<Edge> = DynamicMatrix::new(
        width,
        height,
        edges_indices
            .into_iter()
            .map(|index| {
                let row: usize = index / width;
                let col: usize = index - (row * width);

                let edge = drog_edges.get_data()[index];

                if edge.get_magnitude() < weak_edge_threshold {
                    return Edge::zero();
                }

                let (dir_x, dir_y) = edge.dir();
                let (near_row_offset , near_col_offset): (usize, usize) = (
                    (dir_x.signum() as usize) * (if dir_x.abs() > 0.5 { 1 } else { 0 }),
                    (dir_y.signum() as usize) * (if dir_y.abs() > 0.5 { 1 } else { 0 }),
                );

                let near_row = row + near_row_offset;
                let near_col = col + near_col_offset;

                let near_index = near_row * width + near_col;

                if near_index > 0 && near_index < image_size {
                    let near_edge = drog_edges.get_data()[near_index];

                    if edge.get_magnitude() < near_edge.get_magnitude() {
                        return Edge::zero();
                    }
                }

                edge
            })
            .collect(),
    )
    .unwrap();

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
