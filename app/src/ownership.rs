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
}
