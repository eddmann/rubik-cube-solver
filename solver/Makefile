IMAGE = eddmann/rust-wasm-pack:r1.51.0-w0.9.1
DOCKER = docker run --rm -v $(PWD):/app:rw,delegated -w /app

.PHONY: test
test:
	$(DOCKER) $(IMAGE) cargo test

.PHONY: build
build:
	$(DOCKER) $(IMAGE) wasm-pack build

.PHONY: shell
shell:
	@$(DOCKER) -it $(IMAGE) bash
