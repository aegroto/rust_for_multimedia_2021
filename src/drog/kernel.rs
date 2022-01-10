use convolve2d::DynamicMatrix;

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