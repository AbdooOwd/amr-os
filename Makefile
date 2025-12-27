all: build

build:
	cargo bootimage

run: build
	cargo run

clean:
	cargo clean
