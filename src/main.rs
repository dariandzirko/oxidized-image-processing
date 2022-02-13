use std::f32::consts::E;
use image::{GenericImageView, DynamicImage, RgbImage, Rgb, ImageBuffer};

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("Boy.tif").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();
}

//Going to start with assuming rectangular kernels
struct Kernel {
    //Maybe ndarray will be a lot faster here 
    //TODO look into ndarray 
    matrix: Vec<Vec<f32>>,
    dimensions: (usize,usize), //row, col
}

impl Kernel {
    
    //How do I return nothing from this 
    // fn new(&self) -> Kernel {
    // }
    fn gaussian_1d(radius: f32) -> Kernel {
        let lim = 3.0*(radius.floor() as f32);
        let length = (2.0*lim+1.0) as usize;

        let mut matrix = vec![vec![0.0;length];1];
        let dimensions = (1,length);    

        let mut sum = 0.0;

        for i in 0..length+1 {
            let x = i as f32 - (lim + 1.0);
            let val = E.powf( -( x.powf(2.0))/ (2.0*radius.powf(2.0)) );
            matrix[0][i] = val;
            sum+=val;
        }

        matrix.iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item=*item/sum);

        return Kernel {matrix, dimensions}
    }

    fn gaussian_2d(radius: f32) -> Kernel {
        let lim = 3.0*(radius.floor() as f32);
        let length = (2.0*lim+1.0) as usize;

        let mut matrix = vec![vec![0.0;length];length];
        let dimensions = (length,length);

        let mut sum = 0.0;

        for row in 0..length+1 {
            let x = row as f32  - (lim + 1.0);
            for col in 0..length+1 {
                let y = col as f32 - (lim + 1.0);
                let val = E.powf( -( x.powf(2.0) + y.powf(2.0))/ (2.0*radius.powf(2.0)) );
                matrix[row][col] = val;
                sum+=val;
            }   
        }

        matrix.iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item=*item/sum);

        return Kernel {matrix, dimensions}
    }   

    // Dimensions is only one of 2 things because kernels are 2d
    //TODO flip should probably just flip in 1 dimension with an option to flip in both. So like dimension = 0 would be like along the 
    //TODO x or columns, dimension = 1 would be along y or rows and dimension = 2 would be both or some shit. Or even make a 2d flip idk
    fn flip(&mut self, dimension: usize) -> Kernel {

        let mut mat = self.matrix.clone();
        let dims = self.dimensions;

        mat.iter_mut().for_each(|mut row| row.reverse());
        if dimension > 0 {
            mat.reverse();
        }

        //TODO implement some sort of copy or clone trait or some shit idk 
        Kernel {matrix: mat, dimensions: dims}
    }
}

//TODO This return type is a little cursed 
fn conv_2d(kernel: &mut Kernel, base: RgbImage) -> ImageBuffer<Rgb<u8>, Vec<u8>>{

    let (base_rows, base_cols) = base.dimensions();
    let (kernel_rows, kernel_cols) = (kernel.dimensions.0 as u32, kernel.dimensions.1 as u32);

    let mut zero_pad_base = RgbImage::new(base_cols + 2*(kernel_cols-1), base_rows + 2*(kernel_rows-1) );

    image::imageops::overlay(&mut zero_pad_base, &base, kernel_cols, kernel_rows);

    let result_rows = base_rows+kernel_rows-1; 
    let result_cols = base_cols+kernel_cols-1;

    let result = RgbImage::new(result_cols, result_rows);

    //TODO Fuck
    let flipped_kernel = kernel.flip(2);
    
    for i in 0..result_rows {
        for j in 1..result_cols {
            let sum = 0;
            for k in 0..kernel_rows {
                for l in 0..kernel_cols {
                    // Gna comment about what elements are being accessed when.
                    // So for the flipped kernel its easy, just use k to access
                    // the row element and l for the col element, resulting in
                    // something that looks like this 
                    // flipped_kernel(current_kernel_row, current_kernel_col)
                    // Then we have the element of the zero padded base, which
                    // should be 
                    // zero_pad_base(current_result_row+current_kernel_row-1,current_result_col+current_kernel_col-1)
                    
                    let flipped_kernel_elem = flipped_kernel.matrix[k][l];
                    let zero_padded_elem = zero_pad_base[i+k-1][j+l-1];
                    sum = sum + flipped_kernel_elem*zero_padded_elem;
                }
            }
        }
    }
    
    return result;
}
