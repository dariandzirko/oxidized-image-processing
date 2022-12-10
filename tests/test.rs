use core::panic;
use image::{
    DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage, Rgba,
};
use num::{self, pow, Float};
use oxidized_image_processing::*;
use std::{
    collections::btree_set::Difference, env::temp_dir, f32::consts::E, io::BufRead,
    num::IntErrorKind, vec,
};

fn base() -> GrayImage {
    image::open("tests/truth/Boy.tiff").unwrap().to_luma8()
}

// Assumes that the result of the subtraction should be 0 and
fn subtract_and_compare(function_result: &GrayImage, truth: &GrayImage) -> u64 {
    let mut pixel_sum: u64 = 0;

    let mut difference = subtract_images(function_result, truth).unwrap();

    difference
        .iter_mut()
        .for_each(|pix| pixel_sum += *pix as u64);

    println!("{}", pixel_sum);
    return pixel_sum;
}

#[test]
fn test_subtraction() {
    let test1 = image::open("tests/truth/Gauss_Blur/gauss2d_boy_r1.2.png")
        .unwrap()
        .to_luma8();
    let test2 = image::open("tests/truth/Gauss_Blur/gauss2d_boy_r1.2.png")
        .unwrap()
        .to_luma8();

    assert_eq!(subtract_and_compare(&test1, &test2), 0);
}

#[test]
fn test_blur_1r2() {
    let base = base();
    let gauss2d_filt = Kernel::gaussian_2d(1.2);
    let gauss2d_conv_result = conv_2d(&gauss2d_filt, &base);

    //TODO Remove unwraps
    let r1_2_truth = image::open("tests/truth/Gauss_Blur/gauss2d_boy_r1.2.png")
        .unwrap()
        .to_luma8();
    assert_eq!(subtract_and_compare(&gauss2d_conv_result, &r1_2_truth), 0);
}

#[test]
fn test_blur_1r8() {
    let base = base();
    let gauss2d_filt = Kernel::gaussian_2d(1.8);
    let gauss2d_conv_result = conv_2d(&gauss2d_filt, &base);

    //TODO Remove unwraps
    let r1_8_truth = image::open("tests/truth/Gauss_Blur/gauss2d_boy_r1.8.png")
        .unwrap()
        .to_luma8();

    assert_eq!(subtract_and_compare(&gauss2d_conv_result, &r1_8_truth), 0);
}

#[test]
fn test_blur_2r5() {
    let base = base();
    let gauss2d_filt = Kernel::gaussian_2d(2.5);
    let gauss2d_conv_result = conv_2d(&gauss2d_filt, &base);

    //TODO Remove unwraps
    let r2_5_truth = image::open("tests/truth/Gauss_Blur/gauss2d_boy_r2.5.png")
        .unwrap()
        .to_luma8();

    assert_eq!(subtract_and_compare(&gauss2d_conv_result, &r2_5_truth), 0);
}

#[test]
fn test_blur_4r0() {
    let base = base();
    let gauss2d_filt = Kernel::gaussian_2d(4.0);
    let gauss2d_conv_result = conv_2d(&gauss2d_filt, &base);

    //TODO Remove unwraps
    let r4_0_truth = image::open("tests/truth/Gauss_Blur/gauss2d_boy_r4.0.png")
        .unwrap()
        .to_luma8();

    assert_eq!(subtract_and_compare(&gauss2d_conv_result, &r4_0_truth), 0);
}

#[test]
fn test_highpass_1r2() {
    let base = base();
    let highpass2d_filt = Kernel::highpass_2d(1.2);
    let highpass2d_conv_result = conv_2d(&highpass2d_filt, &base);

    //TODO Remove unwraps
    let r1_2_truth = image::open("tests/truth/Highpass/highpass2d_boy_r1.2.png")
        .unwrap()
        .to_luma8();

    assert_eq!(
        subtract_and_compare(&highpass2d_conv_result, &r1_2_truth),
        0
    );
}

