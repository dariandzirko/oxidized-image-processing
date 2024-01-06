use ndarray::{array, Array2};

use std::f32::consts::E;

//Going to start with assuming rectangular kernels
pub struct Kernel {
    //Maybe ndarray will be a lot faster here
    //TODO look into ndarray
    pub matrix: Array2<f32>,
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

    //Work on this func
    pub fn print_kernel(&self) {
        // let (rows, cols) = self.dimensions;

        // println!();
        // for row in 0..rows {
        //     for col in 0..cols {
        //         print!("{:?} | ", self.matrix.get((row, col)));
        //     }
        //     println!();
        // }

        println!("The kernel is {:?}", self.matrix);
        println!("With dimensions {:?}", self.matrix.dim());
    }

    pub fn sum_kernel(&self) -> f32 {
        let mut sum = 0.0;

        self.matrix.iter().for_each(|item| sum += *item);

        return sum;
    }

    //This is not 100% yet at the moment, the other specific filters won't be either I don't think
    //should document the use case of radius here, it is basically an i32
    pub fn gaussian_1d(radius: f32) -> Kernel {
        let lim = 3.0 * (radius.floor() as f32);
        let length = (2.0 * lim + 1.0) as usize;

        let mut matrix = Array2::<f32>::zeros((1, length));

        let mut sum = 0.0;

        matrix.indexed_iter_mut().for_each(|(index, item)| {
            *item = E.powf(-(index.1 as f32 - (lim)).powf(2.0)) / (2.0 * radius.powf(2.0));
            sum += *item;
        });

        matrix.iter_mut().for_each(|item| *item = *item / sum);

        Kernel {
            matrix: matrix,
            dimensions: (1, length),
        }
    }

    pub fn gaussian_2d(radius: f32) -> Kernel {
        let lim = (3.0 * radius).floor() as f32;
        let length = (2.0 * lim + 1.0) as usize;

        let mut matrix = Array2::<f32>::zeros((length, length));

        let mut sum = 0.0;

        matrix.indexed_iter_mut().for_each(|(index, item)| {
            *item = E.powf(
                -((index.0 as f32 - lim).powf(2.0) + (index.1 as f32 - lim).powf(2.0))
                    / (2.0 * radius.powf(2.0)),
            );
            sum += *item;
        });

        matrix.iter_mut().for_each(|item| *item = *item / sum);

        Kernel {
            matrix: matrix,
            dimensions: (length, length),
        }
    }

    pub fn highpass_2d(radius: f32) -> Kernel {
        let mut dummy_filt = Kernel::gaussian_2d(radius);

        let lim = (dummy_filt.dimensions.0 - 1) / 2;

        dummy_filt
            .matrix
            .iter_mut()
            .for_each(|item| *item = -(*item));

        dummy_filt.matrix[[lim, lim]] += 1.0;

        return dummy_filt;
    }

    pub fn sharpening_2d(radius: f32, beta: f32) -> Kernel {
        let mut dummy_filt = Kernel::highpass_2d(radius);

        let lim = (dummy_filt.matrix.dim().0 - 1) / 2;

        dummy_filt.matrix.iter_mut().for_each(|item| *item /= beta);

        dummy_filt.matrix[[lim, lim]] += 1.0;

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
