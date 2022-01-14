
run:
	cargo run -- -c 3 -n 2

build:
	cargo build --release

test:
	./target/release/yc -c 3 -n 2000