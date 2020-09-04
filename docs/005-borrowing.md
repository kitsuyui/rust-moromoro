# 参照と借用

シンタックス上は参照は `&` で、参照を外すのは `*` といった点は C や Go と同じニュアンスで扱える様子。

ほぼ https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html に書いてあることそのまま。
Rust の特徴的な点に「所有権」の概念があり、メモリ安全のために他の言語では注意深く扱わないといけなかった代入や参照を、コンパイラがいい感じに教えてくれる。ありがたい。

- 所有権はただ 1 つの変数だけが持つ
- 借用
  - 所有権を手放さずにうまく扱う方法が「借用」名前の通り「貸し出す」感じ。
  - 参照では所有権は移らない
    - 逆に言えば参照でなく (`&` つけずに) 関数に引数で渡したり、代入したりすると、確実に所有権が移る
  - ふつうの参照は read only で、同時に複数に貸し出せる。
  - mut な参照は、変更できる (書き込みできる) ので、同時に 1 つにしか貸し出せない。

デスノートの所有権に近い。

## サンプルコード

```rust
// from https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

```sh
$ cargo run
   Compiling rust-moromoro v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in 2.16s
     Running `target/debug/rust-moromoro`
The length of 'hello' is 5.
```

## 参照でなく渡そうとするとどうなるか

```rust
// from https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> usize {
    s.len()
}
```

いちど `calculate_length` に `s1` を渡してしまったので、もう所有権を失った `s1` を他の関数に貸すのに使えないぞ、と。
デスノートのルールみたいだ。

```sh
$ cargo run
   Compiling rust-moromoro v0.1.0 (...)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:43
  |
3 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
4 |     let len = calculate_length(s1);
  |                                -- value moved here
5 |     println!("The length of '{}' is {}.", s1, len);
  |                                           ^^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rust-moromoro`.

To learn more, run the command again with --verbose.
```

# どう付き合っていけばいいか

- 参照透過性のある関数の引数は参照で戻り値は値で返す良さそう (「参照透過性のある」の時点で、だいぶ自明になってしまうが)
  - とはいえ、 int + int みたいな数値計算でいちいち参照を渡すと、そのほうが高コストになってしまうので、こういうときは普通に値をわたせばいい。
- 破壊的な動作をする関数 (サブルーチンのようなもの) は可変参照 (`&mut`) で渡せば良い
- あとは、適宜 `Copy()` を活用すれば良さそう
