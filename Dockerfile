# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM ekidd/rust-musl-builder:nightly-2019-06-08-openssl11 as cargo-build

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/shorty-rs

COPY . .

RUN sudo chown -R rust:rust /usr/src/shorty-rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 movinggauteng && adduser -D -s /bin/sh -u 1000 -G movinggauteng movinggauteng

WORKDIR /home/shorty-rs/bin/

COPY --from=cargo-build /usr/src/shorty-rs/target/x86_64-unknown-linux-musl/release/shorty-rs .

RUN chown movinggauteng:movinggauteng shorty-rs

USER movinggauteng

CMD ["./shorty-rs"]