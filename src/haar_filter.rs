use ndarray::{array, Array1, Array2, Dim};

pub struct OffsetAndSign {
    pub offset_coords: (usize, usize),

    pub sign: i32,
}

impl OffsetAndSign {
    pub fn new(x_offset: usize, y_offset: usize, sign: i32) -> OffsetAndSign {
        OffsetAndSign {
            offset_coords: (x_offset, y_offset),
            sign,
        }
    }
}

pub struct HaarFilter {
    // The idea here is that in my haar function I will be iterating over this list and applying the values
    // The specifics are gna be like its gna be sum of all of -> Sign(base_image_index + offset_coords)
    // The idae of the sign is supposed to indicate "Black or White" recatangles, where "White" in this case is + and "Black" is minus
    // I will probably make the array in order from top to bottom and left to right but technically it will not have to be with my desired solution
    pub corner_descriptors: Array1<OffsetAndSign>,

    // These are for the dimension offsets for the zero pad base
    pub offset_x: usize,
    pub offset_y: usize,
}

impl HaarFilter {
    //Assume left side is black, I suppose all of these function can tag a flag to flip the colors
    //Mh has be to even so that both rectangles can be the same width
    pub fn two_rectangle_horizontal(mv: usize, mh: usize) -> HaarFilter {
        let array = array![
            OffsetAndSign::new(mh, mv, -1),
            OffsetAndSign::new(0, mv, 1),
            OffsetAndSign::new(mh, 0, 1),
            OffsetAndSign::new(0, 0, -1),
            //Other rectangle time
            OffsetAndSign::new(2 * mh, mv, 1),
            OffsetAndSign::new(mh, mv, -1),
            OffsetAndSign::new(2 * mh, 0, -1),
            OffsetAndSign::new(mh, 0, 1)
        ];

        HaarFilter {
            corner_descriptors: array,
            offset_x: mh,
            offset_y: mv / 2,
        }
    }

    //Assume top part is black (-)
    //Mv has be to even so that both rectangles can be the same height
    pub fn two_rectangle_vertical(mv: usize, mh: usize) -> HaarFilter {
        let array = array![
            OffsetAndSign::new(mh, mv, -1),
            OffsetAndSign::new(0, mv, 1),
            OffsetAndSign::new(mh, 0, 1),
            OffsetAndSign::new(0, 0, -1),
            //Other rectangle time
            OffsetAndSign::new(mh, 2 * mv, 1),
            OffsetAndSign::new(0, 2 * mv, -1),
            OffsetAndSign::new(mh, mv, -1),
            OffsetAndSign::new(0, mv, 1)
        ];

        HaarFilter {
            corner_descriptors: array,
            offset_x: mh / 2,
            offset_y: mv,
        }
    }

    //Assume left side is black
    //Mv has be to even so that both rectangles can be the same height
    //The offsets here are going to be hard
    pub fn three_rectangle_horiztonal(mv: usize, mh: usize) -> HaarFilter {
        let array = array![
            OffsetAndSign::new(mh, mv, -1),
            OffsetAndSign::new(0, mv, 1),
            OffsetAndSign::new(mh, 0, 1),
            OffsetAndSign::new(0, 0, -1),
            //2nd rectangle time
            OffsetAndSign::new(2 * mh, mv, 1),
            OffsetAndSign::new(mh, mv, -1),
            OffsetAndSign::new(2 * mh, 0, -1),
            OffsetAndSign::new(mh, 0, 1),
            //3rd rectangle time
            OffsetAndSign::new(3 * mh, mv, -1),
            OffsetAndSign::new(2 * mh, mv, 1),
            OffsetAndSign::new(3 * mh, 0, 1),
            OffsetAndSign::new(2 * mh, 0, -1)
        ];

        HaarFilter {
            corner_descriptors: array,
            //This is wrong but close idea
            offset_x: mh * 3 / 2,
            offset_y: mv / 2,
        }
    }

    //Assume the order will be row0: white, black, row1: black white
    // Mv and Mh are even so they can be divided by 2 so all rectangles are the same shape
    pub fn four_rectangle(mv: usize, mh: usize) -> HaarFilter {
        let array = array![
            OffsetAndSign::new(mh, mv, -1),
            OffsetAndSign::new(0, mv, 1),
            OffsetAndSign::new(mh, 0, 1),
            OffsetAndSign::new(0, 0, -1),
            //Top right rectangle time
            OffsetAndSign::new(2 * mh, mv, 1),
            OffsetAndSign::new(mh, mv, -1),
            OffsetAndSign::new(2 * mh, 0, -1),
            OffsetAndSign::new(mh, 0, 1),
            //Bottom left rectangle time
            OffsetAndSign::new(mh, 2 * mv, 1),
            OffsetAndSign::new(0, 2 * mv, -1),
            OffsetAndSign::new(mh, mv, -1),
            OffsetAndSign::new(0, mv, 1),
            //Bottom right rectangle time
            OffsetAndSign::new(2 * mh, 2 * mv, -1),
            OffsetAndSign::new(mh, 2 * mv, 1),
            OffsetAndSign::new(2 * mh, mv, 1),
            OffsetAndSign::new(mh, mv, -1)
        ];

        HaarFilter {
            corner_descriptors: array,
            offset_x: mh,
            offset_y: mv,
        }
    }
}

//This should take an integral image as a parameter
pub fn apply_haar_filter(
    base_shape: Dim<[usize; 2]>,
    haar_filter: HaarFilter,
    integral_zero_pad_base: &Array2<f32>,
) -> Array2<f32> {
    Array2::<f32>::zeros(base_shape)
        .indexed_iter_mut()
        .map(|(index, item)| {
            haar_filter
                .corner_descriptors
                .iter()
                .fold(*item, |mut acc, corner_descriptor| {
                    acc += corner_descriptor.sign as f32
                        * integral_zero_pad_base[(
                            index.0 + corner_descriptor.offset_coords.0,
                            index.1 + corner_descriptor.offset_coords.1,
                        )];
                    acc
                })
        })
        .collect::<Array1<f32>>()
        .into_shape(base_shape)
        .unwrap()
}
