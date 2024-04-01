pub fn run() {
    // Stack
    // let a1: [u8; 7000000] = [1; 7000000];
    // println!("Length of a1: {}", a1.len());

    // Vector
    let mut v1 = vec![1, 2, 3, 4, 5];
    // let v2 = vec![6, 7, 8, 9, 10];
    let mut v3 = vec![11, 12];
    println!("Stack address of v1: {:p}", &v1);
    println!("Heap address of v1: {:p}", v1.as_ptr());
    println!("Len of v1: {}", v1.len());
    println!("Capacity of v1: {}", v1.capacity());
    v1.insert(1, 100);
    println!("v1: {:?}", v1);
    v1.remove(0);
    println!("v1: {:?}", v1);
    v1.append(&mut v3);
    println!("v1: {:?}", v1);
    println!("v3: {:?}", v3);
}
