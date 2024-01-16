use std::vec;

use image::{self, GenericImageView, GrayImage, Luma, Pixel};
use oxidized_image_processing::{conv_2d, Kernel};

#[derive(Default, Clone)]
pub struct PixelGradientInfo {
    gx: u8,
    gy: u8,
    mag: f32,
    angle: f32
}

impl PixelGradientInfo {
    pub fn new(gx: u8, gy: u8) -> PixelGradientInfo{

        let mut magnitude_gradient =
        f32::sqrt((f32::powf(gx as f32, 2.0) + f32::powf(gy as f32, 2.0)));
        //Try scaling the mag, this is a lazy solution
        //magnitude_gradient = magnitude_gradient*255.0/361.0;

        let angle_gradient = (gy as f32).atan2(gx as f32);
        PixelGradientInfo { gx: gx, gy: gy, mag: magnitude_gradient, angle: angle_gradient }    
    }
}

pub struct EdgeLine {
    name: String, //This will be interpretation of the name of each direction of edges not the name of the normal of the direction
    adjacent1: (i32, i32), //first adjacent pixel in the direction of the edge normal
    adjacent2: (i32, i32) //second adjancent pixel in the direction of the edge normal
}

//Bias here will be promising points that are slightly better than just plain edges
pub fn finder_mark_location(bias: Vec<(usize, usize)>, image: GrayImage) -> (usize, usize) {
    return (0, 0);
}

pub fn gradient_image_content(image: &GrayImage) -> Vec<PixelGradientInfo> {
    let der_x_dir = conv_2d(&Kernel::sobel_x_dir(), image, true);
    let der_y_dir = conv_2d(&Kernel::sobel_y_dir(), image, true);

    // der_x_dir.save("der_x_dir.png");
    // der_y_dir.save("der_y_dir.png");

    //Using different names because my values are wrong rn
    let (cols, rows) = der_x_dir.dimensions();

    let mut result = vec![PixelGradientInfo{..Default::default()}; (cols * rows) as usize];

    for row in 0..rows {
        for col in 0..cols {
            let gx = *der_x_dir
                .get_pixel(col, row)
                .channels()
                .get(0)
                .unwrap();
            let gy = *der_y_dir
                .get_pixel(col, row)
                .channels()
                .get(0)
                .unwrap();
            //println!("col: {col} y: {row} width: {cols} index: {}",((row*cols)+col));

            let index = (row*cols)+col;
            result[index as usize] = PixelGradientInfo::new(gx, gy);
        }
    }

    return result;
}

//There might be an easier match statement to use here/ Maybe some sort of enum based match statement. Damn I really
//don't know because it is matching ranges of angles
pub fn normal_to_direction(angle: f32) -> EdgeLine {
    match angle {
        direction
            //-pi/8 .. pi/8
            if (-std::f32::consts::FRAC_PI_8 ..= std::f32::consts::FRAC_PI_8)
                .contains(&angle) => EdgeLine{name: "vertical_edge".to_owned(), adjacent1: (1, 0), adjacent2: (-1, 0)},
        direction
            //pi/8 .. 3pi/8
            if (std::f32::consts::FRAC_PI_8 ..= std::f32::consts::FRAC_PI_8 * 3.0)
                .contains(&angle) => EdgeLine{name: "neg_45_edge".to_owned(), adjacent1: (1, 1), adjacent2: (-1, -1)},
        direction 
            //3pi/8 .. 5pi/8
            if (std::f32::consts::FRAC_PI_8 * 3.0 ..= std::f32::consts::FRAC_PI_8 * 5.0)
                .contains(&angle) => EdgeLine{name: "horizontal_edge".to_owned(), adjacent1: (0, 1), adjacent2: (0, -1)},
        direction 
            //5pi/8 .. 7pi/8
            if (std::f32::consts::FRAC_PI_8 * 5.0 ..= std::f32::consts::FRAC_PI_8 * 7.0)
                .contains(&angle) => EdgeLine{name: "pos_45_edge".to_owned(), adjacent1: (-1, 1), adjacent2: (1, -1)},

        direction
            //7pi/8 .. -7pi/8
            if (-std::f32::consts::FRAC_PI_8 * 7.0 ..= -std::f32::consts::FRAC_PI_8 * 7.0)
                .contains(&angle) => EdgeLine{name: "vertical_edge".to_owned(), adjacent1: (1, 0), adjacent2: (-1, 0)},
        direction
            //-7pi/8 .. -5pi/8
            if (-std::f32::consts::FRAC_PI_8 * 7.0 ..= -std::f32::consts::FRAC_PI_8 * 5.0)
                .contains(&angle) => EdgeLine{name: "neg_45_edge".to_owned(), adjacent1: (1, 1), adjacent2: (-1, -1)},
        direction 
            //-5pi/8 .. -3pi/8
            if (-std::f32::consts::FRAC_PI_8 * 5.0 ..= -std::f32::consts::FRAC_PI_8 * 3.0)
                .contains(&angle) => EdgeLine{name: "horizontal_edge".to_owned(), adjacent1: (0, 1), adjacent2: (0, -1)},
        direction 
            //-3pi/8 .. -pi/8
            if (-std::f32::consts::FRAC_PI_8 * 3.0 ..= -std::f32::consts::FRAC_PI_8)
                .contains(&angle) => EdgeLine{name: "pos_45_edge".to_owned(), adjacent1: (-1, 1), adjacent2: (1, -1)},
                
        _ => EdgeLine{name: "broken".to_owned(), adjacent1: (100, 100), adjacent2: (-100, -100)},
    }
}

