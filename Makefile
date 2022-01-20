
run:
	cargo run -- -c 3 -n 2 http://localhost:8000/h

build:
	cargo build --release

test:
	./target/release/yc -c 3 -n 2000