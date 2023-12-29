#[cfg(test)]
mod test {
    use ndarray::array;

    use oxidized_image_processing::kernel::Kernel;

    use oxidized_image_processing::helper_ops::conv_2d;
    use oxidized_image_processing::helper_ops::flip_across_x;
    use oxidized_image_processing::helper_ops::flip_across_y;

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

    #[test]
    fn basic_convolution_2d() {
        let mut kernel = array![[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]];
        let base = array![[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]];

        let result = conv_2d(&mut kernel, &base, true);

        let expected_result = array![
            [1., 2., 3., 2., 1.],
            [2., 4., 6., 4., 2.],
            [3., 6., 9., 6., 3.],
            [2., 4., 6., 4., 2.],
            [1., 2., 3., 2., 1.]
        ];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn slightly_hard_convolution_2d() {
        let mut kernel = array![[1., 3., 1.], [1., 3., 1.], [1., 3., 1.]];
        let base = array![[2., 4., 2.], [2., 5., 2.], [6., 8., 9.]];

        let result = conv_2d(&mut kernel, &base, true);

        let expected_result = array![
            [2., 10., 16., 10., 2.],
            [4., 21., 35., 21., 4.],
            [10., 47., 74., 56., 13.],
            [8., 37., 58., 46., 11.],
            [6., 26., 39., 35., 9.]
        ];

        assert_eq!(result, expected_result);
    }
}