//This is certainly wrong 
pub fn non_maxima_suppression(gradient_info: Vec<PixelGradientInfo>, cols: u32, rows: u32) -> (GrayImage, GrayImage){
    let mut result = GrayImage::new(cols, rows);
    let mut mag_image = GrayImage::new(cols, rows);

    let mut max = 0;

    //I think this is correct, where I ignore the outermost border of pixels
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            let pixel_info = &gradient_info[(col+(row*cols)) as usize];
            let normal_line = normal_to_direction(pixel_info.angle);

            //println!("row: {}| col: {} | mag: {}| angle: {}| adjacent1: {:?}| adjacent2: {:?}| name: {}", row, col, pixel_info.mag, pixel_info.angle, normal_line.adjacent1, normal_line.adjacent2, normal_line.name);

            if pixel_info.mag > gradient_info[ ((col as i32 + normal_line.adjacent1.0) + (row as i32 + normal_line.adjacent1.1)*cols as i32) as usize].mag
            && pixel_info.mag > gradient_info[ ((col as i32 + normal_line.adjacent2.0) + (row as i32 + normal_line.adjacent2.1)*cols as i32) as usize].mag {
                let pixel = Luma([pixel_info.mag as u8]);
                result.put_pixel(col as u32, row as u32, pixel);
            }
            let pixel = Luma([pixel_info.mag as u8]);
            mag_image.put_pixel(col as u32, row as u32, pixel);
        }
    }

    return (result, mag_image);
}
pub fn double_threshhold(image: &GrayImage, low_thresh: u8, high_thresh: u8) -> GrayImage {
    let (cols, rows) = image.dimensions();
    let mut result = GrayImage::new(cols, rows);

    let strong_pixel = Luma([255]);
    let weak_pixel = Luma([25]);        

    for row in 0..rows {
        for col in 0..cols {
            
            let pixel_value = image.get_pixel(col, row).channels().get(0).unwrap();

            if *pixel_value >= high_thresh {
                result.put_pixel(col, row, strong_pixel);
            }
            else if *pixel_value >= low_thresh {
                result.put_pixel(col, row, weak_pixel);
            }
        }
    }

    result
}

pub fn canny_edge_detector(image: &GrayImage) -> GrayImage {
    //This is incorrect atm because it does not return the same size image
    let smoothed_image = conv_2d(&Kernel::gaussian_2d(4.0), image, true);
    smoothed_image.save("smoothed_image.png");

    let smoothed_gradient = gradient_image_content(&smoothed_image);
    let (cols, rows) = smoothed_image.dimensions();

    let (non_maxima_suppressed_image, mag_gradient_image) = non_maxima_suppression(smoothed_gradient, cols, rows);
    mag_gradient_image.save("mag_gradient_image.png");
    non_maxima_suppressed_image.save("non_maxima_img.png");
    
    let double_threshed_image = double_threshhold(&non_maxima_suppressed_image, 25, 75);

    return double_threshed_image;
}

//This will take the result of the above. Maybe will return the vector of biased points that I can use to find the locator marks
pub fn box_detector(image: &GrayImage) -> Vec<(usize, usize)> {
    return vec![(0, 0)];
}
