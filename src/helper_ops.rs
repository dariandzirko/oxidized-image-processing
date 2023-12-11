use ndarray::{iter::IterMut, Array2, Axis};

// pub fn flip_x(&mut self) {

//     self.matrix = flip_x(self.matrix);
//
// }
// row
//
// 1 2 3  c
// 4 5 6  o
// 7 8 9  l
//--- flip x ---
// 7 8 9
// 4 5 6
// 1 2 3

//Make these 2 accept more generic primitive types
pub fn flip_across_x(matrix: &mut Array2<f32>) {
    let (rows, cols) = matrix.dim();

    (0..cols).into_iter().for_each(|col| {
        (0..rows / 2).into_iter().for_each(|row| {
            matrix.swap((row, col), (rows - 1 - row, col));
        })
    });
}

pub fn flip_across_y(matrix: &mut Array2<f32>) {
    let (rows, cols) = matrix.dim();

    (0..rows).into_iter().for_each(|row| {
        (0..cols / 2).into_iter().for_each(|col| {
            matrix.swap((row, col), (row, cols - 1 - col));
        })
    });

    // Some random github solution from the ndarray developers
    // let mut it = matrix.axis_iter_mut(Axis(0));

    // ndarray::Zip::from(it.nth(perm.0).unwrap())
    //     .and(it.nth(perm.1 - (perm.0 + 1)).unwrap())
    //     .apply(std::mem::swap);
}

// pub fn flip_2d(&mut self) -> Kernel {
//     let mut dummy_filt = self.flip_x();
//     dummy_filt = dummy_filt.flip_y();

//     return dummy_filt;
// }

#[cfg(test)]
mod test {
    use ndarray::array;

    use super::flip_across_x;
    use super::flip_across_y;

    #[test]
    fn flip_across_x_test() {
        let mut matrix = array![[1., 1., 1.], [2., 2., 2.], [3., 3., 3.]];
        flip_across_x(&mut matrix);
        let flipped_matrix = array![[3., 3., 3.], [2., 2., 2.], [1., 1., 1.]];
        assert_eq!(matrix, flipped_matrix);
    }

    #[test]
    fn flip_across_y_test() {
        let mut matrix = array![[1., 2., 3.], [1., 2., 3.], [1., 2., 3.]];
        flip_across_y(&mut matrix);
        let flipped_matrix = array![[3., 2., 1.], [3., 2., 1.], [3., 2., 1.]];
        assert_eq!(matrix, flipped_matrix);
    }

    #[test]
    fn test_swap() {
        let mut matrix = array![[1., 1., 1.], [2., 2., 2.], [3., 3., 3.]];
        matrix.swap((0, 0), (1, 1));
        // matrix.swap((2, 2), (1, 2));
        let flipped_matrix = array![[2., 1., 1.], [2., 1., 2.], [3., 3., 3.]];
        assert_eq!(matrix, flipped_matrix);
    }

    // fn flip_across_x_twice() {}
}
