use std::result;

use ndarray::{Array1, Array2, Dimension};

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

//Need to work on the same size functionality
pub fn conv_2d(kernel: &mut Array2<f32>, base: &Array2<f32>, same_size: bool) -> Array2<f32> {
    //(width, height) or (cols, rows) everywhere

    //BUUUUUUUUT I don't know if it is row col so this will get me the correct shape but I don't know what the order of the dimensions is
    let base_shape = base.raw_dim();
    let kernel_shape = kernel.raw_dim();

    let zero_pad_base = zero_pad(
        base,
        kernel_shape[1] - 1,
        kernel_shape[0] - 1,
        base_shape[0] + 2 * (kernel_shape[0] - 1),
        base_shape[1] + 2 * (kernel_shape[1] - 1),
    );

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
        kernel
            .indexed_iter()
            .for_each(|(kernel_index, kernel_item)| {
                *item += zero_pad_base
                    .get((index.0 + kernel_index.0, index.1 + kernel_index.1))
                    .unwrap()
                    * kernel_item;
            });
    });

    if same_size {
        let result2 = result
            .indexed_iter()
            .filter(|(index, _item)| {
                !(index.0 < (kernel_shape[0] - 1) / 2
                    || index.0 > base_shape[0] + (kernel_shape[0] - 1) / 2
                    || index.1 < (kernel_shape[1] - 1) / 2
                    || index.1 > base_shape[1] + (kernel_shape[0] - 1) / 2)
            })
            .map(|(_, item)| *item)
            .collect::<Array1<f32>>();
        println!("result2.raw_dim(): {:?}", result2.raw_dim());
        // .into_shape(base_shape)
        // .unwrap();
    }

    result
}

pub fn integral_image(base: &Array2<f32>) -> Array2<f32> {
    let base_shape = base.raw_dim();

    let mut result = Array2::<f32>::zeros((base_shape[0] + 1, base_shape[1] + 1));

    base.indexed_iter().for_each(|(index, _item)| {
        result[[index.0 + 1, index.1 + 1]] = base.get((index.0, index.1)).unwrap()
            + result.get((index.0, index.1 + 1)).unwrap()
            + result.get((index.0 + 1, index.1)).unwrap()
            - result.get((index.0, index.1)).unwrap()
    });

    return result;
}

pub fn image_raised_power(base: &Array2<f32>, power: f32) -> Array2<f32> {
    let mut result = Array2::<f32>::zeros(base.raw_dim());

    base.indexed_iter().for_each(|(index, item)| {
        result[index] = item.powf(power);
    });

    return result;
}

// Just zero pads then places the new matrix ontop of the zero pad
pub fn zero_pad(
    base: &Array2<f32>,
    offset_x: usize,
    offset_y: usize,
    new_width: usize,
    new_height: usize,
) -> Array2<f32> {
    let mut zero_pad_base = Array2::<f32>::zeros((new_width, new_height));

    base.indexed_iter().for_each(|(index, item)| {
        zero_pad_base[(index.0 + offset_x, index.1 + offset_y)] = *item;
    });

    zero_pad_base
}

// // Return order is standard dev image first and mean image second
pub fn local_statistics(
    base: &Array2<f32>,
    window_height: usize,
    window_width: usize,
) -> (Array2<f32>, Array2<f32>) {
    let base_shape = base.raw_dim();

    let zero_pad_base = zero_pad(
        base,
        (window_width + 1) / 2,
        (window_height + 1) / 2,
        base_shape[0] + window_width,
        base_shape[1] + window_height,
    );

    let integral = integral_image(&zero_pad_base);

    let base_squared = image_raised_power(&zero_pad_base, 2.0);
    let integral_squared = integral_image(&base_squared);

    let mut result_mean = Array2::<f32>::zeros(base_shape);
    let mut result_standard_dev = Array2::<f32>::zeros(base_shape);

    let mut num_of_elements_in_window = window_height * window_width;
    let mut window_sum = 0.0;
    let mut squared_window_sum = 0.0;

    let mut window_mean = 0.0;
    let mut squared_window_mean = 0.0;

    let mut sigma = 0.0;
    let mut sigma_squared = 0.0;

    //I deleted the cases dealing with boundaries of window size, because all of the math is based on the zero pad elements?
    //That might be an oopsie
    (0..base_shape[0]).into_iter().for_each(|col| {
        (0..base_shape[1]).into_iter().for_each(|row| {
            //This looks a little gross and is not immediately intuitive
            //There has to be a better way of handling the edge cases here
            //This also isn't probably all that accurate for the right most and bottom most edge cases
            //but that is a small amount of rows and cols that probably aren't all that important

            num_of_elements_in_window = window_height * window_width;
            if row < (window_height / 2) {
                num_of_elements_in_window -= (window_height / 2) + (row * window_height);

                if col < (window_width / 2) {
                    num_of_elements_in_window -=
                        (window_width / 2 - col) * (window_width - window_height / 2 - row);
                }

                if col > base_shape[0] - window_height / 2 {
                    num_of_elements_in_window -= (col + window_width / 2 - base_shape[0])
                        * (window_width - window_height / 2 - row);
                }
            } else if row > base_shape[1] - window_height / 2 {
                num_of_elements_in_window -=
                    (row + window_height / 2 - base_shape[1]) * window_height;

                if col < window_width / 2 {
                    num_of_elements_in_window -= (window_width / 2 - col)
                        * (window_width - (row + window_height / 2 - base_shape[1]));
                }

                if col > base_shape[0] - window_height / 2 {
                    num_of_elements_in_window -= (col + window_width / 2 - base_shape[0])
                        * (window_width - (row + window_height / 2 - base_shape[1]));
                }
            } else {
                if col < window_width / 2 {
                    num_of_elements_in_window -= (window_width / 2 - col) * window_width
                }

                if col > base_shape[0] - window_width / 2 {
                    num_of_elements_in_window -=
                        (col + window_width / 2 - base_shape[0]) * window_width;
                }
            }

            window_sum = integral[(col + window_width, row + window_height)]
                - integral[(col + window_width, row)]
                - integral[(col, row + window_height)]
                + integral[(col, row)];

            squared_window_sum = integral_squared[(col + window_width, row + window_height)]
                - integral_squared[(col + window_width, row)]
                - integral_squared[(col, row + window_height)]
                + integral_squared[(col, row)];

            window_mean = window_sum / num_of_elements_in_window as f32;

            squared_window_mean = squared_window_sum / num_of_elements_in_window as f32;

            sigma_squared = squared_window_mean - window_mean.powf(2.0);

            sigma = sigma_squared.sqrt();

            result_mean[(col, row)] = window_mean;
            result_standard_dev[(col, row)] = sigma;
        })
    });

    return (result_standard_dev, result_mean);
}

//Might want to put more thought and effort into the size of the result. Right now it just returns the size of the first image
pub fn subtract_images(base: &Array2<f32>, secondary: &Array2<f32>) -> Array2<f32> {
    let base_shape = base.raw_dim();

    //for the time being I am just going to return the base image size
    let mut result = Array2::<f32>::zeros(base_shape);

    base.indexed_iter().for_each(|(index, item)| {
        // println!("secondary[index]: {}", secondary[index]);
        // println!("item: {}", item);

        result[index] = item - secondary[index];
        // println!("result[index]: {}", result[index]);
    });

    return result;
}
