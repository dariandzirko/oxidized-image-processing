#[cfg(test)]
mod test {
    use oxidized_image_processing::{
        float_image::FloatImage,
        helper_ops::{haar_filter, integral_image, local_statistics, subtract_images},
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

        let (mean_boy_matrix, standard_dev_boy_matrix) =
            local_statistics(&boy_float_image.matrix, 3, 3);
        let mean_boy_image = FloatImage::new(mean_boy_matrix);
        mean_boy_image
            .to_luma8()
            .save("images/outputs/mean_boy_image.png")
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

        let haar_boy_matrix = haar_filter(&boy_float_image.matrix, 20, 25);
        let haar_boy_image = FloatImage::new(haar_boy_matrix);
        haar_boy_image
            .to_luma8()
            .save("images/outputs/test_horizontal_haar_filter.png")
            .unwrap();
    }

    #[test]
    fn test_blur() {
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
    fn test_otsu() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let haar_boy_matrix = haar_filter(&boy_float_image.matrix, 20, 25);
        let haar_boy_image = FloatImage::new(haar_boy_matrix);
        haar_boy_image
            .to_luma8()
            .save("images/outputs/test_horizontal_haar_filter.png")
            .unwrap();
    }
}
