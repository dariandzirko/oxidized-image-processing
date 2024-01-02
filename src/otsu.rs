use ndarray::Array2;

struct GrayHistogram {
    //Does usize or u32 matter here?
    histogram: [usize; 256],
    probabilities: [f32; 256],
}

fn make_grayhistrogram(image: &Array2<f32>) -> GrayHistogram {
    let mut histogram = [0; 256];
    let mut probabilities = [0.0; 256];

    let image_shape = image.raw_dim();
    let sum = (image_shape[0] * image_shape[1]) as f32;

    image.into_iter().for_each(|item| {
        histogram[*item as usize] += 1;
        probabilities[*item as usize] += 1.0;
    });

    //I feel like this is less readable
    GrayHistogram {
        histogram,
        probabilities: histogram
            .iter()
            .map(|x| *x as f32 / sum)
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap(),
    }
}

pub fn otsu_threshold(image: &Array2<f32>) -> f32 {
    let easy_histogram = make_grayhistrogram(image);

    //q1(k)
    let mut probabilities_class1 = [0.0; 256];

    //m1(k)
    let mut mean_intensities_class1 = [0.0; 256];

    for i in 0..easy_histogram.histogram.len() {
        let mut sum = 0.0;
        let mut sum_probabilities = 0.0;

        for j in 0..i {
            sum += easy_histogram.probabilities[j];
            sum_probabilities += sum * i as f32;
        }

        probabilities_class1[i] = sum;
        mean_intensities_class1[i] = sum_probabilities;
    }

    //mg
    let global_mean_intensity = *mean_intensities_class1.last().unwrap();

    //sigmab^2
    let mut between_class_var = [0.0; 256];

    for i in 0..easy_histogram.histogram.len() {
        between_class_var[i] =
            (global_mean_intensity * probabilities_class1[i] - mean_intensities_class1[i]).powf(2.0)
                / ((probabilities_class1[i]) * (1.0 - probabilities_class1[i]))
    }

    let max = between_class_var.iter().fold(f32::MIN, |a, &b| a.max(b));
    return max;
}

pub fn otsu(image: &Array2<f32>) -> Array2<f32> {
    let image_shape = image.raw_dim();

    let mut result = Array2::<f32>::zeros(image_shape);

    let otsu_threshold = otsu_threshold(image);

    let dark_pixel = 0.0;
    let light_pixel = 255.0;

    image.indexed_iter().for_each(|(index, item)| {
        if *item > otsu_threshold {
            result[index] = light_pixel;
        } else {
            result[index] = dark_pixel;
        }
    });

    result
}
