//!
//! ビルドツール
//!
//! 実行は make で
//!

/// ディレクトリーを作成します。
fn mkdir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	let pt = std::path::Path::new(path);
	if pt.is_dir() {
		return Ok(());
	}
	println!("[INFO] MKDIR> [{}]", path);
	std::fs::create_dir(path)?;
	return Ok(());
}

/// コマンドを実行します。
fn execute_command(command: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
	let (program, args) = command.split_first().unwrap();
	let status = std::process::Command::new(program).args(args).spawn()?.wait()?;
	if !status.success() {
		let code = status.code().unwrap();
		println!("[ERROR] コマンドは {} で終了しました。", code);
		panic!();
	}
	return Ok(());
}

/// wasmer によって実行可能な形式でビルドします。
///
/// これは手元で出力を確認するだけの目的で使用されます。
/// 出力された wasm の main() を呼び出すには、次のように実行します。
///
/// ```sh
/// wasmer main.wasm
/// ```
fn make_wasm_exe() -> Result<(), Box<dyn std::error::Error>> {
	println!("[INFO] build wasm...");
	execute_command(&[
		"zig",
		"build-exe",
		"-O",
		"ReleaseSmall",
		"-target",
		"wasm32-wasi",
		"src/main.zig",
	])?;

	println!("[INFO] lauching wasmer..");
	execute_command(&["cmd.exe", "/C", "wasmer", "main.wasm"])?;

	println!("[INFO] Ok.");
	return Ok(());
}

/// JavaScript から呼び出し可能な wasm を出力します。
fn make_wasm_lib() -> Result<(), Box<dyn std::error::Error>> {
	// JavaScript から呼び出し可能な wasm を出力します。
	println!("[INFO] build wasm...");
	execute_command(&[
		"zig",
		"build-lib",
		"-O",
		"ReleaseSmall",
		"-target",
		"wasm32-wasi",
		"src/main.zig",
		"-dynamic",
		"--export=test1",
		"--export=test2",
		"--export=context",
	])?;

	println!("[INFO] test launching...");
	execute_command(&["cmd.exe", "/C", "node", "loading-test/launch.js"])?;

	println!("[INFO] Ok.");
	return Ok(());
}

/// ファイルをコピーします。
fn copy(left: &str, right: &str) -> Result<(), Box<dyn std::error::Error>> {
	println!("[INFO] COPY> [{}] >> [{}]", left, right);
	std::fs::copy(left, right)?;
	return Ok(());
}

/// wasm を出力してインストールします。
fn make_install() -> Result<(), Box<dyn std::error::Error>> {
	// JavaScript から呼び出し可能な wasm を出力します。
	println!("[INFO] build wasm...");
	execute_command(&[
		"zig",
		"build-lib",
		"-O",
		"ReleaseSmall",
		"-target",
		"wasm32-wasi",
		"src/main.zig",
		"-dynamic",
		"--export=test1",
		"--export=test2",
		"--export=context",
	])?;

	println!("[INFO] mkdir...");
	mkdir("C:\\Inetpub\\wwwroot\\20220821-my-wasm-by-zig")?;

	println!("[INFO] copying...");
	copy(
		"main.wasm",
		"C:\\Inetpub\\wwwroot\\20220821-my-wasm-by-zig\\main.wasm",
	)?;
	copy(
		"WWW\\index.html",
		"C:\\Inetpub\\wwwroot\\20220821-my-wasm-by-zig\\index.html",
	)?;
	copy("WWW\\favicon.ico", "C:\\Inetpub\\wwwroot\\favicon.ico")?;

	// ブラウザーを開く
	execute_command(&[
		"cmd.exe",
		"/C",
		"START",
		"http://localhost/20220821-my-wasm-by-zig",
	])?;

	println!("[INFO] Ok.");
	return Ok(());
}

/// 使用方法を出力します。
fn usage() {
	println!("USAGE:");
	println!("    make --help,    -h      Shows usage.");
	println!("    make --exe,     -e      Make executable wasm bin.");
	println!("    make --lib,     -l      Make js-callable wasm lib.");
	println!("    make --install, -i      Install contets locally.");
	println!();
}

/// Rust アプリケーションのエントリーポイントです。
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = std::env::args().skip(1).collect();
	let arg = if 0 < args.len() { &args[0] } else { "" };
	if arg == "--help" || arg == "-h" {
		// 使用方法を出力します。
		usage();
	} else if arg == "-e" || arg == "--exe" {
		// main.wasm を出力します。
		make_wasm_exe()?;
	} else if arg == "-l" || arg == "--lib" {
		// main.wasm を出力します。
		make_wasm_lib()?;
	} else if arg == "-i" || arg == "--install" {
		// main.wasm を出力して wwwroot 配下に配置します。
		make_install()?;
	} else {
		// 使用方法を出力します。
		usage();
	}

	return Ok(());
}
