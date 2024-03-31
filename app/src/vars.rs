// pub mod sub_a;
// pub mod sub_b;

// const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!");
    // モジュール
    // sub_a::func_a();
    // sub_b::func_b();


    // 変数
    // let mut x = 5;
    // println!("x = {}", x);
    // x = 6;
    // println!("x = {}", x);

    // let _i1 = 3;
    // let _f1 = 3.0;


    // ポインタ
    // println!("{}", usize::BITS);
    // println!("Memory address of MAX_POINTS: {:p}", &MAX_POINTS);

    // let i2: i64 = 1;
    // let i3: i64 = 2;
    // println!("Stack address of i2: {:p}", &i2);
    // println!("Stack address of i3: {:p}", &i3);

    // let y = 5;
    // println!("Stack address of y: {:p}", &y);
    // let y = y + 1;
    // println!("Stack address of y: {:p}", &y);
    // let y = y * 2;
    // println!("Stack address of y: {:p}", &y);
    // println!("y = {}", y);
    // {
    //     let y = 3;
    //     println!("Stack address of y: {:p}", &y);
    //     println!("y = {}", y);
    // }
    // println!("Stack address of y: {:p}", &y);
    // println!("y = {}", y);


    // タプル
    // let t1 = (500, 6.4, "Hello");
    // let (x, y, z) = t1;
    // println!("x = {}, y = {}, z = {}", x, y, z);
    // println!("x = {}, y = {}, z = {}", t1.0, t1.1, t1.2);

    // let mut t2 = ((0, 1), (2, 3));
    // let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    // *x1_ptr = 5;
    // *y1_ptr = -5;
    // println!("t2 = {:?}", t2);


    // 配列
    // let a1 = [1, 2, 3, 4, 5];
    // let a2 = [0; 10];
    // println!("a1 = {:?}", a1);
    // println!("a2 = {:?}", a2);
    // println!("a1[2] = {}", a1[2]);
    // println!("a2[3] = {}", a2[3]);

    // 文字列スライス
    // let s1 = "Helloこんにちは"; // 20 bytes
    // let s2 = "hello"; // 5 bytes
    // println!("Stack address of s1: {:p}", &s1); // 静的領域のポインタと長さの情報が格納されている
    // println!("Stack address of s2: {:p}", &s2);
    // println!("Static memory address of s1: {:p}", s1.as_ptr());
    // println!("Static memory address of s2: {:p}", s2.as_ptr());
    // println!("Length of s1: {}", s1.len());
    // println!("Length of s2: {}", s2.len());

    // String
    // let mut s1 = String::from("hello");
    // let mut s2 = String::from("こんにちは");
    // println!("Stack address of s1: {:p}", &s1); // ヒープ領域のポインタ・長さ・容量の情報が格納されている
    // println!("Stack address of s2: {:p}", &s2);
    // println!("Heap memory address of s1: {:p}", s1.as_ptr());
    // println!("Heap memory address of s2: {:p}", s2.as_ptr());
    // println!("Length of s1: {}", s1.len());
    // println!("Length of s2: {}", s2.len());
    // println!("Capacity of s1: {}", s1.capacity());
    // println!("Capacity of s2: {}", s2.capacity());
    // s1.push_str(", world!");
    // s2.push_str("、世界！");
    // println!("s1 = {}", s1);
    // println!("s2 = {}", s2);
    // println!("Length of s1: {}", s1.len());
    // println!("Length of s2: {}", s2.len());
    // println!("Capacity of s1: {}", s1.capacity());
    // println!("Capacity of s2: {}", s2.capacity());
}
