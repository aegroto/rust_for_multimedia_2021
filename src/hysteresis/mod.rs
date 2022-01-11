use convolve2d::{DynamicMatrix, Matrix};
use itertools::Itertools;

use crate::edge::{Edge, ThresholdedEdge};

pub fn perform_hysteresis_thresholding(
    width: usize,
    height: usize,
    input_edges: &DynamicMatrix<Edge>,
    weak_edge_threshold: f64,
    strong_edge_threshold: f64,
    neighbourhood_size: usize,
) -> DynamicMatrix<ThresholdedEdge> {
    let image_size = width * height;
    let edges_indices = 0..image_size;
    let neighbourhood_size = neighbourhood_size as i32;

    let thresholds_data = edges_indices
        .into_iter()
        .map(|index| {
            let edge = input_edges.get_data()[index];

            if edge.get_magnitude() < weak_edge_threshold {
                ThresholdedEdge::NULL
            } else if edge.get_magnitude() > strong_edge_threshold {
                ThresholdedEdge::STRONG
            } else {
                ThresholdedEdge::WEAK
            }
        })
        .collect();

    let thresholds: DynamicMatrix<ThresholdedEdge> =
        DynamicMatrix::new(width, height, thresholds_data).unwrap();

    let thresholded_edges_data = thresholds
        .get_data()
        .iter()
        .enumerate()
        .map(|(index, edge_type)| match edge_type {
            ThresholdedEdge::STRONG => ThresholdedEdge::STRONG,
            ThresholdedEdge::WEAK => {
                let row: usize = index / width;
                let col: usize = index - (row * width);

                let neighbourhood_range = -neighbourhood_size..neighbourhood_size;

                let has_strong_neighbour = neighbourhood_range
                    .clone()
                    .cartesian_product(neighbourhood_range)
                    .any(|(row_offset, col_offset)| {
                        let neighbour_row = row as i32 + row_offset;
                        let neighbour_col = col as i32 + col_offset;

                        let neighbour_index = neighbour_row * (width as i32) + neighbour_col;

                        if neighbour_index < 0 || neighbour_index >= image_size as i32 {
                            return false;
                        }

                        let neighbour_index = neighbour_index as usize;

                        let neighbour = thresholds.get_data()[neighbour_index];

                        matches!(neighbour, ThresholdedEdge::STRONG)
                    });

                if has_strong_neighbour {
                    ThresholdedEdge::STRONG
                } else {
                    ThresholdedEdge::NULL
                }
            }
            ThresholdedEdge::NULL => ThresholdedEdge::NULL,
        })
        .collect();

    let thresholded_edges: DynamicMatrix<ThresholdedEdge> =
        DynamicMatrix::new(width, height, thresholded_edges_data).unwrap();

    thresholded_edges
}
