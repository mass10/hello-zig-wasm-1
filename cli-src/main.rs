fn mkdir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	let pt = std::path::Path::new(path);
	if pt.is_dir() {
		return Ok(());
	}
	println!("MKDIR> [{}]", path);
	std::fs::create_dir(path)?;
	return Ok(());
}

fn execute_command(command: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
	let (program, args) = command.split_first().unwrap();
	let status = std::process::Command::new(program).args(args).spawn()?.wait()?;
	if !status.success() {
		let code = status.code().unwrap();
		println!("ERROR: コマンドは {} で終了しました。", code);
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
fn make_exe() -> Result<(), Box<dyn std::error::Error>> {
	println!("build...");

	execute_command(&[
		"zig",
		"build-exe",
		"-O",
		"ReleaseSmall",
		"-target",
		"wasm32-wasi",
		"src/main.zig",
	])?;

	println!("Ok.");
	return Ok(());
}

/// JavaScript から呼び出し可能な wasm を出力します。
fn make_wasm() -> Result<(), Box<dyn std::error::Error>> {
	println!("build wasm...");

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
	])?;

	println!("Ok.");
	return Ok(());
}

/// ファイルをコピーします。
fn copy(left: &str, right: &str) -> Result<(), Box<dyn std::error::Error>> {
	println!("COPY> [{}] >> [{}]", left, right);
	std::fs::copy(left, right)?;
	return Ok(());
}

/// wasm を出力してインストールします。
fn make_install() -> Result<(), Box<dyn std::error::Error>> {
	make_wasm()?;

	println!("mkdir...");
	mkdir("C:\\Inetpub\\wwwroot\\20220821-my-wasm-by-zig")?;

	println!("copy...");
	copy(
		"main.wasm",
		"C:\\Inetpub\\wwwroot\\20220821-my-wasm-by-zig\\main.wasm",
	)?;
	copy(
		"WWW\\launch.js",
		"C:\\Inetpub\\wwwroot\\20220821-my-wasm-by-zig\\launch.js",
	)?;
	copy(
		"WWW\\index.html",
		"C:\\Inetpub\\wwwroot\\20220821-my-wasm-by-zig\\index.html",
	)?;
	copy(
		"WWW\\favicon.ico",
		"C:\\Inetpub\\wwwroot\\20220821-my-wasm-by-zig\\favicon.ico",
	)?;

	println!("Ok.");
	return Ok(());
}

/// エントリーポイントです。
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		// wasmer によって呼び出し可能な main.wasm を出力します。
		make_exe()?;
	} else if args.len() == 1 && args[0] == "w" {
		// main.wasm を出力します。
		make_wasm()?;
	} else if args.len() == 1 && args[0] == "i" {
		// main.wasm を出力して wwwroot 配下に配置します。
		make_install()?;
	} else {
		panic!("make w/i");
	}
	return Ok(());
}
