
run:
	cargo run --  -c 100 -n 11750 https://www.baidu.com

crazy:
	cargo run -- crazy  http://172.16.0.116:8000/healthz

build:
	cargo build --release

install:build
	cp target/release/yc  /usr/local/bin/

test:
	./target/release/yc -c 1 -n 2000  http://localhost:8000/h

ls:
#  统计已连接上的，状态为“established
	netstat -na|grep ESTABLISHED|wc -l