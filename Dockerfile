FROM rust:1.34 AS rust
FROM alpine:3.9 AS alpine


FROM rust AS base

	RUN rustup target add x86_64-unknown-linux-musl

	WORKDIR /usr/src/myapp
	COPY . .


FROM base AS build

	RUN cargo build --release --target x86_64-unknown-linux-musl


FROM base AS test

	RUN cargo test --target x86_64-unknown-linux-musl


FROM alpine

	COPY --from=build \
		/usr/src/myapp/target/x86_64-unknown-linux-musl/release/node_extra_exporter \
		/usr/local/bin/node_extra_exporter

