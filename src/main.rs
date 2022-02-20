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
    // let result_display = ImageView::new(ImageInfo::rgb8(1920, 1080), &result);

    // // Create a window with default options and display the image.
    // let window = create_window("result", Default::default()).unwrap();
    // window.set_image("result-001", result_display);

    result.save("result.png").unwrap();
}

//Going to start with assuming rectangular kernels
struct Kernel {
    //Maybe ndarray will be a lot faster here 
    //TODO look into ndarray 
    matrix: Vec<Vec<f32>>,
    // Create a window with default options and display the image.
    dimensions: (usize,usize), //row, col
}

impl Kernel {
    
    //How do I return nothing from this 
    fn new() -> Kernel {
        let matrix = vec![vec![0.0;2];2];
        // Should keep the dimensions as (width, height) to match the library
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
            let x = i as f32 - (lim);
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
            let x = row as f32  - (lim);
            for col in 0..length {
                let y = col as f32 - (lim);
                let val = E.powf( -( x.powf(2.0) + y.powf(2.0) ) / (2.0*radius.powf(2.0)) );
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
    //(width, height) everywhere
    let (base_cols, base_rows) = base.dimensions();
    let (kernel_cols, kernel_rows) = (kernel.dimensions.0 as u32, kernel.dimensions.1 as u32);

    let mut zero_pad_base = GrayImage::new(base_cols + 2*(kernel_cols-1), base_rows + 2*(kernel_rows-1) );

    image::imageops::overlay(&mut zero_pad_base, base, (kernel_cols-1) as i64, (kernel_rows-1) as i64);

    let result_cols = base_cols+kernel_cols-1;
    let result_rows = base_rows+kernel_rows-1; 
    // let result_cols = base_cols;
    // let result_rows = base_rows;    

    let mut result = GrayImage::new(result_cols, result_rows);

    //* The dimension does nothing as of now 
    let flipped_kernel = kernel.flip(2);

    for row in 0..result_rows{
        for col in 0..result_cols {
            let mut sum = 0.0;
            //Going through the kernel math which only includes pixels in the kernel window
            //TODO include all pixel channels so this will work on RGB images
            for kernel_row in 0..kernel_rows {
                for kernel_col in 0..kernel_cols {

                    let flipped_kernel_elem = flipped_kernel.matrix[kernel_row as usize][kernel_col as usize];
                    //*This has to be a fucking war crime
                    let zero_padded_elem  = *zero_pad_base.get_pixel(col+kernel_col, row+kernel_row).channels().get(0).unwrap();
                
                    sum = sum + flipped_kernel_elem*zero_padded_elem as f32;
                }
            }
            let filtered_pixel: image::Luma::<u8> = image::Luma::<u8>([(sum) as u8]);
            //let filtered_pixel = Pixel::from_channels(sum as u8, 0, 0, 0);
            //let test = Rgba::new(0,0,0,0);
            // let t = Pixel::from_channels(
            //     NumCast::from(clamp(t1, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t2, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t3, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t4, 0.0, max)).unwrap()
            // );
            result.put_pixel(col, row, filtered_pixel);
        }
    }

    return result;
}
