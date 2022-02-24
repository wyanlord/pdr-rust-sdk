use pdr_rust_sdk::{get_slice, my_func};

fn main() {
    let res = my_func();
    let s = get_slice("123");

    println!("Hello, world {} {}!", res, s);
}
