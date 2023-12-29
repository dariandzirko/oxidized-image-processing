use ndarray::{Array1, Array2};

//Make these 2 accept more generic primitive types
pub fn flip_across_x(matrix: &mut Array2<f32>) {
    let (rows, cols) = matrix.dim();

    (0..cols).into_iter().for_each(|col| {
        (0..rows / 2).into_iter().for_each(|row| {
            matrix.swap((row, col), (rows - 1 - row, col));
        })
    });
}

pub fn flip_across_y(matrix: &mut Array2<f32>) {
    let (rows, cols) = matrix.dim();

    (0..rows).into_iter().for_each(|row| {
        (0..cols / 2).into_iter().for_each(|col| {
            matrix.swap((row, col), (row, cols - 1 - col));
        })
    });
}

pub fn flip_2d(matrix: &mut Array2<f32>) {
    flip_across_x(matrix);
    flip_across_y(matrix);
}

pub fn conv_2d(kernel: &mut Array2<f32>, base: &Array2<f32>, same_size: bool) -> Array2<f32> {
    //(width, height) everywhere

    //BUUUUUUUUT I don't know if it is row col so this will get me the correct shape but I don't know what the order of the dimensions is
    let base_shape = base.raw_dim();
    let kernel_shape = kernel.raw_dim();

    //There has to be a way I can skip this step and just do the math with out this initialization
    let mut zero_pad_base = Array2::<f32>::zeros((
        base_shape[0] + 2 * (kernel_shape[0] - 1),
        base_shape[1] + 2 * (kernel_shape[1] - 1),
    ));

    //overlay base onto the zero pad
    base.indexed_iter().for_each(|(index, item)| {
        zero_pad_base[(index.0 + kernel_shape[0] - 1, index.1 + kernel_shape[1] - 1)] = *item;
    });

    //Can change shape if I want to return a "same size" convolution image, so the dimensions would just be base_shape
    let mut result = Array2::<f32>::zeros((
        base_shape[0] + (kernel_shape[0] - 1),
        base_shape[1] + (kernel_shape[1] - 1),
    ));

    //flipping the kernel in both x and y
    flip_2d(kernel);

    //Will not work for "3D" images, like RGB stuff, I would need an array 3 where the 3rd dimension would be the RGB dimenion and would need to rewrite
    //float images and kernels to all be able to handle the 3d stuff which might be worthwhile in the future

    result.indexed_iter_mut().for_each(|(index, item)| {
        *item = 0.0;
        kernel
            .indexed_iter()
            .for_each(|(kernel_index, kernel_item)| {
                *item += zero_pad_base
                    .get((index.0 + kernel_index.0, index.1 + kernel_index.1))
                    .unwrap()
                    * kernel_item;
            });
    });

    result
}

//You typically will want to the raw integral image. In this case the value holder.
//So probably should return both. When creating the haar filter use value holder
//not result. Right now I will return the integral image
// pub fn integral_image(base: &Array2<f32>) -> Array2<f32> {
//     let (base_cols, base_rows) = base.dimensions();

//     let mut result = GrayImage::new(base_cols, base_rows);
//     let mut value_holder: Vec<Vec<f32>> =
//         vec![vec![0.0; (base_cols + 1) as usize]; (base_rows + 1) as usize];

//     let mut max: f32 = 0.0;

//     for row in 1..base_rows + 1 {
//         for col in 1..base_cols + 1 {
//             value_holder[row as usize][col as usize] =
//                 *base.get_pixel(col - 1, row - 1).channels().get(0).unwrap() as f32
//                     + value_holder[row as usize][(col - 1) as usize]
//                     + value_holder[(row - 1) as usize][col as usize]
//                     - value_holder[(row - 1) as usize][(col - 1) as usize];

//             if value_holder[row as usize][col as usize] > max {
//                 max = value_holder[row as usize][col as usize]
//             }
//         }
//     }

//     // Uncomment this if you want a normalize integral image not just a huge one
//     // value_holder.iter_mut()
//     // .flat_map(|row| row.iter_mut())
//     // .for_each(|item| *item = *item/max*255.0);

//     // for row in 0..base_rows {
//     //     for col in 0..base_cols {
//     //         let pixel: image::Luma::<u8> = image::Luma::<u8>([value_holder[row as usize][col as usize] as u8]);
//     //         result.put_pixel(col, row, pixel);
//     //     }
//     // }

//     return value_holder;
// }

// // This is a bad solution but whatever
// pub fn integral_image_matrix(base: Array2<f32>) -> Array2<f32> {
//     let base_rows = base.len();
//     let base_cols = base[0].len();

//     let mut value_holder: Vec<Vec<f32>> =
//         vec![vec![0.0; (base_cols + 1) as usize]; (base_rows + 1) as usize];

//     for row in 1..base_rows + 1 {
//         for col in 1..base_cols + 1 {
//             value_holder[row as usize][col as usize] = base[(row - 1) as usize][(col - 1) as usize]
//                 + value_holder[row as usize][(col - 1) as usize]
//                 + value_holder[(row - 1) as usize][col as usize]
//                 - value_holder[(row - 1) as usize][(col - 1) as usize];
//         }
//     }

