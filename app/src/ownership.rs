pub fn run() {
    let s1 = String::from("hello"); // s1が所有権を持つ
    let s2 = s1; // s1の所有権がs2に移動する
    // println!("{} {}", s1, s2); // s1は所有権を持っていないのでエラーになる
    println!("{}", s2); // s2は所有権を持っているので正常に表示される
}
