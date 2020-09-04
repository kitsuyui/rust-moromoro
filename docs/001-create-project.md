# リポジトリの作成

`cargo new` コマンドでプロジェクトを作ると、勝手に Git のリポジトリとしても初期化する模様。

このリポジトリを作るのに、こんなことをした。

```sh
$ cargo new --bin rust-moromoro
$ cd rust-moromoro
$ touch README.md
$ vim README.md
$ git add .
$ git commit
$ git remote add origin git@github.com:kitsuyui/rust-moromoro.git
$ git push
```

バイナリアプリケーションを作るなら `--bin` オプション付きで、ライブラリを作るなら `--lib` オプション付きで実行する模様。
デフォルトが Git リポジトリとして初期化されたのは `--vcs` オプションで変えられた模様。

```sh
$ cargo new --help
cargo-new
Create a new cargo package at <path>

USAGE:
    cargo new [OPTIONS] <path>

OPTIONS:
    -q, --quiet                  No output printed to stdout
        --registry <REGISTRY>    Registry to use
        --vcs <VCS>              Initialize a new repository for the given version control system (git, hg, pijul, or
                                 fossil) or do not initialize any version control at all (none), overriding a global
                                 configuration. [possible values: git, hg, pijul, fossil, none]
        --bin                    Use a binary (application) template [default]
        --lib                    Use a library template
        --edition <YEAR>         Edition to set for the crate generated [possible values: 2015, 2018]
        --name <NAME>            Set the resulting package name, defaults to the directory name
    -v, --verbose                Use verbose output (-vv very verbose/build.rs output)
        --color <WHEN>           Coloring: auto, always, never
        --frozen                 Require Cargo.lock and cache are up to date
        --locked                 Require Cargo.lock is up to date
        --offline                Run without accessing the network
    -Z <FLAG>...                 Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    -h, --help                   Prints help information

ARGS:
    <path>
```



# バージョンの確認

```sh
$ cargo --version
cargo 1.39.0 (1c6ec66d5 2019-09-30)
$ rustc --version
rustc 1.39.0 (4560ea788 2019-11-04)
```
