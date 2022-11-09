#![allow(non_snake_case)]

pub mod jpeg {
    #[derive(Debug, Clone, Copy)]
    pub struct Color {
        pub Y: f32,
        pub Cb: f32,
        pub Cr: f32,
    }
    impl Color {
        pub fn YCbCr(Y: f32, Cb: f32, Cr: f32) -> Color {
            return Color { Y, Cb, Cr };
        }
    }

    impl Default for Color {
        fn default() -> Self {
            Color {
                Y: f32::default(),
                Cb: f32::default(),
                Cr: f32::default(),
            }
        }
    }

    pub type Block = [[f32; 8]; 8];
    pub const NUMBER_BLOCK: u32 = 8;
    pub fn read_vec8_to_color(stream_vec: &Vec<u8>) -> Vec<Color> {
        let mut color_vec = Vec::new();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut cnt = 0;

        for c in stream_vec {
            match cnt {
                0 => {
                    r = *c;
                }
                1 => {
                    g = *c;
                }
                2 => {
                    b = *c;
                    // convert RGB to YCbCr
                    let y: f32 = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
                    let cb: f32 = -0.168 * r as f32 - 0.331 * g as f32 + 0.499 * b as f32;
                    let cr: f32 = 0.5 * r as f32 - 0.419 * g as f32 - 0.081 * b as f32;
                    let myColor = Color::YCbCr(y, cb, cr);
                    color_vec.push(myColor);
                }
                _ => {
                    panic!("Parse error in Color");
                }
            }
            cnt = (cnt + 1) % 3;
        }
        color_vec
    }

    pub fn split_image_block(img_vec: Vec<Color>, w: u32, h: u32) -> Vec<Vec<[Color; 8]>> {
        let mut img_buff = img_vec.chunks(w as usize);
        let mut image_split = Vec::new();

        while let Some(arr) = img_buff.next() {
            image_split.push(arr);
        }

        let number_of_block_vertical = if h % NUMBER_BLOCK == 0 {
            h / NUMBER_BLOCK
        } else {
            (h / NUMBER_BLOCK) + 1
        };

        let number_of_block_horizontal = if w % NUMBER_BLOCK == 0 {
            w / NUMBER_BLOCK
        } else {
            (w / NUMBER_BLOCK) + 1
        };

        let mut row = 0;
        let mut column = 0;
        let mut result = Vec::new();
        while row < number_of_block_vertical * NUMBER_BLOCK {
            column = 0;
            while column < number_of_block_horizontal * NUMBER_BLOCK {
                let row_end = row + NUMBER_BLOCK;
                let column_end = column + NUMBER_BLOCK;
                // println!("Row: {} - Row_End:{}", row, row_end);
                // println!("COlumn: {} - COlumn_End:{}", column, column_end);
                // println!();
                let mut block_scan = Vec::new();
                for x in row..row_end {
                    for y in column..column_end {
                        if x >= h || y >= w {
                            block_scan.push(Color::default());
                        } else {
                            block_scan.push(image_split[x as usize][y as usize]);
                        }
                    }
                }
                result.push(block_scan);
                column += NUMBER_BLOCK;
            }
            row += NUMBER_BLOCK;
        }

        let mut refactor_result = Vec::new();

        for i in result.into_iter() {
            // let mut split_factor = i.chunks(NUMBER_BLOCK as usize);
            // let mut arr_new_factor = Vec::new();
            // while let Some(arr) = split_factor.next() {
            //     let x = arr;
            //     arr_new_factor.push(arr)
            // }
            let mut new_vec = Vec::new();
            let mut temper = [Color::default(); 8];
            let mut current_block = 1;
            let mut block_number = 8;
            for (index, element) in i.into_iter().enumerate() {
                if index  < block_number * current_block - 1 as usize {
                    // println!("Index: {}, Block: {}", index, block_number * (current_block - 1));
                    let new_index = index - block_number * (current_block - 1 as usize);
                    // println!("Index: {}", new_index);
                    temper[new_index] = element;
                } else {
                    // println!("HA");
                    let new_index = index - block_number * (current_block - 1 as usize);
                    temper[new_index] = element;
                    new_vec.push(temper);
                    temper = [Color::default(); 8];
                    current_block += 1;
                }
            }
            refactor_result.push(new_vec.to_vec());
        };
    
        refactor_result 
        
    }

    // pub fn dct()
}
