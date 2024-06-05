#[test]
fn test_f1() {
    let mut tuple = (10, 20, false);
    {
        let x = p24::refs::f1(&mut tuple);
        assert_eq!(*x, 10); // Should return a mutable reference to the first element
        *x += 5;
    }
    assert_eq!(tuple.0, 15); // Check if the first element was modified correctly
    assert_eq!(tuple.1, 20);

    tuple.2 = true;
    {
        let x = p24::refs::f1(&mut tuple);
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
        let x = p24::refs::f2(&mut data, 0);
        assert_eq!(*x, 10); // Should return a mutable reference to the first element
        *x += 1;
    }
    assert_eq!(data, [11, 20, 30, 40, 50]); // Check if the first element was modified correctly

    {
        let x = p24::refs::f2(&mut data, 2);
        assert_eq!(*x, 30); // Should return a mutable reference to the third element
        *x += 1;
    }
    assert_eq!(data, [11, 20, 31, 40, 50]); // Check if the third element was modified correctly

    {
        let x = p24::refs::f2(&mut data, 4);
        assert_eq!(*x, 50); // Should return a mutable reference to the last element
        *x += 1;
    }
    assert_eq!(data, [11, 20, 31, 40, 51]); // Check if the last element was modified correctly
}

#[test]
fn test_f3() {
    let mut data = [10, 20, 30, 40, 50];

    {
        let x = p24::refs::f3(&mut data, 0);
        assert_eq!(*x, 50); // Should return a mutable reference to the last element
        *x += 1;
    }
    assert_eq!(data, [10, 20, 30, 40, 51]); // Check if the last element was modified correctly

    {
        let x = p24::refs::f3(&mut data, 2);
        assert_eq!(*x, 30); // Should return a mutable reference to the third element from the end
        *x += 1;
    }
    assert_eq!(data, [10, 20, 31, 40, 51]); // Check if the third element from the end was modified correctly

    {
        let x = p24::refs::f3(&mut data, 4);
        assert_eq!(*x, 10); // Should return a mutable reference to the first element (0 from the end)
        *x += 1;
    }
    assert_eq!(data, [11, 20, 31, 40, 51]); // Check if the first element was modified correctly
}

//f4 unit tests
#[test]
    fn test_f4_even() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let parts = p24::refs::f4(&data);

        assert_eq!(parts[0], [1, 2]);
        assert_eq!(parts[1], [3, 4]);
        assert_eq!(parts[2], [5, 6]);
        assert_eq!(parts[3], [7, 8]);
    }

    #[test]
    fn test_f4_uneven() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let parts = p24::refs::f4(&data);

        assert_eq!(parts[0], [1, 2, 3]);
        assert_eq!(parts[1], [4, 5, 6]);
        assert_eq!(parts[2], [7, 8, 9]);
        assert_eq!(parts[3], [10]);
        //if the two last code lines are replaced by the following the test should fail
        //assert_eq!(parts[2], [7, 8]);
        //assert_eq!(parts[3], [9, 10]);
    }

    #[test]
    fn test_f4_fewer_elements() {
        let data = [1, 2, 3];
        let parts = p24::refs::f4(&data);

        assert_eq!(parts[0], [1]);
        assert_eq!(parts[1], [2]);
        assert_eq!(parts[2], [3]);
        assert_eq!(parts[3], []);
    }

    #[test]
    fn test_f4_empty() {
        let data: [u32; 0] = [];
        let parts = p24::refs::f4(&data);

        assert_eq!(parts[0], []);
        assert_eq!(parts[1], []);
        assert_eq!(parts[2], []);
        assert_eq!(parts[3], []);
    }