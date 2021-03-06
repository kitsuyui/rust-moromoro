# ビルド

`cargo new --bin` で初期化した時点で `Hello, World!` を print するプログラムが作成されている模様。
親切設計。

```rust
fn main() {
    println!("Hello, world!");
}
```

あとはそのまま `cargo build` すればバイナリが生成される模様。

初回は少し時間がかかるが、次は差分だけでビルドできるので瞬時に終わる模様。

```sh
$ cargo build
   Compiling rust-moromoro v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in 12.52s
yui-macbook:rust-moromoro yui
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
```

```sh
$ ./target/debug/rust-moromoro
Hello, world!
```

`target` ディレクトリ以下にオブジェクトファイルやデバッグシンボルファイル、メタデータなども生成される模様。

```sh
$ find target
target
target/.rustc_info.json
target/debug
target/debug/rust-moromoro
target/debug/rust-moromoro.dSYM
target/debug/.fingerprint
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3/bin-rust_moromoro-08659217bcfe64b3.json
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3/bin-rust_moromoro-08659217bcfe64b3
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3/dep-bin-rust_moromoro-08659217bcfe64b3
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3/invoked.timestamp
target/debug/incremental
target/debug/incremental/rust_moromoro-1ododigxfozca
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/3w9vhs9njjbccr0m.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/query-cache.bin
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/2eovmg0oypd7o7ad.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/1qop8du6flddgbzw.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/317xf69ii0o8lhhk.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/dep-graph.bin
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/1r0t93cd05c99xrt.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/work-products.bin
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/16t0nb7pgeklfzsa.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef.lock
target/debug/.cargo-lock
target/debug/examples
target/debug/deps
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents/Resources
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents/Resources/DWARF
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents/Resources/DWARF/rust_moromoro-08659217bcfe64b3
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents/Info.plist
target/debug/deps/rust_moromoro-08659217bcfe64b3.d
target/debug/deps/rust_moromoro-08659217bcfe64b3
target/debug/build
target/debug/rust-moromoro.d
```

# リリースビルド

開発中のためではなく本ちゃんのビルドをしたい場合 `--release` オプションを使う模様。

```sh
$ cargo build --release
```

```sh
$ ./target/release/rust-moromoro
Hello, world!
```

```sh
$ find target
target
target/.rustc_info.json
target/release
target/release/rust-moromoro
target/release/.fingerprint
target/release/.fingerprint/rust-moromoro-09d71e6080f029f6
target/release/.fingerprint/rust-moromoro-09d71e6080f029f6/bin-rust_moromoro-09d71e6080f029f6.json
target/release/.fingerprint/rust-moromoro-09d71e6080f029f6/dep-bin-rust_moromoro-09d71e6080f029f6
target/release/.fingerprint/rust-moromoro-09d71e6080f029f6/invoked.timestamp
target/release/.fingerprint/rust-moromoro-09d71e6080f029f6/bin-rust_moromoro-09d71e6080f029f6
target/release/incremental
target/release/.cargo-lock
target/release/examples
target/release/deps
target/release/deps/rust_moromoro-09d71e6080f029f6.d
target/release/deps/rust_moromoro-09d71e6080f029f6
target/release/build
target/release/rust-moromoro.d
target/debug
target/debug/rust-moromoro
target/debug/rust-moromoro.dSYM
target/debug/.fingerprint
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3/bin-rust_moromoro-08659217bcfe64b3.json
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3/bin-rust_moromoro-08659217bcfe64b3
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3/dep-bin-rust_moromoro-08659217bcfe64b3
target/debug/.fingerprint/rust-moromoro-08659217bcfe64b3/invoked.timestamp
target/debug/incremental
target/debug/incremental/rust_moromoro-1ododigxfozca
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/3w9vhs9njjbccr0m.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/query-cache.bin
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/2eovmg0oypd7o7ad.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/1qop8du6flddgbzw.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/317xf69ii0o8lhhk.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/dep-graph.bin
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/1r0t93cd05c99xrt.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/work-products.bin
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef-27omewu0dotf0/16t0nb7pgeklfzsa.o
target/debug/incremental/rust_moromoro-1ododigxfozca/s-fqvm7bwc0x-2v62ef.lock
target/debug/.cargo-lock
target/debug/examples
target/debug/deps
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents/Resources
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents/Resources/DWARF
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents/Resources/DWARF/rust_moromoro-08659217bcfe64b3
target/debug/deps/rust_moromoro-08659217bcfe64b3.dSYM/Contents/Info.plist
target/debug/deps/rust_moromoro-08659217bcfe64b3.d
target/debug/deps/rust_moromoro-08659217bcfe64b3
target/debug/build
target/debug/rust-moromoro.d
```

# `cargo run`

モダンな言語らしく、いちいちビルドしなくても実行できる模様。

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/rust-moromoro`
Hello, world!
```

# Cargo.lock

Gemfile.lock, yarn.lock, Pipfile.lock などでおなじみ、ロックファイルのようだ。
たぶん Rust のパッケージのバージョンを固定してくれるんだろう。
今は何にも依存していないので、特に何もロックされていなさそう。

```
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
[[package]]
name = "rust-moromoro"
version = "0.1.0"

```
