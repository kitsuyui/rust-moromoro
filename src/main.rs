use std::process;

fn main() {
    let pid = process::id();
    let count = pid.count_ones() as i32;
    process::exit(count);
}
