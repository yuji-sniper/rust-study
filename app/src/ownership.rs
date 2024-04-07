pub fn run() {
    // 所有権の移動
    let s1 = String::from("hello"); // s1が所有権を持つ
    let s2 = s1; // s1の所有権がs2に移動する
    // println!("{} {}", s1, s2); // s1は所有権を持っていないのでエラーになる
    println!("{}", s2); // s2は所有権を持っているので正常に表示される

    // コピー
    let i1 = 1;
    let i2 = i1; // コピーされる
    println!("{} {}", i1, i2); // アドレスは異なるが値は同じ
    println!("Stack address of i1: {:p}", &i1);
    println!("Stack address of i2: {:p}", &i2);
}
