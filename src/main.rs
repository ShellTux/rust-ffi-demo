use rust_ffi_demo::{add, factorial};

fn main() {
    println!("{} + {} = {}", 34, 35, unsafe { add(34, 35) });

    println!("{}! - 300 = {}", 6, unsafe { factorial(6) } - 300);
}
