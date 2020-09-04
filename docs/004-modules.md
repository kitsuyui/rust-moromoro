# モジュール

## rust のモジュールはどういう考え方なのか

https://doc.rust-jp.rs/book/second-edition/ch07-00-modules.html

- モジュールとは、単に名前空間を分けるためのしくみ
  - なので、同一ファイル内であっても `mod` キーワードでモジュールを切ることができる
  - もちろんファイルを読むのにも使える
- 可視・不可視は `pub` キーワードで制御する
- `use` キーワードで名前空間をいろいろとこねくり回すのに便利。他の言語でいう `from ... import ... as ...` みたいにとらえてよさあそう。
  - 全部のモジュールへの参照を FQDN で書かなくて良くなる

## 実験

`src/main.rs`

```rust
mod hoge;

mod fuga {
    pub fn hello_module() {
        println!("Hello, module fuga!");
    }
}

fn main() {
    hoge::hello_module();
    fuga::hello_module();
}
```

`src/hoge.rs`

```rust
pub fn hello_module() {
    println!("Hello, module hoge!");
}
```

```sh
$ cargo run
   Compiling rust-moromoro v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in 0.89s
     Running `target/debug/rust-moromoro`
Hello, module hoge!
Hello, module fuga!
Hello, world!
```

# extern crate

ざっとドキュメントを読んだ感じでは C の `extern` と同じくらいの気持ちでいれば良さそう。現段階ではあまり深く考えないようにする。
