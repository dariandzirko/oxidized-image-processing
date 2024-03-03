use std::vec;

use ndarray::Array2;

use crate::helper_ops::conv_2d;
use crate::kernel::Kernel;

#[derive(Default, Clone)]
pub struct PixelGradientInfo {
    gx: f32,
    gy: f32,
    mag: f32,
    angle: f32,
}

impl PixelGradientInfo {
    pub fn new(gx: f32, gy: f32) -> PixelGradientInfo {
        let magnitude_gradient = f32::sqrt(f32::powf(gx, 2.0) + f32::powf(gy, 2.0));
        //Try scaling the mag, this is a lazy solution
        //magnitude_gradient = magnitude_gradient*255.0/361.0;

        let angle_gradient = (gy).atan2(gx);
        PixelGradientInfo {
            gx,
            gy,
            mag: magnitude_gradient,
            angle: angle_gradient,
        }
    }
}

pub struct EdgeLine {
    name: String, //This will be interpretation of the name of each direction of edges not the name of the normal of the direction
    adjacent1: (i32, i32), //first adjacent pixel in the direction of the edge normal
    adjacent2: (i32, i32), //second adjancent pixel in the direction of the edge normal
}

pub fn gradient_image_content(image: &Array2<f32>) -> Vec<PixelGradientInfo> {
    let der_x_dir = conv_2d(&mut Kernel::sobel_x_dir().matrix, &image, true);
    let der_y_dir = conv_2d(&mut Kernel::sobel_y_dir().matrix, &image, true);

    der_x_dir
        .indexed_iter()
        .map(|(index, gx)| PixelGradientInfo::new(*gx, *der_y_dir.get(index).unwrap()))
        .collect()
}

//There might be an easier match statement to use here/ Maybe some sort of enum based match statement. Damn I really
//don't know because it is matching ranges of angles
// pub fn normal_to_direction(angle: f32) -> EdgeLine {
//     match angle {
//         _direction
//             //-pi/8 .. pi/8
//             if (-std::f32::consts::FRAC_PI_8 ..= std::f32::consts::FRAC_PI_8)
//                 .contains(&angle) => EdgeLine{name: "vertical_edge".to_owned(), adjacent1: (1, 0), adjacent2: (-1, 0)},
//         _direction
//             //pi/8 .. 3pi/8
//             if (std::f32::consts::FRAC_PI_8 ..= std::f32::consts::FRAC_PI_8 * 3.0)
//                 .contains(&angle) => EdgeLine{name: "neg_45_edge".to_owned(), adjacent1: (1, 1), adjacent2: (-1, -1)},
//         _direction
//             //3pi/8 .. 5pi/8
//             if (std::f32::consts::FRAC_PI_8 * 3.0 ..= std::f32::consts::FRAC_PI_8 * 5.0)
//                 .contains(&angle) => EdgeLine{name: "horizontal_edge".to_owned(), adjacent1: (0, 1), adjacent2: (0, -1)},
//         _direction
//             //5pi/8 .. 7pi/8
//             if (std::f32::consts::FRAC_PI_8 * 5.0 ..= std::f32::consts::FRAC_PI_8 * 7.0)
//                 .contains(&angle) => EdgeLine{name: "pos_45_edge".to_owned(), adjacent1: (-1, 1), adjacent2: (1, -1)},

//         _direction
//             //7pi/8 .. -7pi/8
//             if (-std::f32::consts::FRAC_PI_8 * 7.0 ..= -std::f32::consts::FRAC_PI_8 * 7.0)
//                 .contains(&angle) => EdgeLine{name: "vertical_edge".to_owned(), adjacent1: (1, 0), adjacent2: (-1, 0)},
//         _direction
//             //-7pi/8 .. -5pi/8
//             if (-std::f32::consts::FRAC_PI_8 * 7.0 ..= -std::f32::consts::FRAC_PI_8 * 5.0)
//                 .contains(&angle) => EdgeLine{name: "neg_45_edge".to_owned(), adjacent1: (1, 1), adjacent2: (-1, -1)},
//         _direction
//             //-5pi/8 .. -3pi/8
//             if (-std::f32::consts::FRAC_PI_8 * 5.0 ..= -std::f32::consts::FRAC_PI_8 * 3.0)
//                 .contains(&angle) => EdgeLine{name: "horizontal_edge".to_owned(), adjacent1: (0, 1), adjacent2: (0, -1)},
//         _direction
//             //-3pi/8 .. -pi/8
//             if (-std::f32::consts::FRAC_PI_8 * 3.0 ..= -std::f32::consts::FRAC_PI_8)
//                 .contains(&angle) => EdgeLine{name: "pos_45_edge".to_owned(), adjacent1: (-1, 1), adjacent2: (1, -1)},

//         _ => EdgeLine{name: "broken".to_owned(), adjacent1: (100, 100), adjacent2: (-100, -100)},
//     }
// }

