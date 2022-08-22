//!
//!
//! Zig による WebAssembly の簡単なサンプルです。
//!
//! * 単純な関数をエクスポートしています。
//!
//! https://note.com/shift_tech/n/n58cbf573baef
//!

const std = @import("std");

/// Zig のエントリーポイントです。
///
/// WebAssembly を出力する場合、意味はありません。(消しても OK)
pub fn main() anyerror!void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("ハロー WASM!\n", .{});
}

/// 何らかの整数を返します。
export fn test1() usize {
    return 1;
}

/// `left` と `right` を足した数を返します。
export fn test2(left: usize, right: usize) usize {
    return left + right;
}
