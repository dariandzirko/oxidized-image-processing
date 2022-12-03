use core::panic;
use image::{
    DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage, Rgba,
};
use num::{self, pow, Float};
use std::{
    collections::btree_set::Difference, env::temp_dir, f32::consts::E, io::BufRead,
    num::IntErrorKind, vec,
};

//Going to start with assuming rectangular kernels
pub struct Kernel {
    //Maybe ndarray will be a lot faster here
    //TODO look into ndarray
    matrix: Vec<Vec<f32>>,
    // Create a window with default options and display the image.
    dimensions: (usize, usize), //row, col
}

impl Kernel {
    //How do I return nothing from this
    pub fn new() -> Kernel {
        let matrix = vec![];
        // Should keep the dimensions as (width, height) to match the library
        let dimensions = (0, 0);
        Kernel { matrix, dimensions }
    }

    pub fn print_kernel(&self) {
        let (columns, rows) = self.dimensions;

        println!();
        for row in 0..rows {
            for col in 0..columns {
                print!("{} | ", self.matrix[row][col]);
            }
            println!();
        }
    }

    pub fn sum_kernel(&self) -> f32 {
        let mut sum = 0.0;

        self.matrix
            .iter()
            .flat_map(|row| row.iter())
            .for_each(|item| sum += *item);

        return sum;
    }

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

        dummy_filt.matrix = matrix;
        dummy_filt.dimensions = dimensions;
    }

    pub fn gaussian_2d(radius: f32) -> Kernel {
        let mut dummy_filt = Kernel::new();

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

        dummy_filt.matrix = matrix;
        dummy_filt.dimensions = dimensions;

        return dummy_filt;
    }

    pub fn highpass_2d(radius: f32) -> Kernel {
        let mut dummy_filt = Kernel::gaussian_2d(radius);

        let lim = (dummy_filt.dimensions.0 - 1) / 2;

        dummy_filt
            .matrix
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item = -(*item));

        dummy_filt.matrix[lim][lim] += 1.0;

        return dummy_filt;
    }

    pub fn sharpening_2d(radius: f32, beta: f32) -> Kernel {
        let mut dummy_filt = Kernel::highpass_2d(radius);

        let lim = (dummy_filt.dimensions.0 - 1) / 2;

        dummy_filt
            .matrix
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item /= beta);

        dummy_filt.matrix[lim][lim] += 1.0;

        return dummy_filt;
    }

    pub fn sobel_x_dir() -> Kernel {
        let mut dummy_filt = Kernel::new();

        dummy_filt.matrix = vec![
            vec![-1.0, -2.0, -1.0],
            vec![0.0, 0.0, 0.0],
            vec![1.0, 2.0, 1.0],
        ];

        dummy_filt.dimensions = (3, 3);

        return dummy_filt;
    }

    pub fn sobel_y_dir() -> Kernel {
        let mut dummy_filt = Kernel::new();

        dummy_filt.matrix = vec![
            vec![-1.0, 0.0, 1.0],
            vec![-2.0, 0.0, 2.0],
            vec![-1.0, 0.0, 1.0],
        ];

        dummy_filt.dimensions = (3, 3);

        return dummy_filt;
    }

    // Dimensions is only one of 2 things because kernels are 2d
    //TODO flip should probably just flip in 1 dimension with an option to flip in both. So like dimension = 0 would be like along the
    //TODO x or columns, dimension = 1 would be along y or rows and dimension = 2 would be both or some shit. Or even make a 2d flip idk
    pub fn flip(&self, dimension: usize) -> Kernel {
        let mut mat = self.matrix.clone();
        let dims = self.dimensions;

        mat.iter_mut().for_each(|row| row.reverse());
        if dimension > 0 {
            mat.reverse();
        }

        //TODO implement some sort of copy or clone trait or some shit idk
        Kernel {
            matrix: mat,
            dimensions: dims,
        }
    }
}

//TODO This return type is a little cursed
pub fn conv_2d(kernel: &Kernel, base: &GrayImage, same_size: bool) -> GrayImage {
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
                    //*This has to be a fucking war crime

                    let zero_padded_elem = if same_size {
                        *zero_pad_base
                            .get_pixel(
                                col + kernel_col + kernel_cols / 2,
                                row + kernel_row + kernel_rows / 2,
                            )
                            .channels()
                            .get(0)
                            .unwrap();
                    } else {
                        *zero_pad_base
                            .get_pixel(col + kernel_col, row + kernel_row)
                            .channels()
                            .get(0)
                            .unwrap();
                    };

                    sum = sum + flipped_kernel_elem * zero_padded_elem as f32;
                }
            }

            // Scaling is fucking cursed
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

    // This is a bunch of whore code written by a whore
    //
    // Okay so uint8 is just rounding all the fucking negatives to 0, so attempting to scale negative
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
pub fn integral_image(base: &GrayImage) -> Vec<Vec<f32>> {
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

// This is a bad solution but fuck em for the time being
pub fn integral_image_matrix(base: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
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

pub fn haar_filter(base: &GrayImage, Mh: u32, Mv: u32) -> GrayImage {
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

pub fn image_raised_power(base: &GrayImage, power: f32) -> Vec<Vec<f32>> {
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
    base: &GrayImage,
    window_height: u32,
    window_width: u32,
) -> (GrayImage, GrayImage) {
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
    base: &GrayImage,
    secondary: &GrayImage,
) -> Result<GrayImage, std::io::Error> {
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
