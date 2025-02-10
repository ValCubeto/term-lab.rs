build-unix:
	cargo build --release

build-windows:
	cargo build --release --target x86_64-pc-windows-gnu

run-unix:
	cargo build --release
	./target/release/term-lab $(filter-out $@,$(MAKECMDGOALS))

run-windows:
	cargo build --release --target x86_64-pc-windows-gnu
	wine ./target/x86_64-pc-windows-gnu/release/term-lab.exe $(filter-out $@,$(MAKECMDGOALS))
