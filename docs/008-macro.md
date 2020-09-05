# マクロ

ビット演算のため `u8`, `u16` (`u32`, `u64`, `u128` も) を同一視して扱いたい場合に、
`Bits` トレイトにまったく同じ実装をするのがだるかったので、マクロを使ってみた。

https://doc.rust-jp.rs/book/second-edition/appendix-04-macros.html

## Before

```rust
trait Bits {
    fn bit_count(self) -> u32;
}

impl Bits for u8 {
    fn bit_count(self) -> u32 {
        self.count_ones()
    }
}

impl Bits for u16 {
    fn bit_count(self) -> u32 {
        self.count_ones()
    }
}
```

## After

```rust
trait Bits {
    fn bit_count(self) -> u32;
}

macro_rules! impl_Bits {
    ($T:ty) => {
        impl Bits for $T {
            fn bit_count(self) -> u32 {
                self.count_ones()
            }
        }
    };
}

impl_Bits!(u8);
impl_Bits!(u16);
```

かなりシンプルになった。 `u32`, `u64`, `u128` に対応する場合も以下で OK

```
impl_Bits!(u32);
impl_Bits!(u64);
impl_Bits!(u128);
```
