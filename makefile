default:
	cargo fmt
	cargo run -- %*

exe:
	cargo run -- --exe

lib:
	cargo runn -- --lib

install:
	@echo NOT IMPLEMENTED
