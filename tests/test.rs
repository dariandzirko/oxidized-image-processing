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
    fn flip_across_x_twice() {
        let mut matrix = array![[1., 1., 1.], [2., 2., 2.], [3., 3., 3.]];
        flip_across_x(&mut matrix);
        flip_across_x(&mut matrix);
        let flipped_matrix = array![[1., 1., 1.], [2., 2., 2.], [3., 3., 3.]];
        assert_eq!(matrix, flipped_matrix);
    }
}
