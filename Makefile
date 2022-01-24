
run:
	cargo run --  -t 3 -c 10 -n 100000 http://localhost:8000/h

build:
	cargo build --release

install:build
	cp target/release/yc  /usr/local/bin/

test:
	./target/release/yc -c 1 -n 2000  http://localhost:8000/h