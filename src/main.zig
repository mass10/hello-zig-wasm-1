//!
//!
//! Zig による WebAssembly の簡単なサンプルです。
//!
//! * 単純な関数をエクスポートしています。
//!
//! https://note.com/shift_tech/n/n58cbf573baef
//!

const std = @import("std");

/// コンフィギュレーション構造体
const GlobalCotext = struct {
    /// 文字列
    field1: []const u8 = "",

    /// 整数
    field2: u32 = 123,
};

/// mutable
var _ctx = GlobalCotext{};

/// Zig のエントリーポイントです。
///
/// WebAssembly を出力する場合、意味はありません。(消しても OK)
pub fn main() anyerror!void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("ハロー WASM!\n", .{});
    try stdout.print("u32: [{}]\n", .{_ctx.field2});
    _ctx.field2 = _ctx.field2 + 1;
    try stdout.print("u32: [{}]\n", .{_ctx.field2});
}

/// 何らかの整数を返します。
export fn test1() i32 {
    return 1;
}

/// `left` と `right` を足した数を返します。
export fn test2(left: i32, right: i32) i32 {
    return left + right;
}

/// コンテキストオブジェクトを公開できるか？(できてない)
export fn context() *GlobalCotext {
    return &_ctx;
}
