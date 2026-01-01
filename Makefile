BINARY := greeter
PREFIX ?= $(HOME)/.cargo
BINDIR := $(PREFIX)/bin
TARGET := target/x86_64-unknown-linux-gnu/release/$(BINARY)

.PHONY: install build compress clean

build:
	cargo build --release --locked

compress: build
	@if command -v upx >/dev/null 2>&1; then \
		upx --best --lzma $(TARGET); \
	else \
		echo "UPX not found, skipping compression"; \
	fi

install: compress
	install -m 755 $(TARGET) $(BINDIR)/$(BINARY)

uninstall:
	rm -f $(BINDIR)/$(BINARY)

clean:
	cargo clean