pub fn normal_to_direction(angle: f32) -> EdgeLine {
    //-pi/8 .. pi/8
    if (-std::f32::consts::FRAC_PI_8..=std::f32::consts::FRAC_PI_8).contains(&angle) {
        return EdgeLine {
            name: "vertical_edge".to_owned(),
            adjacent1: (1, 0),
            adjacent2: (-1, 0),
        };
    }
    //pi/8 .. 3pi/8
    if (std::f32::consts::FRAC_PI_8..=std::f32::consts::FRAC_PI_8 * 3.0).contains(&angle) {
        return EdgeLine {
            name: "neg_45_edge".to_owned(),
            adjacent1: (1, 1),
            adjacent2: (-1, -1),
        };
    }
    //3pi/8 .. 5pi/8
    if (std::f32::consts::FRAC_PI_8 * 3.0..=std::f32::consts::FRAC_PI_8 * 5.0).contains(&angle) {
        return EdgeLine {
            name: "horizontal_edge".to_owned(),
            adjacent1: (0, 1),
            adjacent2: (0, -1),
        };
    }
    //5pi/8 .. 7pi/8
    if (std::f32::consts::FRAC_PI_8 * 5.0..=std::f32::consts::FRAC_PI_8 * 7.0).contains(&angle) {
        return EdgeLine {
            name: "pos_45_edge".to_owned(),
            adjacent1: (-1, 1),
            adjacent2: (1, -1),
        };
    }

    // Second half

    //7pi/8 .. -7pi/8
    if (std::f32::consts::FRAC_PI_8 * 7.0..=std::f32::consts::PI).contains(&angle)
        || (-std::f32::consts::PI..=-std::f32::consts::FRAC_PI_8 * 7.0).contains(&angle)
    {
        return EdgeLine {
            name: "vertical_edge".to_owned(),
            adjacent1: (1, 0),
            adjacent2: (-1, 0),
        };
    }
    //-7pi/8 .. -5pi/8
    if (-std::f32::consts::FRAC_PI_8 * 7.0..=-std::f32::consts::FRAC_PI_8 * 5.0).contains(&angle) {
        return EdgeLine {
            name: "neg_45_edge".to_owned(),
            adjacent1: (1, 1),
            adjacent2: (-1, -1),
        };
    }
    //-5pi/8 .. -3pi/8
    if (-std::f32::consts::FRAC_PI_8 * 5.0..=-std::f32::consts::FRAC_PI_8 * 3.0).contains(&angle) {
        return EdgeLine {
            name: "horizontal_edge".to_owned(),
            adjacent1: (0, 1),
            adjacent2: (0, -1),
        };
    }
    //-3pi/8 .. -pi/8
    if (-std::f32::consts::FRAC_PI_8 * 3.0..=-std::f32::consts::FRAC_PI_8).contains(&angle) {
        return EdgeLine {
            name: "pos_45_edge".to_owned(),
            adjacent1: (-1, 1),
            adjacent2: (1, -1),
        };
    }

    return EdgeLine {
        name: "broken".to_owned(),
        adjacent1: (100, 100),
        adjacent2: (-100, -100),
    };
}

//This is certainly wrong
pub fn non_maxima_suppression(
    gradient_info: Vec<PixelGradientInfo>,
    rows: usize,
    cols: usize,
) -> (Array2<f32>, Array2<f32>) {
    let mut result = Array2::zeros((rows, cols));
    let mut mag_image = Array2::zeros((rows, cols));

    //I think this is correct, where I ignore the outermost border of pixels
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let pixel_info = &gradient_info[(col + (row * cols)) as usize];
            let normal_line = normal_to_direction(pixel_info.angle);

            let one_line_side = gradient_info[((col as i32 + normal_line.adjacent1.0)
                + (row as i32 + normal_line.adjacent1.1) * cols as i32)
                as usize]
                .mag;
            let other_line_side = gradient_info[((col as i32 + normal_line.adjacent2.0)
                + (row as i32 + normal_line.adjacent2.1) * cols as i32)
                as usize]
                .mag;

            if pixel_info.mag > one_line_side && pixel_info.mag > other_line_side {
                result[(row, col)] = pixel_info.mag;
            }

            mag_image[(row, col)] = pixel_info.mag;
        }
    }

    (result, mag_image)
}

pub fn double_threshhold(image: &Array2<f32>, low_thresh: f32, high_thresh: f32) -> Array2<f32> {
    let (rows, cols) = image.dim();
    let mut result = Array2::zeros((rows, cols));

    let strong_pixel = 255.0;
    let weak_pixel = 25.0;

    result.indexed_iter_mut().for_each(|(index, element)| {
        if image.get(index).unwrap() >= &high_thresh {
            *element = strong_pixel;
        } else if *image.get(index).unwrap() >= low_thresh {
            *element = weak_pixel;
        }
    });

    result
}

pub fn canny_edge_detector(image: &Array2<f32>) -> Array2<f32> {
    //This is incorrect atm because it does not return the same size image
    let smoothed_image = conv_2d(&mut Kernel::gaussian_2d(2.0).matrix, &image, true);

    let smoothed_gradient = gradient_image_content(&smoothed_image);
    let (rows, cols) = smoothed_image.dim();

    let (non_maxima_suppressed_image, mag_gradient_image) =
        non_maxima_suppression(smoothed_gradient, rows, cols);

    let double_threshed_image = double_threshhold(&non_maxima_suppressed_image, 15.0, 50.0);

    return double_threshed_image;
}
