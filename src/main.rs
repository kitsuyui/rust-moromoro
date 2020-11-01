pub mod sorts;

fn main() {
    let a = [1, 3, 2, 5, 4];

    let mut b = a.clone();
    sorts::builtin_sort::sort(&mut b);
    println!("builtin sort: {:?}", b);

    let mut b = a.clone();
    sorts::bubble_sort::sort(&mut b);
    println!("bubble sort: {:?}", b);

    let mut b = a.clone();
    sorts::heap_sort::sort(&mut b);
    println!("heap sort: {:?}", b);
}
