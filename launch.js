///
/// コマンドラインから WebAssembly を実行するサンプル
///
/// https://note.com/shift_tech/n/n58cbf573baef
///

const fs = require("fs");

/**
 * WebAssembly をファイルシステムからロードします。
 */
function launchWebAssembly() {
	const content = fs.readFileSync("main.wasm");
	console.log("[DEBUG] module is", content);
	WebAssembly.compile(content)
	.then((module) => {
		const lib = new WebAssembly.Instance(module, {
			env: {
				memoryBase: 0,
				tableBase: 0,
				memory: new WebAssembly.Memory({ initial: 256 }),
				table: new WebAssembly.Table({ initial: 0, element: "anyfunc" }),
			},
		}).exports;
		console.log(lib.test1());
		console.log(lib.test2(100, 1000));
	})
	.catch((e) => {
			console.error(e);
	});
}

/**
 * エントリーポイント
 */
function main() {

	launchWebAssembly();
}

main();
