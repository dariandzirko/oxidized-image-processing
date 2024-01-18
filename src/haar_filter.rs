pub struct Offset_and_Sign {
    pub offset_coords: (usize, usize),

    pub sign: i32,
}

impl Offset_and_Sign {
    pub fn new(x_offset: usize, y_offset: usize, sign: i32) -> Offset_and_Sign {
        Offset_and_Sign {
            offset_coords: (x_offset, y_offset).sign,
        }
    }
}

pub struct Haar_Filter {
    // The idea here is that in my haar function I will be iterating over this list and applying the values
    // The specifics are gna be like its gna be sum of all of -> Sign(base_image_index + offset_coords)
    // The idae of the sign is supposed to indicate "Black or White" recatangles, where "White" in this case is + and "Black" is minus
    // I will probably make the array in order from top to bottom and left to right but technically it will not have to be with my desired solution
    pub corner_descriptors: Array1<Offset_and_Sign>,

    // These are for the dimension offsets for the zero pad base
    offset_x: usize,
    offset_y: usize,
}

impl Haar_Filter {
    //Assume left side is black, I suppose all of these function can tag a flag to flip the colors
    //Mh has be to even so that both rectangles can be the same width
    pub fn two_rectangle_horizontal(Mv: usize, Mh: usize) -> Haar_Filter {
        // gray = integral_zero_pad_base[(index.0 + Mh, index.1 + Mv)]
        //     - integral_zero_pad_base[(index.0, index.1 + Mv)]
        //     - integral_zero_pad_base[(index.0 + Mh, index.1)]
        //     + integral_zero_pad_base[(index.0, index.1)];

        // white = integral_zero_pad_base[(index.0 + 2 * Mh, index.1 + Mv)]
        //     - integral_zero_pad_base[(index.0 + Mh, index.1 + Mv)]
        //     - integral_zero_pad_base[(index.0 + 2 * Mh, index.1)]
        //     + integral_zero_pad_base[(index.0 + Mh, index.1)];

        //        *item = white - gray;

        let array = array![
            Offset_and_Sign::new(Mh, Mv, -1),
            Offset_and_Sign::new(0, Mv, 1),
            Offset_and_Sign::new(Mh, 0, 1),
            Offset_and_Sign::new(0, 0, -1),
            //Other rectangle time
            Offset_and_Sign::new(2 * Mh, Mv, 1),
            Offset_and_Sign::new(Mh, Mv, -1),
            Offset_and_Sign::new(2 * Mh, 0, -1),
            Offset_and_Sign::new(Mh, 0, 1)
        ];

        Haar_Filter {
            corner_descriptors: array,
            offset_x: Mh,
            offsets: Mv / 2,
        }
    }

    //Assume top part is black (-)
    //Mv has be to even so that both rectangles can be the same height
    pub fn two_rectangle_vertical(Mv: usize, Mh: usize) -> Haar_Filter {
        let array = array![
            Offset_and_Sign::new(Mh, Mv, -1),
            Offset_and_Sign::new(0, Mv, 1),
            Offset_and_Sign::new(Mh, 0, 1),
            Offset_and_Sign::new(0, 0, -1),
            //Other rectangle time
            Offset_and_Sign::new(Mh, 2 * Mv, 1),
            Offset_and_Sign::new(0, 2 * Mv, -1),
            Offset_and_Sign::new(Mh, Mv, -1),
            Offset_and_Sign::new(0, Mv, 1)
        ];

        Haar_Filter {
            corner_descriptors: array,
            offset_x: Mh,
            offsets: Mv / 2,
        }
    }

    //Assume left side is black
    //Mh is divisible by 3 as each rectangle with have the same width
    //Mv has be to even so that both rectangles can be the same height
    pub fn three_rectangle_horiztonal(Mv: usize, Mh: usize) -> Haar_Filter {
        let array = array![
            Offset_and_Sign::new(Mh, Mv, -1),
            Offset_and_Sign::new(0, Mv, 1),
            Offset_and_Sign::new(Mh, 0, 1),
            Offset_and_Sign::new(0, 0, -1),
            //2nd rectangle time
            Offset_and_Sign::new(2 * Mh, Mv, 1),
            Offset_and_Sign::new(Mh, Mv, -1),
            Offset_and_Sign::new(2 * Mh, 0, -1),
            Offset_and_Sign::new(Mh, 0, 1)
            //3rd rectangle time
            Offset_and_Sign::new(3 * Mh, Mv, -1),
            Offset_and_Sign::new(2* Mh, Mv, 1),
            Offset_and_Sign::new(3 * Mh, 0, 1),
            Offset_and_Sign::new(2* Mh, 0, -1)
        ];

        Haar_Filter {
            corner_descriptors: array,
            offset_x: Mh,
            offsets: Mv / 2,
        }
    }

    //Assume the order will be row0: white, black, row1: black white
    // Mv and Mh are even so they can be divided by 2 so all rectangles are the same shape
    pub fn four_rectangle(Mv: usize, Mh: usize) -> Haar_Filter {}
}

pub fn apply_haar_filter(base: &Array2<f32>, haar_filter: Haar_Filter) -> Array2<f32> {
    let base_shape = base.raw_dim();
    let offset_y = Mv / 2;
    let offset_x = Mh;

    let zero_pad_base = zero_pad(
        &base,
        offset_x,
        offset_y,
        base_shape[0] + 2 * offset_x,
        base_shape[1] + 2 * offset_y,
    );

    let mut result = Array2::<f32>::zeros(base_shape);

    let integral_zero_pad_base = integral_image(&zero_pad_base);

    let mut gray = 0.0;
    let mut white = 0.0;

    result.indexed_iter_mut().for_each(|(index, item)| {
        gray = integral_zero_pad_base[(index.0 + Mh, index.1 + Mv)]
            - integral_zero_pad_base[(index.0, index.1 + Mv)]
            - integral_zero_pad_base[(index.0 + Mh, index.1)]
            + integral_zero_pad_base[(index.0, index.1)];

        white = integral_zero_pad_base[(index.0 + 2 * Mh, index.1 + Mv)]
            - integral_zero_pad_base[(index.0 + Mh, index.1 + Mv)]
            - integral_zero_pad_base[(index.0 + 2 * Mh, index.1)]
            + integral_zero_pad_base[(index.0 + Mh, index.1)];

        *item = white - gray;
    });

    return result;
}
