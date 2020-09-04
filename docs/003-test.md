# テスト

`#[test]` マーカをつけたコードがテストコード扱いになる模様。
足し算をするだけのちゃちな関数を作ってテストしてみる。

```rust
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
```

# テストの実行

```sh
$ cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/rust_moromoro-c09a9e232855ac1e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

良さそう

# テストコードのシンボルはビルドしたバイナリに残るのか

結論: 残らない。テスト書きまくってもビルドしたバイナリがでかくなることはなさそう。

`cargo test` で実行したバイナリの中にはテストコード用のシンボルがある

```
$ strings < target/debug/deps/rust_moromoro-c09a9e232855ac1e | grep test_adder
 right: ``src/main.rstest_adder1
__ZN13rust_moromoro11test_adder128_$u7b$$u7b$closure$u7d$$u7d$17h00271a53d7a8da4cE
__ZN13rust_moromoro11test_adder117hb01269150e06d730E
__ZN13rust_moromoro11test_adder128_$u7b$$u7b$closure$u7d$$u7d$17h00271a53d7a8da4cE
__ZN13rust_moromoro11test_adder117hb01269150e06d730E
```

が、普通にビルドしたバイナリの中にはない。

```
$ cargo build
   Compiling rust-moromoro v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
$ strings < target/debug/rust-moromoro | grep test_adder
(無)
```
