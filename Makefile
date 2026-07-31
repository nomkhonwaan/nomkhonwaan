.PHONY: build dev run

build:
	deno task build

dev:
	DEBUG=1 deno task start

run:
	deno task preview