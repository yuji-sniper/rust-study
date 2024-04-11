pub fn run() {
    // 所有権の移動
    let s1 = String::from("hello"); // s1が所有権を持つ
    let s2 = s1; // s1の所有権がs2に移動する
    // println!("{} {}", s1, s2); // s1は所有権を持っていないのでエラーになる
    println!("{}", s2); // s2は所有権を持っているので正常に表示される

    // int型の場合（Copyトレイトを実装している型）
    let i1 = 1;
    let i2 = i1; // コピーされる
    println!("{} {}", i1, i2); // アドレスは異なるが値は同じ
    println!("Stack address of i1: {:p}", &i1);
    println!("Stack address of i2: {:p}", &i2);

    // 文字列スライス（これもCopyトレイトを実装している型）。参照をコピーしている。
    let sl1 = "hello";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2); // アドレスは異なるが値は同じ
    println!("Stack address of sl1: {:p}", &sl1);
    println!("Stack address of sl2: {:p}", &sl2);

    // クローン
    let s3 = String::from("hello");
    let s4 = s3.clone(); // クローンされる
    println!("{} {}", s3, s4); // s3とs4は別の所有権を持つ
    println!("Heap address of s3: {:p}", s3.as_ptr());
    println!("Heap address of s4: {:p}", s4.as_ptr());

    // 関数に所有権を渡す
    let s5 = String::from("hello");
    println!("Stack address of s5: {:p}", &s5);
    println!("Heap address of s5: {:p}", s5.as_ptr());
    println!("Length of s5: {}", s5.len());
    println!("Capacity of s5: {}", s5.capacity());
    take_ownership(s5); // s5の所有権が関数に移動する
    // println!("{}", s5); // s5は所有権を持っていないのでエラーになる

    // 関数から所有権を返す
    let s6 = String::from("hello");
    println!("Stack address of s6: {:p}", &s6);
    println!("Heap address of s6: {:p}", s6.as_ptr());
    println!("Length of s6: {}", s6.len());
    let s7 = take_give_back_ownership(s6); // s6の所有権が関数に移動し、関数から返された所有権がs7に移動する
    println!("Stack address of s7: {:p}", &s7);
    println!("Heap address of s7: {:p}", s7.as_ptr()); // ヒープアドレスは同じ
    println!("Length of s7: {}", s7.len());
    // println!("{}", s6); // s6は所有権を持っていないのでエラーになる
}

fn take_ownership(s: String) {
    println!("Stack address of s: {:p}", &s);
    println!("Heap address of s: {:p}", s.as_ptr()); // ヒープアドレスは同じ
    println!("Length of s: {}", s.len());
    println!("Capacity of s: {}", s.capacity());
    println!("{}", s);
}

fn take_give_back_ownership(s: String) -> String {
    s // 所有権を返す
}