//     return value_holder;
// }

// pub fn haar_filter(base: &Array2<f32>, Mh: u32, Mv: u32) -> Array2<f32> {
//     let (base_cols, base_rows) = base.dimensions();

//     let offset_row = Mv / 2;
//     let offset_col = Mh;

//     let mut zero_pad_base = GrayImage::new(base_cols + 2 * offset_col, base_rows + 2 * offset_row);
//     image::imageops::overlay(
//         &mut zero_pad_base,
//         base,
//         offset_col as i64,
//         offset_row as i64,
//     );

//     let mut value_holder: Vec<Vec<f32>> = vec![vec![0.0; base_cols as usize]; base_rows as usize];
//     let mut result = GrayImage::new(base_cols, base_rows);

//     let integral_zero_pad_base = integral_image(&zero_pad_base);

//     let mut gray = 0.0;
//     let mut white = 0.0;
//     let mut result_value = 0.0;
//     let mut max = 0.0;

//     for row in 1..base_rows - 1 {
//         for col in 0..base_cols {
//             gray = integral_zero_pad_base[(row + Mv) as usize][(col + Mh) as usize]
//                 - integral_zero_pad_base[(row + Mv) as usize][(col) as usize]
//                 - integral_zero_pad_base[row as usize][(col + Mh) as usize]
//                 + integral_zero_pad_base[row as usize][col as usize];

//             white = integral_zero_pad_base[(row + Mv) as usize][(col + 2 * Mh) as usize]
//                 - integral_zero_pad_base[(row + Mv) as usize][(col + Mh) as usize]
//                 - integral_zero_pad_base[row as usize][(col + 2 * Mh) as usize]
//                 + integral_zero_pad_base[row as usize][(col + Mh) as usize];

//             result_value = white - gray;
//             value_holder[(row - 1) as usize][col as usize] = result_value;

//             if max < result_value {
//                 max = result_value;
//             }
//         }
//     }

//     // let result_pixel: image::Luma::<u8> = image::Luma::<u8>([result_value]);

//     // result.put_pixel(col, row, result_pixel);

//     value_holder
//         .iter_mut()
//         .flat_map(|row| row.iter_mut())
//         .for_each(|item| *item = *item / max * 255.0 + 128.0);

//     for row in 0..base_rows {
//         for col in 0..base_cols {
//             let pixel: image::Luma<u8> =
//                 image::Luma::<u8>([value_holder[row as usize][col as usize] as u8]);
//             result.put_pixel(col, row, pixel);
//         }
//     }

//     return result;
// }

// pub fn image_raised_power(base: &Array2<f32>, power: f32) -> Array2<f32> {
//     let (base_cols, base_rows) = base.dimensions();

//     let mut float_result = vec![vec![0.0; base_cols as usize]; base_rows as usize];

//     //this should be an iterator
//     for row in 0..base_rows {
//         for col in 0..base_cols {
//             float_result[row as usize][col as usize] =
//                 (*base.get_pixel(col, row).channels().get(0).unwrap() as f32).powf(power);
//         }
//     }

//     return float_result;
// }

// // Return order is standard dev iamge first and mean image second
// pub fn local_statistics(
//     base: &Array2<f32>,
//     window_height: u32,
//     window_width: u32,
// ) -> (Array2<f32>, Array2<f32>) {
//     let (base_cols, base_rows) = base.dimensions();

//     let mut zero_pad_base = GrayImage::new(base_cols + window_width, base_rows + window_height);

//     image::imageops::overlay(
//         &mut zero_pad_base,
//         base,
//         ((window_width + 1) / 2) as i64,
//         ((window_height + 1) / 2) as i64,
//     );

//     let integral = integral_image(&zero_pad_base);

//     let base_squared = image_raised_power(&zero_pad_base, 2.0);
//     let integral_squared = integral_image_matrix(base_squared);

//     let mut mean_value_holder: Vec<Vec<f32>> =
//         vec![vec![0.0; base_cols as usize]; base_rows as usize];
//     let mut standard_dev_value_holder: Vec<Vec<f32>> =
//         vec![vec![0.0; base_cols as usize]; base_rows as usize];

//     let mut result_mean = GrayImage::new(base_cols, base_rows);
//     let mut result_standard_dev = GrayImage::new(base_cols, base_rows);

//     let mut num_of_elements_in_window;
//     let mut window_sum;
//     let mut squared_window_sum;

//     let mut window_mean;
//     let mut squared_window_mean;

//     let mut sigma;
//     let mut sigma_squared;

//     let mut mean_max = 0.0;
//     let mut standard_dev_max = 0.0;

//     for row in 0..base_rows {
//         for col in 0..base_cols {
//             num_of_elements_in_window = window_height * window_width;

//             if row < (window_height / 2) {
//                 num_of_elements_in_window -= (window_height / 2) + (row * window_height);

//                 if col < (window_width / 2) {
//                     num_of_elements_in_window -=
//                         (window_width / 2 - col) * (window_width - window_height / 2 - row);
//                 }

