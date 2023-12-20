pub mod float_image;
pub mod helper_ops;
pub mod kernel;

use ndarray::{array, Array1, Array2};

use std::f32::consts::E;

use crate::helper_ops::vec_of_vec_into_array2;

//Going to start with assuming rectangular kernels
pub struct Kernel {
    //Maybe ndarray will be a lot faster here
    //TODO look into ndarray
    matrix: Array2<f32>,
    // Create a window with default options and display the image.
    dimensions: (usize, usize), //row, col
}

impl Kernel {
    //How do I return nothing instead of an 1,1 matrix? Option on the constructor?
    pub fn new() -> Kernel {
        let matrix = array![[0.0]];
        // Should keep the dimensions as (width, height) to match the library
        let dimensions = (1, 1);
        Kernel { matrix, dimensions }
    }

    pub fn print_kernel(&self) {
        let (rows, cols) = self.dimensions;

        println!();
        for row in 0..rows {
            for col in 0..cols {
                print!("{:?} | ", self.matrix.get((row, col)));
            }
            println!();
        }
    }

    pub fn sum_kernel(&self) -> f32 {
        let mut sum = 0.0;

        self.matrix.iter().for_each(|item| sum += *item);

        return sum;
    }

    //This is not 100% yet at the moment, the other specific filters won't be either I don't think
    //should document the use case of radius here, it is basically an i32
    pub fn gaussian_1d(radius: f32) {
        let mut dummy_filt = Kernel::new();

        let lim = 3.0 * (radius.floor() as f32);
        let length = (2.0 * lim + 1.0) as usize;

        let mut matrix = vec![vec![0.0; length]; 1];
        let dimensions = (1, length);

        let mut sum = 0.0;

        for i in 0..length + 1 {
            let x = i as f32 - (lim);
            let val = E.powf(-(x.powf(2.0)) / (2.0 * radius.powf(2.0)));
            matrix[0][i] = val;
            sum += val;
        }

        matrix
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item = *item / sum);

        Kernel {
            matrix: vec_of_vec_into_array2(&mut matrix, dimensions),
            dimensions: dimensions,
        };
    }

    pub fn gaussian_2d(radius: f32) -> Kernel {
        let lim = (3.0 * radius).floor() as f32;
        let length = (2.0 * lim + 1.0) as usize;

        let mut matrix = vec![vec![0.0; length]; length];
        let dimensions = (length, length);

        let mut sum = 0.0;

        for row in 0..length {
            let x = row as f32 - (lim);
            for col in 0..length {
                let y = col as f32 - (lim);
                let val = E.powf(-(x.powf(2.0) + y.powf(2.0)) / (2.0 * radius.powf(2.0)));
                matrix[row][col] = val;
                sum += val;
            }
        }

        matrix
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item = *item / sum);

        Kernel {
            matrix: vec_of_vec_into_array2(&mut matrix, dimensions),
            dimensions: dimensions,
        }
    }

    pub fn highpass_2d(radius: f32) -> Kernel {
        let mut dummy_filt = Kernel::gaussian_2d(radius);

        let lim = (dummy_filt.dimensions.0 - 1) / 2;

        dummy_filt
            .matrix
            .iter_mut()
            .for_each(|item| *item = -(*item));

        //This is a lazy solution, but I don't think that there should be a situation where this returns None
        *dummy_filt.matrix.get_mut((lim, lim)).unwrap() += 1.0;

        return dummy_filt;
    }

    pub fn sharpening_2d(radius: f32, beta: f32) -> Kernel {
        let mut dummy_filt = Kernel::highpass_2d(radius);

        let lim = (dummy_filt.dimensions.0 - 1) / 2;

        dummy_filt.matrix.iter_mut().for_each(|item| *item /= beta);

        //This is a lazy solution, but I don't think that there should be a situation where this returns None pt. 2
        *dummy_filt.matrix.get_mut((lim, lim)).unwrap() += 1.0;

        return dummy_filt;
    }

    pub fn sobel_y_dir() -> Kernel {
        let matrix = array![[1., 2., 1.], [0., 0., 0.], [-1., -2., -1.]];

        Kernel {
            matrix,
            dimensions: (3, 3),
        }
    }

    pub fn sobel_x_dir() -> Kernel {
        let matrix = array![[1., 0., -1.], [2., 0., -2.], [1., 0., -1.]];

        Kernel {
            matrix,
            dimensions: (3, 3),
        }
    }
}
