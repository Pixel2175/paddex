all:install

build:
	cargo build --release --manifest-path tools/Cargo.toml

install:build
	mkdir -p bin/
	cp ./tools/target/release/md-to-html ./tools/target/release/template-renderer bin/

clean:
	cargo clean --manifest-path tools/Cargo.toml
	rm -r bin/
