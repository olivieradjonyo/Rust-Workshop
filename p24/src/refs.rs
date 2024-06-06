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

#[cfg(test)]
mod tests {

    #[test]
    fn test_f1() {
        let mut tuple = (10, 20, false);
        {
            let x = crate::refs::f1(&mut tuple);
            assert_eq!(*x, 10); // Should return a mutable reference to the first element
            *x += 5;
        }
        assert_eq!(tuple.0, 15); // Check if the first element was modified correctly
        assert_eq!(tuple.1, 20);

        tuple.2 = true;
        {
            let x = crate::refs::f1(&mut tuple);
            assert_eq!(*x, 20); // Should return a mutable reference to the second element
            *x += 5;
        }
        assert_eq!(tuple.0, 15);
        assert_eq!(tuple.1, 25); // Check if the second element was modified correctly
    }

    #[test]
    fn test_f2() {
        let mut data = [10, 20, 30, 40, 50];

        {
            let x = crate::refs::f2(&mut data, 0);
            assert_eq!(*x, 10); // Should return a mutable reference to the first element
            *x += 1;
        }
        assert_eq!(data, [11, 20, 30, 40, 50]); // Check if the first element was modified correctly

        {
            let x = crate::refs::f2(&mut data, 2);
            assert_eq!(*x, 30); // Should return a mutable reference to the third element
            *x += 1;
        }
        assert_eq!(data, [11, 20, 31, 40, 50]); // Check if the third element was modified correctly

        {
            let x = crate::refs::f2(&mut data, 4);
            assert_eq!(*x, 50); // Should return a mutable reference to the last element
            *x += 1;
        }
        assert_eq!(data, [11, 20, 31, 40, 51]); // Check if the last element was modified correctly
    }

    #[test]
    fn test_f3() {
        let mut data = [10, 20, 30, 40, 50];

        {
            let x = crate::refs::f3(&mut data, 0);
            assert_eq!(*x, 50); // Should return a mutable reference to the last element
            *x += 1;
        }
        assert_eq!(data, [10, 20, 30, 40, 51]); // Check if the last element was modified correctly

        {
            let x = crate::refs::f3(&mut data, 2);
            assert_eq!(*x, 30); // Should return a mutable reference to the third element from the end
            *x += 1;
        }
        assert_eq!(data, [10, 20, 31, 40, 51]); // Check if the third element from the end was modified correctly

        {
            let x = crate::refs::f3(&mut data, 4);
            assert_eq!(*x, 10); // Should return a mutable reference to the first element (0 from the end)
            *x += 1;
        }
        assert_eq!(data, [11, 20, 31, 40, 51]); // Check if the first element was modified correctly
    }

    //f4 unit tests
    #[test]
    fn test_f4_even() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let parts = crate::refs::f4(&data);

        assert_eq!(parts[0], [1, 2]);
        assert_eq!(parts[1], [3, 4]);
        assert_eq!(parts[2], [5, 6]);
        assert_eq!(parts[3], [7, 8]);
    }

    #[test]
    fn test_f4_uneven() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let parts = crate::refs::f4(&data);

        assert_eq!(parts[0], [1, 2, 3]);
        assert_eq!(parts[1], [4, 5, 6]);
        assert_eq!(parts[2], [7, 8, 9]);
        assert_eq!(parts[3], [10]);
        //if the two last code lines are replaced by the following the test should
        //assert_eq!(parts[2], [7, 8]);
        //assert_eq!(parts[3], [9, 10]);
    }

    #[test]
    fn test_f4_fewer_elements() {
        let data = [1, 2, 3];
        let parts = crate::refs::f4(&data);

        assert_eq!(parts[0], [1]);
        assert_eq!(parts[1], [2]);
        assert_eq!(parts[2], [3]);
        assert_eq!(parts[3], []);
    }

    #[test]
    fn test_f4_empty() {
        let data: [u32; 0] = [];
        let parts = crate::refs::f4(&data);

        assert_eq!(parts[0], []);
        assert_eq!(parts[1], []);
        assert_eq!(parts[2], []);
        assert_eq!(parts[3], []);
    }
}
