# Copyright @ 2023 - 2024, R3E Network
# All Rights Reserved

.PHONY: all
all:
	cargo build --release
	mv target/release/r3e-faas ./r3e-faas

# musl
.PHONY: musl
musl:
	cargo build --release --target x86_64-unknown-linux-musl
	mv target/x86_64-unknown-linux-musl/release/r3e-faas ./r3e-faas

.PHONY: test
test:
	cargo test -- --nocapture