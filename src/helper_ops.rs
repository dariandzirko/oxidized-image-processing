use ndarray::{Array1, Array2};

// Some random github solution from the ndarray developers that I do not yet understand
// let mut it = matrix.axis_iter_mut(Axis(0));

// ndarray::Zip::from(it.nth(perm.0).unwrap())
//     .and(it.nth(perm.1 - (perm.0 + 1)).unwrap())
//     .apply(std::mem::swap);

// row
//
// 1 2 3  c
// 4 5 6  o
// 7 8 9  l
//--- flip x ---
// 7 8 9
// 4 5 6
// 1 2 3

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

//These could just be the same type T instead of f32
pub fn vec_of_vec_into_array2(
    matrix: &mut Vec<Vec<f32>>,
    dimensions: (usize, usize),
) -> Array2<f32> {
    let temp: Array1<f32> = matrix
        .into_iter()
        .flat_map(|item| item.into_iter())
        .collect();

    temp.into_shape(dimensions).unwrap()
}

//TODO This return type is a little cursed
pub fn conv_2d(kernel: &Array2<f32>, base: &Array2<f32>, same_size: bool) -> Array2<f32> {
    //TODO I think my rows and columns are backwards
    //(width, height) everywhere
    let (base_cols, base_rows) = base.dimensions();
    let (kernel_cols, kernel_rows) = (kernel.dimensions.0 as u32, kernel.dimensions.1 as u32);

    let mut zero_pad_base = GrayImage::new(
        base_cols + 2 * (kernel_cols - 1),
        base_rows + 2 * (kernel_rows - 1),
    );

    image::imageops::overlay(
        &mut zero_pad_base,
        base,
        (kernel_cols - 1) as i64,
        (kernel_rows - 1) as i64,
    );

    //Swap the bounds and the zero_padded_elem commented lines to either get the "true"
    //convolution or the same size convolution.
    let result_cols = if same_size {
        base_cols
    } else {
        base_cols + kernel_cols - 1
    };

    let result_rows = if same_size {
        base_rows
    } else {
        base_rows + kernel_rows - 1
    };

    let mut min_value = 1000.0;
    let mut max_value = -1000.0;

    let mut result = GrayImage::new(result_cols, result_rows);

    //* The dimension does nothing as of now
    let flipped_kernel = kernel.flip(2);

    //This is a bad solution
    let kernel_sum = kernel.sum_kernel().round();
    let mut temp_solution_flag = false;
    if kernel_sum <= 0.0 {
        temp_solution_flag = true;
    }

    print!("kernel_sum: {}", kernel_sum);
    println!("temp_solution_flag: {}", temp_solution_flag);

    //* Right now the convolution returns an image the same size as the original image
    for row in 0..result_rows {
        for col in 0..result_cols {
            let mut sum = 0.0;
            //Going through the kernel math which only includes pixels in the kernel window
            //TODO include all pixel channels so this will work on RGB images
            for kernel_row in 0..kernel_rows {
                for kernel_col in 0..kernel_cols {
                    let flipped_kernel_elem =
                        flipped_kernel.matrix[kernel_row as usize][kernel_col as usize];
                    //*This has to be not the best way to do this

                    let zero_padded_elem: u8 = if same_size {
                        *zero_pad_base
                            .get_pixel(
                                col + kernel_col + kernel_cols / 2,
                                row + kernel_row + kernel_rows / 2,
                            )
                            .channels()
                            .get(0)
                            .unwrap()
                    } else {
                        *zero_pad_base
                            .get_pixel(col + kernel_col, row + kernel_row)
                            .channels()
                            .get(0)
                            .unwrap()
                    };

                    sum = sum + flipped_kernel_elem * zero_padded_elem as f32;
                }
            }

            // Scaling is hard
            if sum > max_value {
                max_value = sum;
            }
            if sum < min_value {
                min_value = sum;
            }
            // TODO Fix this. This is a horrible solution to negatives
            if temp_solution_flag {
                sum += 128.0;
            }

            sum = num::clamp(sum, 0.0, 255.0);
            let filtered_pixel: image::Luma<u8> = image::Luma::<u8>([sum as u8]);
            //let filtered_pixel = Pixel::from_channels(sum as u8, 0, 0, 0);
            //let test = Rgba::new(0,0,0,0);
            // let t = Pixel::from_channels(
            //     NumCast::from(clamp(t1, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t2, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t3, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t4, 0.0, max)).unwrap()
            // );
            result.put_pixel(col, row, filtered_pixel);
        }
    }

    // Okay so uint8 is just rounding all the negatives to 0, so attempting to scale negative
    // Numbers by a minimum convolution summation value (ie. -121) will just result in an image
    // with no real information because half of the values were set to 0 in the convolution calculation.
    // So there will be no "depth" or "content" most of that was already thrown out
    // For now I will just add by 128 above if therer is a negative.
    #[cfg(ignore)]
    {
        let mut bottom_offset = 0;
        let mut top_offset = 0;
        let mut offset_flag = false;
        // This is scuffed scaling. Will most likely be a source of a bunch of issues in the future
        if min_value < 0.0 {
            bottom_offset = min_value.abs() as u8;
            offset_flag = true;
        }

        if max_value > 256.0 {
            top_offset = (max_value - 256.0) as u8;
            offset_flag = true;
        }

        if offset_flag {
            println!("bottom_offset: {}", bottom_offset);
            println!("top_offset: {}", top_offset);
            for row in 0..result_rows {
                for col in 0..result_cols {
                    //println!("{:?}", result.get_pixel(col, row).channels().get(0));
                    let pixel = result.get_pixel(col, row).map(|v| v.wrapping_add(128));
                    result.put_pixel(col, row, pixel);
                }
            }
        }
    }
    // if max_value-min_value > 256.0 {
    //     unimplemented!("Need to do scaling thing here");
    // }

    return result;
}

