use std::f32::consts::FRAC_1_SQRT_2;

use image::{GrayImage, ImageError};

use convolve2d::*;

pub fn drog(size: usize, std_dev: f64) -> (DynamicMatrix<f64>, DynamicMatrix<f64>) {
    let stride = (size >> 1) as f64;
    let exp_coefficient = -0.5 / (std_dev * std_dev);
    let coefficient = 1.0 / std_dev;
    let allocation = size * size;
    let std_dev_pow = std_dev.powi(2);

    // Set the values according to the gaussian function
    let mut x_data = std::vec![0.0; allocation];
    let mut y_data = std::vec![0.0; allocation];

    for i in 0..allocation {
        let r = (i / size) as f64 - stride;
        let c = (i % size) as f64 - stride;

        let x_sq = r * r + c * c;
        let gaussian_coefficient = coefficient * f64::exp(x_sq * exp_coefficient);

        x_data[i] = -(r / std_dev_pow) * gaussian_coefficient;
        y_data[i] = -(c / std_dev_pow) * gaussian_coefficient;
    }

    (
        DynamicMatrix::new(size, size, x_data).unwrap(),
        DynamicMatrix::new(size, size, y_data).unwrap(),
    )
}

fn main() -> Result<(), ImageError> {
    // Params
    let sigma = 2.0;
    let kernel_size = 10;
    // let kernel_integer_norm_factor = 1000.0;

    // Load image and convert to Luma8
    let image_reader = image::io::Reader::open("test_assets/myownlena.jpg")?;
    let image = image_reader.decode()?;
    let image_luma = image.into_luma8();
    image_luma.save("test_outputs/myownlena_luma8.jpg").unwrap();

    let image_matrix: DynamicMatrix<SubPixels<u8, 1>> = image_luma.into();
    // let image_matrix = image_matrix.map_subpixels(|x| x as i32);

    // Gradient
    // let gaussian_kernel = kernel::gaussian(kernel_size, sigma);
    // .map(|k| (k * kernel_integer_norm_factor) as i32);
    let (drog_kernel_x, drog_kernel_y) = drog(kernel_size, sigma);

    println!("Drog kernel x: {:#?}", drog_kernel_x);
    println!("Drog kernel y: {:#?}", drog_kernel_y);

    let normalized_image_matrix = image_matrix.map_subpixels(|x| (x as f64) / 255.0);

    // DroG X convolution
    println!("Performing DroG X convolution...");
    let drog_x_convolution = convolve2d(&normalized_image_matrix, &drog_kernel_x);

    GrayImage::from(
        drog_x_convolution
            .clone()
            .map_subpixels(|v| f64::round(v * 255.0) as u8),
    )
    .save("test_outputs/myownlena_drog_x.png")
    .unwrap();

    // DroG Y convolution
    println!("Performing DroG Y convolution...");
    let drog_y_convolution = convolve2d(&normalized_image_matrix, &drog_kernel_y);

    GrayImage::from(
        drog_y_convolution
            .clone()
            .map_subpixels(|v| f64::round(v * 255.0) as u8),
    )
    .save("test_outputs/myownlena_drog_y.png")
    .unwrap();

    // DroG magnitude
    println!("Calculating DroG magnitude...");
    let indices_sequence = 0..normalized_image_matrix.get_data().len();
    let drog_magnitude: DynamicMatrix<SubPixels<f64, 3>> = DynamicMatrix::new(
        normalized_image_matrix.get_width(),
        normalized_image_matrix.get_height(),
        indices_sequence
            .map(|i| {
                let x = drog_x_convolution.get_data()[i].0[0];
                let y = drog_x_convolution.get_data()[i].0[0];

                SubPixels([x, y, vec_magnitude(x, y)])
            })
            .collect(),
    )
    .unwrap();

    GrayImage::from(
        drog_magnitude
            .clone()
            .map_subpixels(|v| f64::round(v * 255.0) as u8)
            .map(|subpixel| SubPixels([subpixel.0[2]])),
    )
    .save("test_outputs/myownlena_drog_magnitude.png")
    .unwrap();

    Ok(())
}

fn vec_magnitude(x: f64, y: f64) -> f64 {
    let vec_x = FRAC_1_SQRT_2 * (x as f32);
    let vec_y = FRAC_1_SQRT_2 * (y as f32);

    f32::hypot(vec_x, vec_y) as f64
}
