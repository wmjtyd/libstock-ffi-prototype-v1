CARGO = cargo
CC = clang

.PHONY: build build-release
.PHONY: clean clean-dist
.PHONY: create-dist-dir
.PHONY: dist dist-debug

build:
	$(CARGO) build

build-release:
	$(CARGO) build --release

clean: clean-dist
	$(CARGO) clean

dist/libstock.h: create-dist-dir build
	$(CARGO) run --bin gen-header --features=headers dist/libstock.h

dist/libstock-release.h: create-dist-dir build-release
	$(CARGO) run --release --bin gen-header --features=headers dist/libstock-release.h

clean-dist:
	rm -rf dist

create-dist-dir: clean-dist
	mkdir dist

dist-debug: build create-dist-dir dist/libstock.h
	cp ./target/debug/libwmjtyd_libstock* ./dist/

	# Remap the debug symbol.
	sed -i '' 's/target\/debug/dist/g' ./dist/*.d

dist: build-release create-dist-dir dist/libstock-release.h
	cp ./target/release/libwmjtyd_libstock* ./dist/

	# Remove debug symbols.
	rm ./dist/*.d

exp: dist/libwmjtyd_libstock_ffi.dylib
	clang -o ./exp exp.c -L./dist -lwmjtyd_libstock_ffi -std=c11 
