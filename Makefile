
run:
	cargo run -- -c 2 -n 100 http://qq.com

build:
	cargo build --release

install:build
	cp target/release/yc  /usr/local/bin/

test:
	./target/release/yc -c 1 -n 2000  http://localhost:8000/h