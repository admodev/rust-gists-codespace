all: build

.PHONY: deps
deps:
	@echo "Dependencies downloaded!"

.PHONY: build
build:
	cargo build
	@echo "Build done!"

clean:
	cargo clean
	@echo "Project is now clean! ✨"