//You typically will want to the raw integral image. In this case the value holder.
//So probably should return both. When creating the haar filter use value holder
//not result. Right now I will return the integral image
pub fn integral_image(base: &Array2<f32>) -> Vec<Vec<f32>> {
    let (base_cols, base_rows) = base.dimensions();

    let mut result = GrayImage::new(base_cols, base_rows);
    let mut value_holder: Vec<Vec<f32>> =
        vec![vec![0.0; (base_cols + 1) as usize]; (base_rows + 1) as usize];

    let mut max: f32 = 0.0;

    for row in 1..base_rows + 1 {
        for col in 1..base_cols + 1 {
            value_holder[row as usize][col as usize] =
                *base.get_pixel(col - 1, row - 1).channels().get(0).unwrap() as f32
                    + value_holder[row as usize][(col - 1) as usize]
                    + value_holder[(row - 1) as usize][col as usize]
                    - value_holder[(row - 1) as usize][(col - 1) as usize];

            if value_holder[row as usize][col as usize] > max {
                max = value_holder[row as usize][col as usize]
            }
        }
    }

    // Uncomment this if you want a normalize integral image not just a huge one
    // value_holder.iter_mut()
    // .flat_map(|row| row.iter_mut())
    // .for_each(|item| *item = *item/max*255.0);

    // for row in 0..base_rows {
    //     for col in 0..base_cols {
    //         let pixel: image::Luma::<u8> = image::Luma::<u8>([value_holder[row as usize][col as usize] as u8]);
    //         result.put_pixel(col, row, pixel);
    //     }
    // }

    return value_holder;
}

// This is a bad solution but whatever
pub fn integral_image_matrix(base: Array2<f32>) -> Array2<f32> {
    let base_rows = base.len();
    let base_cols = base[0].len();

    let mut value_holder: Vec<Vec<f32>> =
        vec![vec![0.0; (base_cols + 1) as usize]; (base_rows + 1) as usize];

    for row in 1..base_rows + 1 {
        for col in 1..base_cols + 1 {
            value_holder[row as usize][col as usize] = base[(row - 1) as usize][(col - 1) as usize]
                + value_holder[row as usize][(col - 1) as usize]
                + value_holder[(row - 1) as usize][col as usize]
                - value_holder[(row - 1) as usize][(col - 1) as usize];
        }
    }

    return value_holder;
}

