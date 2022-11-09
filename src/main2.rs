

fn main() {
    // 4 X 4
        // 0 1 2 3
        // 4 - 4 = 0 
        
        // 4 5 6 7
        // 8 - 4 = 4

        // 8 9 10 11
        // 12 - 4 = 8
        let vec =vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];  
        let mut new_vec = Vec::new();
        let mut block_number = 4;
        let mut temper = [0; 4];
        let mut current_block = 1;
        for (index, element) in vec.into_iter().enumerate() {
            // println!("Wow: {}, {}, Block: {}", index, block_number * current_block, (index as u8) - block_number * (current_block - 1) );

            if index  < block_number * current_block - 1 as usize {
                println!("Index: {}, Block: {}", index, block_number * (current_block - 1));
                let new_index = index - block_number * (current_block - 1 as usize);
                // println!("Index: {}", new_index);
                temper[new_index] = element;
            } else {
                // println!("HA");
                let new_index = index - block_number * (current_block - 1 as usize);
                temper[new_index] = element;
                new_vec.push(temper);
                temper = [0; 4];
                current_block += 1;
            }
        }
        
        println!("{:?}", new_vec);
    }