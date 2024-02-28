#[cfg(test)]
mod test {
    use num::Float;
    use oxidized_image_processing::{
        float_image::FloatImage,
        haar_filter::{apply_haar_filter, HaarFilter},
        helper_ops::{conv_2d, integral_image, local_statistics, subtract_images, zero_pad},
        kernel,
        otsu::otsu,
    };

    //Can I make one instance of 'boy_image' then just copy the matrix all over the place

    #[test]
    fn test_horizontal_haar_filter() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let base_shape: ndarray::prelude::Dim<[usize; 2]> = boy_float_image.matrix.raw_dim();

        let haar_filter = HaarFilter::two_rectangle_horizontal(20, 25);

        //Can make the integral image here staticed, and referenced
        let zero_pad_base = zero_pad(
            &boy_float_image.matrix,
            haar_filter.offset_x,
            haar_filter.offset_y,
            base_shape[0] + 2 * haar_filter.offset_x,
            base_shape[1] + 2 * haar_filter.offset_y,
        );

        // let mut result = Array2::<f32>::zeros(base_shape);

        let integral_zero_pad_base = integral_image(&zero_pad_base);

        let haar_boy_matrix = apply_haar_filter(base_shape, haar_filter, &integral_zero_pad_base);
        let haar_boy_image = FloatImage::new(haar_boy_matrix);
        haar_boy_image
            .to_luma8()
            .save("images/outputs/test_horizontal_haar_filter.png")
            .unwrap();
    }

    #[ignore = "experiment"]
    #[test]
    fn experiment_subtract_haar_images() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let test_horizontal_haar_filter_image =
            image::open("images/outputs/test_horizontal_haar_filter.png")
                .unwrap()
                .to_luma8();

        let test_horizontal_haar_filter_float_image =
            FloatImage::from_luma8(test_horizontal_haar_filter_image);

        let boy_minux_test_matrix = subtract_images(
            &boy_float_image.matrix,
            &test_horizontal_haar_filter_float_image.matrix,
        );
        let boy_minus_test_image = FloatImage::new(boy_minux_test_matrix);
        boy_minus_test_image
            .to_luma8()
            .save("images/outputs/boy_minus_test_image.png")
            .unwrap();

        let truth_horizontal_haar_filter_image =
            image::open("images/outputs/truth_horizontal_haar_filter.png")
                .unwrap()
                .to_luma8();
        let truth_horizontal_haar_filter_float_image =
            FloatImage::from_luma8(truth_horizontal_haar_filter_image);

        let boy_minux_truth_matrix = subtract_images(
            &boy_float_image.matrix,
            &truth_horizontal_haar_filter_float_image.matrix,
        );
        let boy_minus_truth_image = FloatImage::new(boy_minux_truth_matrix);
        boy_minus_truth_image
            .to_luma8()
            .save("images/outputs/boy_minus_truth_image.png")
            .unwrap();
    }

    #[test]
    fn test_integral_image() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let integral_boy_matrix = integral_image(&boy_float_image.matrix);
        let integral_boy_image = FloatImage::new(integral_boy_matrix);
        integral_boy_image
            .to_luma8()
            .save("images/outputs/integral_boy_image.png")
            .unwrap();
    }

    #[test]
    fn test_local_statistics() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let (standard_dev_boy_matrix, mean_boy_matrix) =
            local_statistics(&boy_float_image.matrix, 5, 5);
        let mean_boy_image = FloatImage::new(mean_boy_matrix);
        mean_boy_image
            .to_luma8()
            .save("images/outputs/mean_boy_image.png")
            .unwrap();
        let boy_minus_standard_dev_image = FloatImage::new(subtract_images(
            &boy_float_image.matrix,
            &standard_dev_boy_matrix,
        ));
        boy_minus_standard_dev_image
            .to_luma8()
            .save("images/outputs/boy_minus_standard_dev_image.png")
            .unwrap();
        let standard_dev_boy_image = FloatImage::new(standard_dev_boy_matrix);
        standard_dev_boy_image
            .to_luma8()
            .save("images/outputs/standard_dev_boy_image.png")
            .unwrap();
    }

    #[test]
    fn test_subtract_images() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let boy_minus_boy_matrix =
            subtract_images(&boy_float_image.matrix, &boy_float_image.matrix);
        let boy_minus_boy_image = FloatImage::new(boy_minus_boy_matrix);
        boy_minus_boy_image
            .to_luma8()
            .save("images/outputs/boy_minus_boy_image.png")
            .unwrap();
    }

    #[test]
    fn test_subtract_images_negatives() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        // let two_times_boy_matrix = boy_float_image.matrix.for_each(|item| *item *= 2.0);
        let two_times_boy_matrix = boy_float_image.matrix.map(|item| *item * 2.0);

        let boy_minus_two_times_boy_matrix =
            subtract_images(&boy_float_image.matrix, &two_times_boy_matrix);
        let boy_minus_two_times_boy_image = FloatImage::new(boy_minus_two_times_boy_matrix);
        boy_minus_two_times_boy_image
            .to_luma8()
            .save("images/outputs/boy_minus_two_times_boy_matrix.png")
            .unwrap();
    }

    #[test]
    fn test_sharpen() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let mut sharpen_filter = kernel::Kernel::sharpening_2d(1.8, 2.5);

        let sharpen_boy_matrix =
            conv_2d(&mut sharpen_filter.matrix, &boy_float_image.matrix, false);

        let sharpen_boy_image = FloatImage::new(sharpen_boy_matrix);
        sharpen_boy_image
            .to_luma8()
            .save("images/outputs/sharpen_boy_image.png")
            .unwrap();
    }

    #[test]
    fn test_blur() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let mut blur_filter = kernel::Kernel::gaussian_2d(1.8);

        let blur_boy_matrix = conv_2d(&mut blur_filter.matrix, &boy_float_image.matrix, false);

        let blur_boy_image = FloatImage::new(blur_boy_matrix);
        blur_boy_image
            .to_luma8()
            .save("images/outputs/blur_boy_image.png")
            .unwrap();
    }

    #[test]
    fn test_blur_same_size() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let mut blur_filter = kernel::Kernel::gaussian_2d(1.8);

        let blur_boy_matrix_same = conv_2d(&mut blur_filter.matrix, &boy_float_image.matrix, true);

        let blur_boy_image_same = FloatImage::new(blur_boy_matrix_same);
        blur_boy_image_same
            .to_luma8()
            .save("images/outputs/blur_boy_image_same.png")
            .unwrap();

        assert_eq!(
            blur_boy_image_same.matrix.raw_dim(),
            boy_float_image.matrix.raw_dim()
        )
    }

    #[test]
    fn test_otsu() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let otsu_boy_matrix = otsu(&boy_float_image.matrix);
        let otsu_boy_image = FloatImage::new(otsu_boy_matrix);
        otsu_boy_image
            .to_luma8()
            .save("images/outputs/otsu_boy_image.png")
            .unwrap();
    }

    #[test]
    fn test_downsample_by_factor() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let mut boy_float_image = FloatImage::from_luma8(boy_image);

        boy_float_image.downsample_by_factor(2);
        boy_float_image
            .to_luma8()
            .save("images/outputs/boy_image_downsample_two.png")
            .unwrap();
    }

    #[test]
    fn test_blur_and_downsample_by_factor() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let mut boy_float_image = FloatImage::from_luma8(boy_image);

        boy_float_image.blur_and_downsample_by_factor(2);
        boy_float_image
            .to_luma8()
            .save("images/outputs/boy_image_blur_downsample_two.png")
            .unwrap();
    }

    #[test]
    fn experiment_subtract_downsampled_images() {
        let boy_image_blur_downsample_two =
            image::open("images/outputs/boy_image_blur_downsample_two.png")
                .unwrap()
                .to_luma8();
        let boy_float_image_blur_downsample_two =
            FloatImage::from_luma8(boy_image_blur_downsample_two);

        let boy_image_downsample_two = image::open("images/outputs/boy_image_downsample_two.png")
            .unwrap()
            .to_luma8();
        let boy_float_image_downsample_two = FloatImage::from_luma8(boy_image_downsample_two);

        let downsample_subtraction = subtract_images(
            &boy_float_image_blur_downsample_two.matrix,
            &boy_float_image_downsample_two.matrix,
        );

        let downsample_subtraction_image = FloatImage::new(downsample_subtraction);
        downsample_subtraction_image
            .to_luma8()
            .save("images/outputs/downsample_subtraction_image.png")
            .unwrap();
    }
}
