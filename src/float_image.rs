use image::{
    DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage, Rgba,
};
use ndarray::Array2;
use num::Float;

use crate::kernel::{self, Kernel};

pub struct FloatImage {
    matrix: Array2<f32>,
    min: f32,
    max: f32,
}

impl Default for FloatImage {
    fn default() -> FloatImage {
        FloatImage {
            matrix: Array2::<f32>::zeros((0, 0)),
            min: 300.0,
            max: -1.0,
        }
    }
}

impl FloatImage {
    pub fn new(matrix: Array2<f32>) -> FloatImage {
        let mut float_image = FloatImage::default();
        float_image.matrix = matrix;
        float_image.populate_min_max();
        float_image
    }

    pub fn from_luma8(image: GrayImage) -> FloatImage {
        let (col_num, row_num) = image.dimensions();
        let mut matrix = Array2::<f32>::zeros((row_num as usize, col_num as usize));
        let mut pixel_value = 0.0;

        image.enumerate_pixels().for_each(|(row, col, pixel)| {
            pixel_value = *pixel.channels().get(0).unwrap() as f32;
            matrix[[row as usize, col as usize]] = pixel_value;
        });

        FloatImage::new(matrix)
    }

    pub fn populate_min_max(&mut self) {
        self.matrix.iter().for_each(|item| {
            if *item < self.min {
                self.min = *item;
            }
            if *item > self.max {
                self.max = *item;
            }
        })
    }

    pub fn to_luma8(&self) -> GrayImage {
        let (col_num, row_num) = self.matrix.dim();

        let mut result = GrayImage::new(col_num as u32, row_num as u32);

        self.matrix.indexed_iter().for_each(|((col, row), item)| {
            result.put_pixel(
                col as u32,
                row as u32,
                image::Luma::<u8>([((*item - self.min) * 255.0 / (self.max - self.min)) as u8]),
            );
        });

        result
    }
}

// I think these shoudln't exist and it should just all essentially be help_ops that perform all the math on Array2<f32>
// pub fn conv_2d(filter: Kernel, image: FloatImage, same_size: bool) -> FloatImage {}

// pub conv_2d(filter: Array2, image: Array2) {

// pub fn integral_image() {}
// pub fn integral_image_matrix() {}
// pub fn haar_filter() {}
// pub fn image_raised_power() {}
// pub fn local_statistics() {}
// pub fn subtract_image() {}
