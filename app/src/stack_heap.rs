pub fn run() {
    // Stack
    // let a1: [u8; 7000000] = [1; 7000000];
    // println!("Length of a1: {}", a1.len());

    // Vector
    // let mut v1 = vec![1, 2, 3, 4, 5];
    // let mut v3 = vec![11, 12];
    // println!("Stack address of v1: {:p}", &v1);
    // println!("Heap address of v1: {:p}", v1.as_ptr());
    // println!("Len of v1: {}", v1.len());
    // println!("Capacity of v1: {}", v1.capacity());
    // v1.insert(1, 100);
    // println!("v1: {:?}", v1);
    // v1.remove(0);
    // println!("v1: {:?}", v1);
    // v1.append(&mut v3);
    // println!("v1: {:?}", v1);
    // println!("v3: {:?}", v3);

    // Box pointer
    let t1: (i64, String) = (10, String::from("Hello"));
    println!("Stack address of t1: {:p}", &t1);
    println!("Heap address of t1.1: {:?}", &t1.1.as_ptr());
    println!("Len of t1.1: {}", t1.1.len());
    println!("Capacity of t1.1: {}", t1.1.capacity());
    println!("t1: {:?}", t1);
    let mut b1 = Box::new(t1);
    (*b1).1 += " world";
    println!("b1: {:?}", b1);
    println!("Stack address of b1: {:p}", &b1);
    println!("Heap address of b1: {:p}", b1);
}
