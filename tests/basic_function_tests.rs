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
        println!("result {}", result);
    }

    #[test]
    fn slightly_hard_convolution_2d() {
        let mut kernel = array![[1., 3., 1.], [1., 3., 1.], [1., 3., 1.]];
        let base = array![[2., 4., 2.], [2., 5., 2.], [6., 8., 9.]];

        let result = conv_2d(&mut kernel, &base, true);
        println!("result {}", result);
    }
}
