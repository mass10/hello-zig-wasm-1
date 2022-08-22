# 始める前に: rustup をインストールする

https://www.rust-lang.org/ja/tools/install

# 始める前に: Zig をインストールする

https://ziglang.org/download/

# 始める前に: wasmer をインストールする

```CMD
cargo install wasmer-cli --features singlepass,cranelift

cargo install wapm-cli
```


# プロジェクトを作成する

```CMD
MKDIR hello-zig-wasm-1
CD hello-zig-wasm-1

zig init-exe
```

# wasm をビルドする

```CMD
zig build-exe -O ReleaseSmall -target wasm32-wasi src/main.zig

zig build-lib -O ReleaseSmall -target wasm32-wasi -dynamic --export=test1 --export=test2 src/main.zig

```

# wasm を単独で実行してみる

```CMD
wasmer.exe main.wasm
```

# IIS 配下に設置して、ウェブページ上で動かしてみる

```CMD
DIR C:\Inetpub\wwwroot\
MKDIR C:\Inetpub\wwwroot\20220821-my-wasm-by-zig
COPY /Y main.wasm C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
COPY /Y launch.js C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
COPY /Y index.html C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
START http://localhost/20220821-my-wasm-by-zig/
```

# 参考
* wasm で DOM 操作 https://zenn.dev/igrep/articles/2021-11-wasm-reference-types
* wasm で DOM 操作(具体例) https://github.com/shritesh/zig-wasm-dom/blob/gh-pages/zigdom.zig
* 簡単な wasm サンプル https://fukuno.jig.jp/2927
* zig + wasm https://note.com/shift_tech/n/n58cbf573baef
