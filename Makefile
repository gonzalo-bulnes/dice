define RUST_MUSL_CMD
	docker run --rm -it \
	--user "$$(id -u)":"$$(id -g)" \
	--volume "$$(pwd)":/home/rust/src \
	ekidd/rust-musl-builder
endef

.PHONY: all
all: clean build

.PHONY: build
build: target/release/dice-linux-amd64

.PHONY: clean
clean:
	@rm -f target/release/dice-linux-amd64

target/release/dice-linux-amd64: src/*.rs src/**/*.rs
	@${RUST_MUSL_CMD} \
		cargo build --release
	@mv target/x86_64-unknown-linux-musl/release/dice target/release/dice-linux-amd64

.PHONY: test
test:
	${RUST_MUSL_CMD} \
		cargo test
