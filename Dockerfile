FROM rust:1.33 AS base

	RUN set -ex ; \
		rustup install nightly ; \
		rustup default nightly ; \
		rustup target add x86_64-unknown-linux-musl ;

	WORKDIR /usr/src/myapp
	COPY . .

	RUN cargo build --release --target x86_64-unknown-linux-musl


FROM alpine

	COPY --from=base \
		/usr/src/myapp/target/x86_64-unknown-linux-musl/release/node_extra_exporter \
		/usr/local/bin/node_extra_exporter


