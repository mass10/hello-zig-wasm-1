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
```

# wasm を単独で実行してみる

```CMD
wasmer.exe main.wasm
```
