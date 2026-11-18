PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin

.PHONY: all build release install test test-tmp check fmt clean

all: release

build:
	cargo build

release:
	cargo build --release

install: release
	install -d "$(DESTDIR)$(BINDIR)"
	install -m 755 target/release/srcset "$(DESTDIR)$(BINDIR)/srcset"

test: test-tmp

test-tmp:
	@echo 'Converting examples/simple/pnp-bellcm-10100.tif to JPEG in /tmp/srcset-make-test'
	cargo run --release -- examples/simple/pnp-bellcm-10100.tif -t jpg -o /tmp/srcset-make-test -v
	@echo 'Converting examples/simple/pnp-fsa-8c02000.tif to PNG in /tmp/srcset-make-test'
	cargo run --release -- examples/simple/pnp-fsa-8c02000.tif -t png -o /tmp/srcset-make-test -v

check:
	cargo check

fmt:
	cargo fmt

clean:
	cargo clean