#[test]
fn test_highpass_1r8() {
    let base = base();
    let highpass2d_filt = Kernel::highpass_2d(1.8);
    let highpass2d_conv_result = conv_2d(&highpass2d_filt, &base);

    //TODO Remove unwraps
    let r1_8_truth = image::open("tests/truth/Highpass/highpass2d_boy_r1.8.png")
        .unwrap()
        .to_luma8();

    assert_eq!(
        subtract_and_compare(&highpass2d_conv_result, &r1_8_truth),
        0
    );
}

#[test]
fn test_highpass_2r5() {
    let base = base();
    let highpass2d_filt = Kernel::highpass_2d(2.5);
    let highpass2d_conv_result = conv_2d(&highpass2d_filt, &base);

    //TODO Remove unwraps
    let r2_5_truth = image::open("tests/truth/Highpass/highpass2d_boy_r2.5.png")
        .unwrap()
        .to_luma8();

    assert_eq!(
        subtract_and_compare(&highpass2d_conv_result, &r2_5_truth),
        0
    );
}

#[test]
fn test_highpass_4r0() {
    let base = base();
    let highpass2d_filt = Kernel::highpass_2d(4.0);
    let highpass2d_conv_result = conv_2d(&highpass2d_filt, &base);

    //TODO Remove unwraps
    let r4_0_truth = image::open("tests/truth/Highpass/highpass2d_boy_r4.0.png")
        .unwrap()
        .to_luma8();

    assert_eq!(
        subtract_and_compare(&highpass2d_conv_result, &r4_0_truth),
        0
    );
}

#[test]
fn test_sharpen_1r8_4b0() {
    let base = base();
    let sharpen2d_filt = Kernel::sharpening_2d(1.8, 4.0);
    let sharpen2d_conv_result = conv_2d(&sharpen2d_filt, &base);

    //TODO Remove unwraps
    let r1_8_b4_0_truth = image::open("tests/truth/Sharpen/sharpen2d_boy_r1.8_b4.0.png")
        .unwrap()
        .to_luma8();

    assert_eq!(
        subtract_and_compare(&sharpen2d_conv_result, &r1_8_b4_0_truth),
        0
    );
}

#[test]
fn test_sharpen_1r8_8b0() {
    let base = base();
    let sharpen2d_filt = Kernel::sharpening_2d(1.8, 8.0);
    let sharpen2d_conv_result = conv_2d(&sharpen2d_filt, &base);

    //TODO Remove unwraps
    let r1_8_b8_0_truth = image::open("tests/truth/Sharpen/sharpen2d_boy_r1.8_b8.0.png")
        .unwrap()
        .to_luma8();

    assert_eq!(
        subtract_and_compare(&sharpen2d_conv_result, &r1_8_b8_0_truth),
        0
    );
}

#[test]
fn test_sharpen_2r5_4b0() {
    let base = base();
    let sharpen2d_filt = Kernel::sharpening_2d(2.5, 4.0);
    let sharpen2d_conv_result = conv_2d(&sharpen2d_filt, &base);

    //TODO Remove unwraps
    let r2_5_b4_0_truth = image::open("tests/truth/Sharpen/sharpen2d_boy_r2.5_b4.0.png")
        .unwrap()
        .to_luma8();

    assert_eq!(
        subtract_and_compare(&sharpen2d_conv_result, &r2_5_b4_0_truth),
        0
    );
}

#[test]
fn test_sharpen_2r5_8b0() {
    let base = base();
    let sharpen2d_filt = Kernel::sharpening_2d(2.5, 8.0);
    let sharpen2d_conv_result = conv_2d(&sharpen2d_filt, &base);

    //TODO Remove unwraps
    let r2_5_b8_0_truth = image::open("tests/truth/Sharpen/sharpen2d_boy_r2.5_b8.0.png")
        .unwrap()
        .to_luma8();

    assert_eq!(
        subtract_and_compare(&sharpen2d_conv_result, &r2_5_b8_0_truth),
        0
    );
}
