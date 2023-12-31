#[cfg(test)]
mod test {
    use oxidized_image_processing::{
        float_image::{self, FloatImage},
        helper_ops::haar_filter,
    };

    #[test]
    fn test_haar_filter_for_real() {
        let boy_image = image::open("images/inputs/Boy.tif").unwrap().to_luma8();
        let boy_float_image = FloatImage::from_luma8(boy_image);

        let haar_boy_matrix = haar_filter(&boy_float_image.matrix, 20, 25);
        let haar_boy_image = FloatImage::new(haar_boy_matrix);
        haar_boy_image
            .to_luma8()
            .save("images/outputs/please_work.png")
            .unwrap();
    }
}
