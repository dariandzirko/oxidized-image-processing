use ndarray::Array2;

// pub fn flip_x(&mut self) {

//     self.matrix = flip_x(self.matrix);
//
// }

//Make this accept more generic primitive types
pub fn flip_x(matrix: &mut Array2<f32>) {
    let (row, col) = matrix.dim();

    matrix
        .indexed_iter_mut()
        .for_each(|(index, item)| matrix[[index.0, col - index.1]] = *item);
}

// pub fn flip_y(&mut self) -> Kernel {
//     let mut dummy_filt = Kernel::new();

//     let (row, col) = self.matrix.dim();

//     self.matrix
//         .indexed_iter_mut()
//         .for_each(|(index, item)| dummy_filt.matrix[[row - index.0, index.1]] = *item);

//     return dummy_filt;
// }

// pub fn flip_2d(&mut self) -> Kernel {
//     let mut dummy_filt = self.flip_x();
//     dummy_filt = dummy_filt.flip_y();

//     return dummy_filt;
// }