pub fn haar_filter(base: &Array2<f32>, Mh: u32, Mv: u32) -> Array2<f32> {
    let (base_cols, base_rows) = base.dimensions();

    let offset_row = Mv / 2;
    let offset_col = Mh;

    let mut zero_pad_base = GrayImage::new(base_cols + 2 * offset_col, base_rows + 2 * offset_row);
    image::imageops::overlay(
        &mut zero_pad_base,
        base,
        offset_col as i64,
        offset_row as i64,
    );

    let mut value_holder: Vec<Vec<f32>> = vec![vec![0.0; base_cols as usize]; base_rows as usize];
    let mut result = GrayImage::new(base_cols, base_rows);

    let integral_zero_pad_base = integral_image(&zero_pad_base);

    let mut gray = 0.0;
    let mut white = 0.0;
    let mut result_value = 0.0;
    let mut max = 0.0;

    for row in 1..base_rows - 1 {
        for col in 0..base_cols {
            gray = integral_zero_pad_base[(row + Mv) as usize][(col + Mh) as usize]
                - integral_zero_pad_base[(row + Mv) as usize][(col) as usize]
                - integral_zero_pad_base[row as usize][(col + Mh) as usize]
                + integral_zero_pad_base[row as usize][col as usize];

            white = integral_zero_pad_base[(row + Mv) as usize][(col + 2 * Mh) as usize]
                - integral_zero_pad_base[(row + Mv) as usize][(col + Mh) as usize]
                - integral_zero_pad_base[row as usize][(col + 2 * Mh) as usize]
                + integral_zero_pad_base[row as usize][(col + Mh) as usize];

            result_value = white - gray;
            value_holder[(row - 1) as usize][col as usize] = result_value;

            if max < result_value {
                max = result_value;
            }
        }
    }

    // let result_pixel: image::Luma::<u8> = image::Luma::<u8>([result_value]);

    // result.put_pixel(col, row, result_pixel);

    value_holder
        .iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|item| *item = *item / max * 255.0 + 128.0);

    for row in 0..base_rows {
        for col in 0..base_cols {
            let pixel: image::Luma<u8> =
                image::Luma::<u8>([value_holder[row as usize][col as usize] as u8]);
            result.put_pixel(col, row, pixel);
        }
    }

    return result;
}

pub fn image_raised_power(base: &Array2<f32>, power: f32) -> Array2<f32> {
    let (base_cols, base_rows) = base.dimensions();

    let mut float_result = vec![vec![0.0; base_cols as usize]; base_rows as usize];

    for row in 0..base_rows {
        for col in 0..base_cols {
            float_result[row as usize][col as usize] =
                (*base.get_pixel(col, row).channels().get(0).unwrap() as f32).powf(power);
        }
    }

    return float_result;
}

// Return order is standard dev iamge first and mean image second
pub fn local_statistics(
    base: &Array2<f32>,
    window_height: u32,
    window_width: u32,
) -> (Array2<f32>, Array2<f32>) {
    let (base_cols, base_rows) = base.dimensions();

    let mut zero_pad_base = GrayImage::new(base_cols + window_width, base_rows + window_height);

    image::imageops::overlay(
        &mut zero_pad_base,
        base,
        ((window_width + 1) / 2) as i64,
        ((window_height + 1) / 2) as i64,
    );

    let integral = integral_image(&zero_pad_base);

    let base_squared = image_raised_power(&zero_pad_base, 2.0);
    let integral_squared = integral_image_matrix(base_squared);

    let mut mean_value_holder: Vec<Vec<f32>> =
        vec![vec![0.0; base_cols as usize]; base_rows as usize];
    let mut standard_dev_value_holder: Vec<Vec<f32>> =
        vec![vec![0.0; base_cols as usize]; base_rows as usize];

    let mut result_mean = GrayImage::new(base_cols, base_rows);
    let mut result_standard_dev = GrayImage::new(base_cols, base_rows);

    let mut num_of_elements_in_window;
    let mut window_sum;
    let mut squared_window_sum;

    let mut window_mean;
    let mut squared_window_mean;

    let mut sigma;
    let mut sigma_squared;

    let mut mean_max = 0.0;
    let mut standard_dev_max = 0.0;

    for row in 0..base_rows {
        for col in 0..base_cols {
            num_of_elements_in_window = window_height * window_width;

            if row < (window_height / 2) {
                num_of_elements_in_window -= (window_height / 2) + (row * window_height);

                if col < (window_width / 2) {
                    num_of_elements_in_window -=
                        (window_width / 2 - col) * (window_width - window_height / 2 - row);
                }

                if row > base_cols - window_height / 2 {
                    num_of_elements_in_window -= (col + window_width / 2 - base_cols)
                        * (window_width - window_height / 2 - row);
                }
            } else if row > base_rows - window_height / 2 {
                num_of_elements_in_window -= (row + window_height / 2 - base_rows) * window_height;

                if col < window_width / 2 {
                    num_of_elements_in_window -= (window_width / 2 - col)
                        * (window_width - (row + window_height / 2 - base_rows));
                }

                if col > base_cols - window_height / 2 {
                    num_of_elements_in_window -= (col + window_width / 2 - base_cols)
                        * (window_width - (row + window_height / 2 - base_rows));
                }
            } else {
                if col < window_width / 2 {
                    num_of_elements_in_window -= (window_width / 2 - col) * window_width
                }

                if col > base_cols - window_width / 2 {
                    num_of_elements_in_window -=
                        (col + window_width / 2 - base_cols) * window_width;
                }
            }

            window_sum = integral[(row + window_height) as usize][(col + window_width) as usize]
                - integral[row as usize][(col + window_width) as usize]
                - integral[(row + window_height) as usize][col as usize]
                + integral[row as usize][col as usize];

            squared_window_sum = integral_squared[(row + window_height) as usize]
                [(col + window_width) as usize]
                - integral_squared[row as usize][(col + window_width) as usize]
                - integral_squared[(row + window_height) as usize][col as usize]
                + integral_squared[row as usize][col as usize];

            window_mean = window_sum / num_of_elements_in_window as f32;

            squared_window_mean = squared_window_sum / num_of_elements_in_window as f32;

            sigma_squared = squared_window_mean - window_mean.powf(2.0);

            sigma = sigma_squared.sqrt();

            if window_mean > mean_max {
                mean_max = window_mean;
            }

            if sigma > standard_dev_max {
                standard_dev_max = sigma;
            }

            mean_value_holder[row as usize][col as usize] = window_mean;
            standard_dev_value_holder[row as usize][col as usize] = sigma;
        }
    }

    mean_value_holder
        .iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|item| *item = *item / mean_max * 255.0);

    standard_dev_value_holder
        .iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|item| *item = *item / standard_dev_max * 255.0);

    for row in 0..base_rows {
        for col in 0..base_cols {
            let mean_pixel: image::Luma<u8> =
                image::Luma::<u8>([mean_value_holder[row as usize][col as usize] as u8]);
            let standard_dev_pixel: image::Luma<u8> =
                image::Luma::<u8>([standard_dev_value_holder[row as usize][col as usize] as u8]);

            result_mean.put_pixel(col, row, mean_pixel);
            result_standard_dev.put_pixel(col, row, standard_dev_pixel)
        }
    }

    return (result_standard_dev, result_mean);
}

