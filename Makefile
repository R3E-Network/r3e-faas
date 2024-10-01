# Copyright @ 2023 - 2024, R3E Network
# All Rights Reserved

.PHONY: all
all:
	cargo build --release

# musl
.PHONY: musl
musl:
	cargo build --release --target x86_64-unknown-linux-musl