//                 if row > base_cols - window_height / 2 {
//                     num_of_elements_in_window -= (col + window_width / 2 - base_cols)
//                         * (window_width - window_height / 2 - row);
//                 }
//             } else if row > base_rows - window_height / 2 {
//                 num_of_elements_in_window -= (row + window_height / 2 - base_rows) * window_height;

//                 if col < window_width / 2 {
//                     num_of_elements_in_window -= (window_width / 2 - col)
//                         * (window_width - (row + window_height / 2 - base_rows));
//                 }

//                 if col > base_cols - window_height / 2 {
//                     num_of_elements_in_window -= (col + window_width / 2 - base_cols)
//                         * (window_width - (row + window_height / 2 - base_rows));
//                 }
//             } else {
//                 if col < window_width / 2 {
//                     num_of_elements_in_window -= (window_width / 2 - col) * window_width
//                 }

//                 if col > base_cols - window_width / 2 {
//                     num_of_elements_in_window -=
//                         (col + window_width / 2 - base_cols) * window_width;
//                 }
//             }

//             window_sum = integral[(row + window_height) as usize][(col + window_width) as usize]
//                 - integral[row as usize][(col + window_width) as usize]
//                 - integral[(row + window_height) as usize][col as usize]
//                 + integral[row as usize][col as usize];

//             squared_window_sum = integral_squared[(row + window_height) as usize]
//                 [(col + window_width) as usize]
//                 - integral_squared[row as usize][(col + window_width) as usize]
//                 - integral_squared[(row + window_height) as usize][col as usize]
//                 + integral_squared[row as usize][col as usize];

//             window_mean = window_sum / num_of_elements_in_window as f32;

//             squared_window_mean = squared_window_sum / num_of_elements_in_window as f32;

//             sigma_squared = squared_window_mean - window_mean.powf(2.0);

//             sigma = sigma_squared.sqrt();

//             if window_mean > mean_max {
//                 mean_max = window_mean;
//             }

//             if sigma > standard_dev_max {
//                 standard_dev_max = sigma;
//             }

//             mean_value_holder[row as usize][col as usize] = window_mean;
//             standard_dev_value_holder[row as usize][col as usize] = sigma;
//         }
//     }

//     mean_value_holder
//         .iter_mut()
//         .flat_map(|row| row.iter_mut())
//         .for_each(|item| *item = *item / mean_max * 255.0);

//     standard_dev_value_holder
//         .iter_mut()
//         .flat_map(|row| row.iter_mut())
//         .for_each(|item| *item = *item / standard_dev_max * 255.0);

//     for row in 0..base_rows {
//         for col in 0..base_cols {
//             let mean_pixel: image::Luma<u8> =
//                 image::Luma::<u8>([mean_value_holder[row as usize][col as usize] as u8]);
//             let standard_dev_pixel: image::Luma<u8> =
//                 image::Luma::<u8>([standard_dev_value_holder[row as usize][col as usize] as u8]);

//             result_mean.put_pixel(col, row, mean_pixel);
//             result_standard_dev.put_pixel(col, row, standard_dev_pixel)
//         }
//     }

//     return (result_standard_dev, result_mean);
// }

// pub fn subtract_images(
//     base: &Array2<f32>,
//     secondary: &Array2<f32>,
// ) -> Result<Array2<f32>, std::io::Error> {
//     let (base_cols, base_rows) = base.dimensions();
//     let (secondary_cols, secondary_rows) = secondary.dimensions();

//     println!(
//         "base_cols: {} base_rows: {} secondary_cols: {} secondary_rows: {}",
//         base_cols, base_rows, secondary_cols, secondary_rows
//     );

//     if base_cols >= secondary_cols && base_rows >= secondary_rows {
//         let mut value_holder: Vec<Vec<i32>> = vec![vec![0; base_cols as usize]; base_rows as usize];

//         let mut result = GrayImage::new(base_cols, secondary_cols);
//         let mut difference;
//         let mut max = 0;
//         let mut min = 1000;

//         for row in 0..secondary_rows {
//             for col in 0..secondary_cols {
//                 difference = *base.get_pixel(col, row).channels().get(0).unwrap() as i32
//                     - *secondary.get_pixel(col, row).channels().get(0).unwrap() as i32;

//                 if difference > max {
//                     max = difference;
//                 }

//                 if difference < min {
//                     min = difference;
//                 }

//                 value_holder[row as usize][col as usize] = difference;
//             }
//         }

//         if !(0..255).contains(&(max - min)) {
//             value_holder
//                 .iter_mut()
//                 .flat_map(|row| row.iter_mut())
//                 .for_each(|item| *item = (*item - min) / (max.abs() + min.abs()) * 255);
//         }

//         for row in 0..secondary_rows {
//             for col in 0..secondary_cols {
//                 let difference_pixel: image::Luma<u8> =
//                     image::Luma::<u8>([value_holder[row as usize][col as usize] as u8]);

//                 result.put_pixel(col, row, difference_pixel);
//             }
//         }

//         return Ok(result);
//     } else {
//         return Err(std::io::ErrorKind::InvalidInput.into());
//     }
// }
