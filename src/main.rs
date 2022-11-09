mod compression;
use compression::jpeg::*;
use image::GenericImageView;

fn main() {
    let file = "./rust.jpeg";
    // let byte_stream: Vec<u8> = get_file_as_byte_vec(&file.to_string());

    let dynamic_image = image::open(file).unwrap();
    println!("Color: {:?}",  dynamic_image.as_bytes());
    // let (w_number, h_number) = dynamic_image.dimensions();
    // let binary_stream = dynamic_image.as_bytes().to_vec();

    // let vec_ycbcr = read_vec8_to_color(&binary_stream);
    // println!("W: {}, H: {}", w_number, h_number);
    // println!("Length: {}", binary_stream.len());
    // println!("Vec ycbcr: {:?}", vec_ycbcr.len());
    // let vec_block_iamge = split_image_block(vec_ycbcr, w_number, h_number);



    // println!("{:?}", a);
    // let plane_vec: Vec<Vec<Block>> = Vec::new();
    // println!("{}", Default::default());
    // let mut bitplanes: Vec<Vec<Block>> = vec![vec![Default::default(); 1024]; 3];
    // println!("Bit Plances: {:?}", bitplanes);
    // println!("Bit Lenght: {:?}", bitplanes[0][0]);
    //    let a =  split_image_block(vec_ycbcr, w_number, h_number);
    //    println!("A: {:?}", a);
    // let mut new_arr = Vec::new();
    // for i in 0..81 {
    //     new_arr.push(vec_ycbcr[i]);
    // }
    // let w_temp = 9;
    // let h_temp = 9;
    // let mut img_buff = vec_ycbcr.chunks(w_number as usize);
    // let mut b = Vec::new();
    // while let Some(arr) = img_buff.next() {
    //     b.push(arr)
    // }
    // // println!("Tempo: {:?}", b);
    // let block_size = 4;
  
    // // let number_of_block_vertical = h_number / block_size;
    // // let number_of_block_horizontal = w_number / block_size;
    // let number_of_block_vertical = if h_number % block_size == 0 {
    //     h_number / block_size
    // } else {
    //     (h_number / block_size) + 1
    // };

    // let number_of_block_horizontal = if w_number % block_size == 0 {
    //     w_number / block_size
    // } else {
    //     (w_number / block_size) + 1
    // };
    // println!("NUmber vertical: {}", number_of_block_vertical);
    // println!("Number horizontal: {}", number_of_block_horizontal);
    // // for h in 0..h_number{
    // //     for w in 0..w_number {
    // //     }
    // // }
    // // println!("{:?}", vec_ycbcr);
    // // for i in &b {
    // //     println!("{:?}", i);
    // //     println!("");
    // // }
    // let mut row = 0;
    // let mut column = 0;
    // let mut result = Vec::new();
    // while row < number_of_block_vertical * block_size {
    //     column = 0;
    //     while column < number_of_block_horizontal * block_size {
    //         let row_end = row + block_size;
    //         let column_end = column + block_size;

    //         // if row_end > w_temp {
    //         //     row_end = w_temp;
    //         // }

    //         // if column_end > w_temp {
    //         //     column_end = w_temp
    //         // }
    //         println!("Row: {} - Row_End:{}", row, row_end);
    //         println!("COlumn: {} - COlumn_End:{}", column, column_end);
    //         println!();

    //         let mut block_scan = Vec::new();
    //         for x in row..row_end {
    //             for y in column..column_end {
    //                 if x >= h_number || y >= w_number {
    //                     block_scan.push(Color::default());
    //                 } else {
    //                     block_scan.push(b[x as usize][y as usize]);
    //                 }
    //             }
    //         }

    //         result.push(block_scan);
    //         column += block_size;
    //     }

    //     row += block_size;
    // }
    // // let split_factor = result.chunks(4);

    // let mut refactor = Vec::new();

    // for i in result.iter() {
    //     let mut split_factor = i.chunks(4);
    //     let mut arr_new_factor = Vec::new();
    //     while let Some(arr) = split_factor.next() {
    //         arr_new_factor.push(arr)
    //     }
    //     refactor.push(arr_new_factor);
    // }

    // for i in refactor {
        // println!("{:?}", i);
        // println!("");
    // }

}
