use std::f32::consts::E;
use image::{GenericImageView, DynamicImage, RgbImage, Rgb, ImageBuffer, Luma, GrayImage, Pixel, Rgba};
use show_image::{ImageView, ImageInfo, create_window};

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("Boy.tiff").unwrap().to_luma8();
    let mut gauss2d_filt = Kernel::new();
    gauss2d_filt.gaussian_2d(1.8);
    
    let result= conv_2d(&gauss2d_filt, &img);
    let result_display = ImageView::new(ImageInfo::rgb8(1920, 1080), &result);

    // Create a window with default options and display the image.
    let window = create_window("result", Default::default()).unwrap();
    window.set_image("result-001", result_display);

    result.save("result.png").unwrap();
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
    fn new() -> Kernel {
        let matrix = vec![vec![0.0;2];2];
        let dimensions = (1,1);    
        Kernel {matrix, dimensions}
    }

    fn gaussian_1d(&mut self, radius: f32) {
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

        self.matrix = matrix;
        self.dimensions = dimensions;
    }

    fn gaussian_2d(&mut self, radius: f32) {
        let lim = (3.0*radius).floor() as f32;
        let length = (2.0*lim+1.0) as usize;

        let mut matrix = vec![vec![0.0;length];length];
        let dimensions = (length,length);

        let mut sum = 0.0;

        for row in 0..length {
            let x = row as f32  - (lim + 1.0);
            for col in 0..length {
                let y = col as f32 - (lim + 1.0);
                let val = E.powf( -( x.powf(2.0) + y.powf(2.0))/ (2.0*radius.powf(2.0)) );
                matrix[row][col] = val;
                sum+=val;
            }   
        }

        matrix.iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item=*item/sum);

        self.matrix = matrix;
        self.dimensions = dimensions;
    }   

    // Dimensions is only one of 2 things because kernels are 2d
    //TODO flip should probably just flip in 1 dimension with an option to flip in both. So like dimension = 0 would be like along the 
    //TODO x or columns, dimension = 1 would be along y or rows and dimension = 2 would be both or some shit. Or even make a 2d flip idk
    fn flip(& self, dimension: usize) -> Kernel {

        let mut mat = self.matrix.clone();
        let dims = self.dimensions;

        mat.iter_mut().for_each(|row| row.reverse());
        if dimension > 0 {
            mat.reverse();
        }

        //TODO implement some sort of copy or clone trait or some shit idk 
        Kernel {matrix: mat, dimensions: dims}
    }
}

//TODO This return type is a little cursed 
fn conv_2d(kernel: &Kernel, base: &GrayImage) -> GrayImage{

    //TODO I think my rows and columns are backwards

    let (base_cols, base_rows) = base.dimensions();
    let (kernel_cols, kernel_rows) = (kernel.dimensions.0 as u32, kernel.dimensions.1 as u32);

    let mut zero_pad_base = GrayImage::new(base_cols + 2*(kernel_cols-1), base_rows + 2*(kernel_rows-1) );

    image::imageops::overlay(&mut zero_pad_base, base, kernel_cols as i64, kernel_rows as i64);

    let result_rows = base_rows+kernel_rows-1; 
    let result_cols = base_cols+kernel_cols-1;

    let mut result = GrayImage::new(result_cols, result_rows);

    //* The dimension does nothing as of now 
    let flipped_kernel = kernel.flip(2);

    println!("Base dims {:} {:}", base_rows, base_cols);
    println!("Zero_pad dims {:} {:}", base_rows + 2*(kernel_rows-1),base_cols + 2*(kernel_cols-1));
    println!("Result dims {:} {:}", result_rows, result_cols);
    println!("Kernel dims {:} {:}", kernel_rows, kernel_cols);
    println!("Loop dims {:} {:}", result_rows - 1 + kernel_rows - 1, result_cols - 1 + kernel_cols - 1);

    // Base dims 768 512
    // Zero_pad dims 788 532
    // Result dims 778 522
    // Kernel dims 11 11
    // Loop dims 787 531
    for row in 0..result_rows {
        for col in 0..result_cols {
            let mut sum = 0.0;

            //So going through the kernel math which only includes pixels in the kernel window
            for kernel_row in 0..kernel_rows {
                for kernel_col in 0..kernel_cols {
                    // Gna comment about what elements are being accessed when.
                    // So for the flipped kernel its easy, just use k to access
                    // the row element and l for the col element, resulting in
                    // something that looks like this 
                    // flipped_kernel(current_kernel_row, current_kernel_col)
                    // Then we have the element of the zero padded base, which
                    // should be 
                    // zero_pad_base(current_result_row+current_kernel_row-1,current_result_col+current_kernel_col-1)

                    let flipped_kernel_elem = flipped_kernel.matrix[kernel_row as usize][kernel_col as usize];
                    //*This has to be a fucking war crime
                    let zero_padded_elem  = *zero_pad_base.get_pixel(row+kernel_row,col+kernel_col).channels().get(0).unwrap();

                    //println("{?}", zero_padded_elem);   

                    sum = sum + flipped_kernel_elem*zero_padded_elem as f32;
                }
            }
            //have to normalize sum to 255 maybe but prob not because filter is normalized to 1
            
            //TODO create a fucking pixel
            let filtered_pixel: image::Luma::<u8> = image::Luma::<u8>([sum as u8]);
            //let filtered_pixel = Pixel::from_channels(sum as u8, 0, 0, 0);
            //let test = Rgba::new(0,0,0,0);
            // let t = Pixel::from_channels(
            //     NumCast::from(clamp(t1, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t2, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t3, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t4, 0.0, max)).unwrap()
            // );
            result.put_pixel(row, col, filtered_pixel);
        }
    }

    return result;
}
