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

# Publish

```CMD
DIR C:\Inetpub\wwwroot\
MKDIR C:\Inetpub\wwwroot\20220821-my-wasm-by-zig
COPY /Y main.wasm C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
COPY /Y launch.js C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
COPY /Y index.html C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
START http://localhost/20220821-my-wasm-by-zig/
```
