# ジェネリクス

意外ととっつきやすい。
bubble sort のアルゴリズムを T: Ord に一般化してみる。

```rust
pub fn sort(slice: &mut [i32]) {
    let size = slice.len();
    for i in 0..size {
        for j in 1..(size - i) {
            if slice[j] < slice[j - 1] {
                slice.swap(j, j - 1);
            }
        }
    }
}
```

```rust
pub fn sort<T: Ord>(slice: &mut [T]) {
    let size = slice.len();
    for i in 0..size {
        for j in 1..(size - i) {
            if slice[j] < slice[j - 1] {
                slice.swap(j, j - 1);
            }
        }
    }
}
```
