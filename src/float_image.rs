use image::{
    DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage, Rgba,
};
use ndarray::Array2;
use num::Float;

use crate::kernel::Kernel;

pub struct FloatImage {
    matrix: Array2<f32>,
}

impl FloatImage {
    pub fn new(matrix: Array2<f32>) -> FloatImage {
        FloatImage { matrix }
    }

    pub fn from_luma8(image: GrayImage) -> FloatImage {
        let (col_num, row_num) = image.dimensions();
        let mut matrix = Array2::<f32>::zeros((row_num as usize, col_num as usize));

        image.enumerate_pixels().for_each(|(row, col, pixel)| {
            matrix[[row as usize, col as usize]] = *pixel.channels().get(0).unwrap() as f32;
        });

        FloatImage { matrix }
    }

    pub fn to_luma8(&self) -> GrayImage {}
}

pub fn conv_2d(filter: Kernel, image: FloatImage) -> FloatImage {
    let (filter_col_num, filter_row_num) = image.matrix.dim();
    let (image_col_num, image_row_num) = image.matrix.dim();

    let mut matrix = Array2::<f32>::zeros((row_num as usize, col_num as usize));

    FloatImage { matrix }
}
pub fn integral_image() {}
pub fn integral_image_matrix() {}
pub fn haar_filter() {}
pub fn image_raised_power() {}
pub fn local_statistics() {}
pub fn subtract_image() {}
