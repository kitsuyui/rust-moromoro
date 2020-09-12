# bitcount の最適化をしたい

プロセス ID の立っているビットを数えて、その数を終了ステータスコードとして出力するプログラムを書く。
x86_64 に最適化して出力してくれれば、 SSE4.2 の POPCNT を利用してくれるはず。

なお、入力をプロセス ID にするのは、プログラム内の定数にするとコンパイル時に最適化されて、数え終わったビットだけが残る可能性があるため。
終了ステータスにするのは、 print を入れると文字列処理のシンボルが大きく、単に読みづらくなるため。

```
$ ./program & pid=$! ; wait "$pid" ; echo "$pid" $?
```

を実行すると `62321 10` のような結果を得る。 (`62321 == 0b1111001101110001`)

# 例題

## C の場合

x86 (x86_64) アーキテクチャ依存になってしまうけど

```c
#include <unistd.h>
#include <x86intrin.h>

int main(void) {
    pid_t pid;
    pid = getpid();
    return _popcnt32(pid);
}
```

```sh
$ cc -O3 -march=native main.c
$ otool -tvV ./a.out| less
```

```
./a.out:
(__TEXT,__text) section
_main:
0000000100000f80        pushq   %rbp
0000000100000f81        movq    %rsp, %rbp
0000000100000f84        callq   0x100000f90 ## symbol stub for: _getpid
0000000100000f89        popcntl %eax, %eax
0000000100000f8d        popq    %rbp
0000000100000f8e        retq
```

## Rust

```
$ RUSTFLAGS='-C opt-level=3 -C debug-assertions=no -C debuginfo=0 -C target-cpu=native -C panic=abort' cargo build --release
$ otool -tvV target/release/rust-moromoro | less
```

## このあたりをちゃんと追っていくと

Rust の count_ones は `std::intrinsics` の `ctpop` を使っていて、
これが LLVM intrinsics の `ctpop` と対応していて、最終的に SSE4.2 の `POPCNT` までつながっているのがわかる。

https://doc.rust-lang.org/std/primitive.u32.html#method.count_ones
https://doc.rust-lang.org/std/intrinsics/fn.ctpop.html
https://llvm.org/docs/LangRef.html#llvm-ctpop-intrinsic
https://github.com/llvm/llvm-project/blob/master/libclc/generic/include/integer/popcount.h
https://github.com/llvm/llvm-project/blob/master/llvm/test/Analysis/CostModel/X86/ctpop.ll
https://github.com/llvm/llvm-project/blob/master/clang/lib/Headers/popcntintrin.h

## メモ: gnzlbg/bitwise

gnzlbg さんが bitwise のライブラリを作っていた。
最初は LLVMint を使っていたが、後に自作の bitintr クレートに切り替えた模様。

https://github.com/gnzlbg/bitwise
https://github.com/gnzlbg/bitintr
https://github.com/huonw/llvmint

# 結論

- Rust の最適化は、最終的には LLVM の最適化と同じ水準になっていくと考えられる。
- なので、ビット演算をする場合、 LLVM intrinsics に存在する操作は、それをそのまま信じて使うのが、パフォーマンス・信頼性・ポータビリティともにベスト。
  - gnzlbg/bitwise, gnzlbg/bitintr のコミットログを読んでいくと、 core::arch の成熟にともなって、だんだんとコードがシンプルになっていく様子がわかる。
  - `asm!` を使って手で最適化すると、コンパイラの最適化を妨げる場合もあるので、なるべくやめたほうがよい、と書いてある。 (そりゃそうだ)
  - `llvm_asm!` の方ならマシなんだろうか……?
