#[cfg(test)]
mod test {
    use oxidized_image_processing::{
        float_image::FloatImage,
        helper_ops::{conv_2d, haar_filter, integral_image, local_statistics, subtract_images},
        kernel,
        otsu::otsu,
    };

    //Can I make one instance of 'boy_image' then just copy the matrix all over the place

    #[test]
    fn test_horizontal_haar_filter() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let haar_boy_matrix = haar_filter(&boy_float_image.matrix, 20, 25);
        let haar_boy_image = FloatImage::new(haar_boy_matrix);
        haar_boy_image
            .to_luma8()
            .save("images/outputs/test_horizontal_haar_filter.png")
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

        let sharpen_boy_matrix = conv_2d(&mut sharpen_filter.matrix, &boy_float_image.matrix);

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

        let blur_boy_matrix = conv_2d(&mut blur_filter.matrix, &boy_float_image.matrix);

        let blur_boy_image = FloatImage::new(blur_boy_matrix);
        blur_boy_image
            .to_luma8()
            .save("images/outputs/blur_boy_image.png")
            .unwrap();
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
}