pub fn subtract_images(
    base: &Array2<f32>,
    secondary: &Array2<f32>,
) -> Result<Array2<f32>, std::io::Error> {
    let (base_cols, base_rows) = base.dimensions();
    let (secondary_cols, secondary_rows) = secondary.dimensions();

    println!(
        "base_cols: {} base_rows: {} secondary_cols: {} secondary_rows: {}",
        base_cols, base_rows, secondary_cols, secondary_rows
    );

    if base_cols >= secondary_cols && base_rows >= secondary_rows {
        let mut value_holder: Vec<Vec<i32>> = vec![vec![0; base_cols as usize]; base_rows as usize];

        let mut result = GrayImage::new(base_cols, secondary_cols);
        let mut difference;
        let mut max = 0;
        let mut min = 1000;

        for row in 0..secondary_rows {
            for col in 0..secondary_cols {
                difference = *base.get_pixel(col, row).channels().get(0).unwrap() as i32
                    - *secondary.get_pixel(col, row).channels().get(0).unwrap() as i32;

                if difference > max {
                    max = difference;
                }

                if difference < min {
                    min = difference;
                }

                value_holder[row as usize][col as usize] = difference;
            }
        }

        if !(0..255).contains(&(max - min)) {
            value_holder
                .iter_mut()
                .flat_map(|row| row.iter_mut())
                .for_each(|item| *item = (*item - min) / (max.abs() + min.abs()) * 255);
        }

        for row in 0..secondary_rows {
            for col in 0..secondary_cols {
                let difference_pixel: image::Luma<u8> =
                    image::Luma::<u8>([value_holder[row as usize][col as usize] as u8]);

                result.put_pixel(col, row, difference_pixel);
            }
        }

        return Ok(result);
    } else {
        return Err(std::io::ErrorKind::InvalidInput.into());
    }
}

#[cfg(test)]
mod test {
    use ndarray::array;

    use super::flip_across_x;
    use super::flip_across_y;

    #[test]
    fn flip_across_x_test() {
        let mut matrix = array![[1., 1., 1.], [2., 2., 2.], [3., 3., 3.]];
        flip_across_x(&mut matrix);
        let flipped_matrix = array![[3., 3., 3.], [2., 2., 2.], [1., 1., 1.]];
        assert_eq!(matrix, flipped_matrix);
    }

    #[test]
    fn flip_across_y_test() {
        let mut matrix = array![[1., 2., 3.], [1., 2., 3.], [1., 2., 3.]];
        flip_across_y(&mut matrix);
        let flipped_matrix = array![[3., 2., 1.], [3., 2., 1.], [3., 2., 1.]];
        assert_eq!(matrix, flipped_matrix);
    }

    #[test]
    fn flip_across_x_twice() {
        let mut matrix = array![[1., 1., 1.], [2., 2., 2.], [3., 3., 3.]];
        flip_across_x(&mut matrix);
        flip_across_x(&mut matrix);
        let flipped_matrix = array![[1., 1., 1.], [2., 2., 2.], [3., 3., 3.]];
        assert_eq!(matrix, flipped_matrix);
    }
}
