use std::f64::consts::FRAC_1_SQRT_2;

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    x_dir: f64,
    y_dir: f64,
    magnitude: f64
}

impl Edge {
    pub fn new(vec_x: f64, vec_y: f64) -> Self {
        let vec_x = FRAC_1_SQRT_2 * vec_x;
        let vec_y = FRAC_1_SQRT_2 * vec_y;
        let magnitude = f64::hypot(vec_x, vec_y);

        let magnitude_recip = if magnitude != 0.0 {
            magnitude.recip()
        } else {
            1.0
        };

        Self {
            x_dir: vec_x * magnitude_recip,
            y_dir: vec_y * magnitude_recip,
            magnitude
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn get_magnitude(&self) -> f64 {
        self.magnitude
    }

    pub fn angle(&self) -> f64 {
        f64::atan2(self.y_dir, self.x_dir)
    }

    pub fn dir(&self) -> (f64, f64) {
        (self.x_dir * self.get_magnitude(), self.y_dir * self.get_magnitude())
    }

    pub fn dir_norm(&self) -> (f64, f64) {
        (self.x_dir, self.y_dir)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ThresholdedEdge {
    STRONG,
    WEAK,
    NULL 
}