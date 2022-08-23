///
/// コマンドラインから WebAssembly を実行するサンプル
///
/// https://note.com/shift_tech/n/n58cbf573baef
///

const fs = require("fs");

/**
 * WebAssembly をファイルシステムからロードします。
 */
async function launchWebAssembly() {
	// ファイルシステムから wasm をロードします。
	const content = fs.readFileSync("main.wasm");

	const module = await WebAssembly.compile(content)

	const mywasm = new WebAssembly.Instance(module, {
		env: {
			memoryBase: 0,
			tableBase: 0,
			memory: new WebAssembly.Memory({ initial: 256 }),
			table: new WebAssembly.Table({ initial: 0, element: "anyfunc" }),
		},
	}).exports;

	// wasm で export されている操作を呼び出します。
	console.log(mywasm.test1());
	console.log(mywasm.test2(100, 1000));
	console.log(mywasm.context().field2);
}

/**
 * エントリーポイント
 */
async function main() {

	await launchWebAssembly();
}

main();
