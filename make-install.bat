@SETLOCAL
@ECHO OFF


REM ========== WebAssembly を出力 ==========
REM CALL zig build-exe -O ReleaseSmall -target wasm32-wasi src/main.zig
CALL zig build-lib -O ReleaseSmall -target wasm32-wasi src/main.zig -dynamic --export=test1 --export=test2 

REM ========== IIS の仮想ディレクトリ配下に配置 ==========
MKDIR C:\Inetpub\wwwroot\20220821-my-wasm-by-zig

COPY /Y main.wasm C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
COPY /Y RESOURCES\launch.js C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
COPY /Y RESOURCES\index.html C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
COPY /Y RESOURCES\favicon.ico C:\Inetpub\wwwroot\20220821-my-wasm-by-zig\
