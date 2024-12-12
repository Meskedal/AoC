pub fn input_generator(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

pub fn part_1(input: &Vec<u32>) -> u64 {
    let mut filesystem: Vec<u32> = vec![];
    
    let mut index_left = 0;
    let mut index_right = input.len() - 1;

    let mut file_block_id_right = (input.len()/2) as u32; // The max file block id is given by the length of the input divided by 2 as the input comes in pairs. 
    let mut file_block_id_left = 0;
    let mut file_block_count_right = input[index_right];
    let mut current_free_space = 0;
    'main: loop {
        // If we have available free spaces, it is time to fill them with the right side file block count 
        while current_free_space > 0 {
            // If the current file block count has been fully moved, then we need to iterate to the next file block count, skipping the free space count for the right iterator
            if file_block_count_right == 0 {
                index_right -= 2; 
                if index_left >= index_right {
                    // If the left index at this point has reached the right index, then there are nothing left to do. 
                    break 'main;
                }
                file_block_count_right = input[index_right];
                file_block_id_right -= 1; 
            }
            filesystem.push(file_block_id_right);
            file_block_count_right -= 1;
            current_free_space -= 1;
        }

        let left = input[index_left];
        for _ in 0..left {
            filesystem.push(file_block_id_left);
        }
        file_block_id_left += 1;

        index_left += 1;
        // at this point we know that the left index is pointing at a free spaces count
        current_free_space = input[index_left];
        index_left += 1;

        if index_left >= index_right {
            // If there are remaining file block count on the right side, dump them before exiting. 
            for _ in 0..file_block_count_right {
                filesystem.push(file_block_id_right);
            } 
            break;
        }
    }
    let filesystem_checksum = filesystem.iter().enumerate().fold(0_u64, |acc, (i, &x)| {
        acc + x as u64 * i as u64
    });
    filesystem_checksum
}

// This function populates the final checksum directly instead of using a vector to store the filesystem
// before calculating the checksum.
pub fn part_1_v2(input: &Vec<u32>) -> u64 {
    let mut filesystem_checksum = 0;
    let mut filesystem_index = 0;
    let mut index_left = 0;
    let mut index_right = input.len() - 1;

    let mut file_block_id_right = (input.len()/2) as u32; // The max file block id is given by the length of the input divided by 2 as the input comes in pairs. 
    let mut file_block_id_left = 0;
    let mut file_block_count_right = input[index_right];
    let mut current_free_space = 0;
    'main: loop {
        // If we have available free spaces, fill them with the right side file block count 
        while current_free_space > 0 {
            // If the current file block count has been fully moved, then we need to iterate to the next file block count, skipping the free space count for the right iterator
            if file_block_count_right == 0 {
                index_right -= 2; 
                if index_left >= index_right {
                    // If the left index at this point has reached the right index, then there are nothing left to do. 
                    break 'main;
                }
                file_block_count_right = input[index_right];
                file_block_id_right -= 1; 
            }
            filesystem_checksum += file_block_id_right as u64 * filesystem_index as u64;
            filesystem_index += 1;
            file_block_count_right -= 1;
            current_free_space -= 1;
        }

        let left = input[index_left];
        for _ in 0..left {
            filesystem_checksum += file_block_id_left as u64 * filesystem_index as u64;
            filesystem_index += 1;
        }
        file_block_id_left += 1;

        index_left += 1;
        // at this point we know that the left index is pointing at a free spaces count
        current_free_space = input[index_left];
        index_left += 1;

        if index_left >= index_right {
            // If there are remaining file block count on the right side, dump them before exiting. 
            for _ in 0..file_block_count_right {
                filesystem_checksum += file_block_id_right as u64 * filesystem_index as u64;
                filesystem_index += 1;
            } 
            break;
        }
    }
    filesystem_checksum
}

pub fn part_2(input: &Vec<u32>) -> i32 {
    let mut filesystem_checksum = 0;

    filesystem_checksum
}