use ndarray::Array2;
use std::f32::consts::E;

//Going to start with assuming rectangular kernels
pub struct Kernel {
    matrix: Array2<f32>,
}

impl Kernel {
    //How do I return nothing from this
    pub fn new() -> Kernel {
        let matrix = Array2::<f32>::zeros((0, 0));
        // Should keep the dimensions as (width, height) to match the library
        Kernel { matrix }
    }

    pub fn print_kernel(&self) {
        println!("The kernel is {:?}", self.matrix);
        println!("With dimensions {:?}", self.matrix.dim());
    }

    pub fn sum_kernel(&self) -> f32 {
        let mut sum = 0.0;

        self.matrix.iter().for_each(|item| sum += *item);

        return sum;
    }

    pub fn gaussian_1d(radius: f32) -> Kernel {
        let mut dummy_filt = Kernel::new();

        let lim = 3.0 * (radius.floor() as f32);
        let length = (2.0 * lim + 1.0) as usize;

        let mut matrix = Array2::<f32>::zeros((1, length));

        let mut sum = 0.0;

        matrix.indexed_iter_mut().for_each(|(index, item)| {
            *item = E.powf(-(index.1 as f32 - (lim)).powf(2.0)) / (2.0 * radius.powf(2.0));
            sum += *item;
        });

        matrix.iter_mut().for_each(|item| *item = *item / sum);

        dummy_filt.matrix = matrix;

        return dummy_filt;
    }

    pub fn gaussian_2d(radius: f32) -> Kernel {
        let mut dummy_filt = Kernel::new();

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

        dummy_filt.matrix = matrix;

        return dummy_filt;
    }

    pub fn highpass_2d(radius: f32) -> Kernel {
        let mut dummy_filt = Kernel::gaussian_2d(radius);

        let lim = (dummy_filt.matrix.dim().0 - 1) / 2;

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
        let mut dummy_filt = Kernel::new();

        // Why does this not work >:\
        // dummy_filt.matrix.row_mut(0) = array![1.0, 2.0, 1.0].into_owned();
        // dummy_filt.matrix.row_mut(1) = array![0.0, 0.0, 0.0].into_owned();
        // dummy_filt.matrix.row_mut(2) = array![-1.0, -2.0, -1.0].into_owned();

        //Row 0
        dummy_filt.matrix.row_mut(0)[0] = 1.0;
        dummy_filt.matrix.row_mut(0)[1] = 2.0;
        dummy_filt.matrix.row_mut(0)[2] = 1.0;
        //Row 1
        dummy_filt.matrix.row_mut(1)[0] = 0.0;
        dummy_filt.matrix.row_mut(1)[1] = 0.0;
        dummy_filt.matrix.row_mut(1)[2] = 0.0;
        //Row 2
        dummy_filt.matrix.row_mut(2)[0] = -1.0;
        dummy_filt.matrix.row_mut(2)[1] = -2.0;
        dummy_filt.matrix.row_mut(2)[2] = -1.0;

        return dummy_filt;
    }

    pub fn sobel_x_dir() -> Kernel {
        let mut dummy_filt = Kernel::new();

        //col 0
        dummy_filt.matrix.column_mut(0)[0] = 1.0;
        dummy_filt.matrix.column_mut(0)[1] = 2.0;
        dummy_filt.matrix.column_mut(0)[2] = 1.0;
        //col 1
        dummy_filt.matrix.column_mut(1)[0] = 0.0;
        dummy_filt.matrix.column_mut(1)[1] = 0.0;
        dummy_filt.matrix.column_mut(1)[2] = 0.0;
        //col 2
        dummy_filt.matrix.column_mut(2)[0] = -1.0;
        dummy_filt.matrix.column_mut(2)[1] = -2.0;
        dummy_filt.matrix.column_mut(2)[2] = -1.0;

        return dummy_filt;
    }

    pub fn flip_x(&mut self) -> Kernel {
        let mut dummy_filt = Kernel::new();

        let (row, col) = self.matrix.dim();

        self.matrix
            .indexed_iter_mut()
            .for_each(|(index, item)| dummy_filt.matrix[[index.0, col - index.1]] = *item);

        return dummy_filt;
    }

    pub fn flip_y(&mut self) -> Kernel {
        let mut dummy_filt = Kernel::new();

        let (row, col) = self.matrix.dim();

        self.matrix
            .indexed_iter_mut()
            .for_each(|(index, item)| dummy_filt.matrix[[row - index.0, index.1]] = *item);

        return dummy_filt;
    }

    pub fn flip_2d(&mut self) -> Kernel {
        let mut dummy_filt = self.flip_x();
        dummy_filt = dummy_filt.flip_y();

        return dummy_filt;
    }
}
