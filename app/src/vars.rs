pub mod sub_a;
pub mod sub_b;

pub fn run() {
    println!("Here is vars module!");
    sub_a::func_a();
    sub_b::func_b();
}
