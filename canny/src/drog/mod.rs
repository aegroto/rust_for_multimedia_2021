use convolve2d::{DynamicMatrix, Matrix, SubPixels, convolve2d};
use image::GrayImage;

use crate::edge::Edge;

mod kernel;

pub fn perform_drog_convolution(
    normalized_image_matrix: &DynamicMatrix<SubPixels<f64, 1>>,
    kernel_size: usize,
    sigma: f64
) -> DynamicMatrix<Edge> {
    let (drog_kernel_x, drog_kernel_y) = kernel::drog(kernel_size, sigma);

    println!("Drog kernel x: {:#?}", drog_kernel_x);
    println!("Drog kernel y: {:#?}", drog_kernel_y);

    println!("Performing DroG X convolution...");
    let drog_x_convolution = convolve2d(normalized_image_matrix, &drog_kernel_x);
    GrayImage::from(
        drog_x_convolution
            .clone()
            .map_subpixels(|v| f64::round(v * 255.0) as u8),
    )
    .save("test_outputs/myownlena_drog_x.png")
    .unwrap();
    println!("Performing DroG Y convolution...");
    let drog_y_convolution = convolve2d(normalized_image_matrix, &drog_kernel_y);
    GrayImage::from(
        drog_y_convolution
            .clone()
            .map_subpixels(|v| f64::round(v * 255.0) as u8),
    )
    .save("test_outputs/myownlena_drog_y.png")
    .unwrap();

    println!("Calculating DroG magnitude...");
    let indices_sequence = 0..normalized_image_matrix.get_data().len();

    let drog_edges: DynamicMatrix<Edge> = DynamicMatrix::new(
        normalized_image_matrix.get_width(),
        normalized_image_matrix.get_height(),
        indices_sequence
            .map(|i| {
                Edge::new(
                    drog_x_convolution.get_data()[i].0[0],
                    drog_y_convolution.get_data()[i].0[0]
                )
            })
            .collect(),
    )
    .unwrap();

    drog_edges
}
