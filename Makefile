CARGO_TARGET=specs/x86_64-amr_os.json


all: build

build:
	cargo build --target $(CARGO_TARGET)
