# 共有ライブラリを作る

dynamic link library とか shared library とも言う

`Cargo.toml` に追加

```toml
[lib]
name = "moromoro"
crate_type = ["cdylib"]
```

`crate_type` に `"rlib"` も含めておくと Rust 用のライブラリとしてもビルドする。
`src/lib.rs` の中身: シンボルを残すため、 `#[no_mangle]` アトリビュートが必要。

```rust
#[no_mangle]
pub fn adder(n: u32, m: u32) -> u32 {
    n + m
}
```

```sh
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
$ ls target/debug/libmoromoro.dylib
target/debug/libmoromoro.dylib
```

# Python からロードして叩いてみる

```python
>>> from ctypes import cdll
>>> moromoro = cdll.LoadLibrary('target/debug/libmoromoro.dylib')
>>> moromoro.adder(1, 2)
3
>>> type(moromoro.adder(1, 2))
<class 'int'>
```
