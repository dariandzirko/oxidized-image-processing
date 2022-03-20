use oxidized_image_processing::*;
use core::panic;
use std::{f32::consts::E, io::BufRead, env::temp_dir, num::IntErrorKind, vec, collections::btree_set::Difference};
use image::{GenericImageView, DynamicImage, RgbImage, Rgb, ImageBuffer, Luma, GrayImage, Pixel, Rgba};
use num::{self, pow, Float};

fn base() -> GrayImage{
    image::open("tests/truth/Boy.tiff").unwrap().to_luma8()
}

#[test]
fn test_blur_1r8(){
    let mut pixel_sum: u64 = 0;

    let base = base();
    let gauss2d_filt = Kernel::gaussian_2d(1.8); 
    let gauss2d_conv_result = conv_2d(&gauss2d_filt, &base);

    //TODO Remove unwraps
    let r1_8_truth = image::open("tests/truth/Gauss_Blur/gauss2d_boy_r1.8.png").unwrap().to_luma8();

    let mut difference = subtract_images(&gauss2d_conv_result, &r1_8_truth).unwrap();
    difference.save("tests/truth/Gauss_Blur/Test_Results/test_diff_r1.8_gauss2d_boy.png").unwrap();

    difference.iter_mut()
    .for_each(|pix| pixel_sum+=*pix as u64);

    assert_eq!(pixel_sum,0);
}
