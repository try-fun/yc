
run:
	cargo run -- -c 10 -n 100 http://localhost:8000/h

build:
	cargo build --release

test:
	./target/release/yc -c 1 -n 2000  http://localhost:8000/h