extern crate libc;
use libc::c_int;

extern {
    fn Add(x: c_int, y: c_int) -> c_int;
}

fn main() {
    let x: c_int = 20;
    let y: c_int = 22;
    let result: c_int;

    unsafe {
        result = Add(x, y);
    }

    println!("The answer is: {}", result);
}
