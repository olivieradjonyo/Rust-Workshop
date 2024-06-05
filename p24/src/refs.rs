//f1: accepts mutable reference to tuple with two u32s and bool flag. If flag is false, returns mutable reference to the first element in the tuple. If flag is true, returns mutable reference to the second element in the tuple.
pub fn f1(tuple: &mut (u32, u32, bool)) -> &mut u32 {
    if tuple.2 {
        &mut tuple.1
    } else {
        &mut tuple.0
    }
}

//– f2: accepts mutable slice &mut [u32] and number n, returns mutable reference to the n-th element in the slice (counting from zero).
pub fn f2(slice: &mut [u32], n: usize) -> &mut u32 {
    &mut slice[n]
}

//– f3: accepts slice &mut [u32] and number n, returns mutable reference to the n-th element from the end of the slice (counting from zero, i.e. 0 means the last element).
pub fn f3(slice: &mut [u32], n: usize) -> &mut u32 {
    let len = slice.len();
    &mut slice[len - 1 - n]
}

//– f4: accepts slice &[u32], partitions it into 4 equal (as much as possible) parts, and returns 4 resulting slices
pub fn f4(slice: &[u32]) -> [&[u32]; 4] {
    let len = slice.len();
    let part_size = (len + 3) / 4; // Calculate the size of each part

    let part1_end = part_size.min(len);
    let part2_end = (2 * part_size).min(len);
    let part3_end = (3 * part_size).min(len);
    
    let part1 = &slice[0..part1_end];
    let part2 = &slice[part1_end..part2_end];
    let part3 = &slice[part2_end..part3_end];
    let part4 = &slice[part3_end..len];
    
    [part1, part2, part3, part4]
}

