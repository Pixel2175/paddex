all:install

build:
	cargo build --release --manifest-path tools/Cargo.toml

install:build
	mkdir -p bin/
	pip show pywebview >/dev/null || pip install pywebview --break-system-packages
	pip show watchdog >/dev/null || pip install watchdog --break-system-packages
	pip show gi >/dev/null || pip show qtpy >/dev/null || pip install PyGObject --break-system-packages
	cp ./tools/target/release/md-to-html ./tools/target/release/template-renderer bin/

clean:
	cargo clean --manifest-path tools/Cargo.toml
