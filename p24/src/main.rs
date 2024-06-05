fn main() {
    //-----------------------------f1-----------------------------
    let mut my_tuple = (10, 20, false);
    {
        let x = p24::refs::f1(&mut my_tuple);
        *x += 1;
    }
    println!("{:?}", my_tuple); // Output: (11, 20, false)

    my_tuple.2 = true;
    {
        let x = p24::refs::f1(&mut my_tuple);
        *x += 1;
    }
    println!("{:?}", my_tuple); // Output: (11, 21, true)
    //-----------------------------f2-----------------------------
    let mut data = [10, 20, 30, 40, 50];
    {
        let x = p24::refs::f2(&mut data, 2);
        *x += 1;
    }
    println!("{:?}", data); // Output: [10, 20, 31, 40, 50]
    //-----------------------------f3-----------------------------
    let mut data = [10, 20, 30, 40, 50];
    {
        let x = p24::refs::f3(&mut data, 0);
        *x += 1;
    }
    println!("{:?}", data); // Output: [10, 20, 30, 40, 51]

    {
        let x = p24::refs::f3(&mut data, 2);
        *x += 1;
    }
    println!("{:?}", data); // Output: [10, 20, 31, 40, 51]
    //-----------------------------f4-----------------------------
    let data = [10, 20, 30, 40, 50, 60, 70, 80];
    let parts = p24::refs::f4(&data);

    for (i, part) in parts.iter().enumerate() {
        println!("Part {}: {:?}", i + 1, part);
    }
}