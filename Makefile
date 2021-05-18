.PHONY: build
build:
	make solver/build
	make client/install
	make client/build

.PHONY: test
test:
	make solver/test

client/%:
	(cd client; make $*)

solver/%:
	(cd solver; make $*)
