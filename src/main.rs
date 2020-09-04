fn main() {
    adder(1, 2);
    println!("Hello, world!");
}

fn adder(n: u64, m: u64) -> u64 {
    n + m
}

#[test]
fn test_adder1() {
    // 1 + 2 = 3
    assert_eq!(adder(1, 2), 3);
}
