.PHONY: build test clean

CARGO := $(shell which cargo 2>/dev/null || echo $(HOME)/.cargo/bin/cargo)

build:
	$(CARGO) build

test:
	$(CARGO) test

clean:
	$(CARGO) clean
	rm -rf target/
