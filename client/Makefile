IMAGE = node:14.15-alpine
DOCKER = docker run --rm -v $(PWD):/app:rw,delegated -v $(PWD)/../solver/:/solver:rw,delegated -w /app

.PHONY: install
install:
	@$(DOCKER) $(IMAGE) yarn install

.PHONY: start
start:
	@$(DOCKER) -it -p 3000:3000 $(IMAGE) yarn start

.PHONY: build
build:
	$(DOCKER) $(IMAGE) yarn build

.PHONY: shell
shell:
	@$(DOCKER) -it $(IMAGE) sh
