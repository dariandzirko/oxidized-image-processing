use image::{GrayImage, Pixel};
use ndarray::Array2;

pub struct FloatImage {
    pub matrix: Array2<f32>,
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
        let mut matrix = Array2::<f32>::zeros((col_num as usize, row_num as usize));
        let mut pixel_value = 0.0;

        image.enumerate_pixels().for_each(|(col, row, pixel)| {
            pixel_value = *pixel.channels().get(0).unwrap() as f32;
            matrix[[col as usize, row as usize]] = pixel_value;
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

    //First it will functionaly scale between 0 and 1 then it will just multiply the fraction by the desired constant
    pub fn scale_to_constant(&mut self, constant: f32) {
        self.matrix
            .iter_mut()
            .for_each(|item| *item = (*item - self.min) / (self.max - self.min) * constant);
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

// pub fn local_statistics() {}
// pub fn subtract_image() {}
