use convolve2d::{DynamicMatrix, Matrix};

use crate::edge::Edge;

pub fn perform_nonmax_suppression(
    width: usize,
    height: usize,
    drog_edges: &DynamicMatrix<Edge>,
    distance_range: usize
) -> DynamicMatrix<Edge> {
    let image_size = width * height;
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

                if is_max(
                    row,
                    col,
                    width,
                    image_size,
                    &drog_edges,
                    &edge,
                    distance_range,
                ) {
                    edge
                } else {
                    Edge::zero()
                }
            })
            .collect(),
    )
    .unwrap();
    nonmax_edges
}

fn is_max(
    row: usize,
    col: usize,
    width: usize,
    image_size: usize,
    drog_edges: &DynamicMatrix<Edge>,
    edge: &Edge,
    distance_range: usize,
) -> bool {
    let distance_range = distance_range as i32;
    for distance in -distance_range..distance_range {
        let (dir_x, dir_y) = edge.dir();

        let (near_row_offset, near_col_offset): (i32, i32) = (
            (dir_x.signum() as i32) * (if dir_x.abs() > 0.25 { distance } else { 0 }),
            (dir_y.signum() as i32) * (if dir_y.abs() > 0.25 { distance } else { 0 }),
        );
        let near_row: i32 = row as i32 + near_row_offset;
        let near_col: i32 = col as i32 + near_col_offset;

        let near_index = near_row * (width as i32) + near_col;

        if near_index < 0 || near_index >= image_size as i32 {
            continue;
        }

        let near_index = near_index as usize;

        let near_edge = drog_edges.get_data()[near_index];

        if edge.get_magnitude() < near_edge.get_magnitude() {
            return false;
        }
    }

    true
}
