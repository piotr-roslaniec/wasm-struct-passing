SHELL = /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:

build: pkg

pkg: src
	# Build for both targets
	wasm-pack build -t bundler -d pkg/pkg-bundler
	wasm-pack build -t nodejs -d pkg/pkg-node

	# Types are duplicated, clean them up to avoid confusion
	mv pkg/pkg-node/wasm_struct_passing.d.ts pkg/
	rm pkg/pkg-bundler/wasm_struct_passing.d.ts

	# Copy template
	cp package.template.json pkg/package.json
	cp LICENSE pkg/LICENSE


.PHONY: clean

clean:
	rm -rf pkg
