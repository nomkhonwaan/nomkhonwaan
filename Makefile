.PHONY: build dev run test

build:
	deno task build

dev:
	DEBUG=1 deno task start

run:
	deno task preview

test:
	deno test --no-check utils/*_test.ts