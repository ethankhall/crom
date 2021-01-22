FROM rust:1.46 as builder

RUN USER=root cargo new --bin crom
WORKDIR /crom
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/crom*
RUN cargo run --release -- write-version next-release
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /crom/target/release/crom /usr/bin/crom

WORKDIR /target

ENTRYPOINT ["/usr/bin/crom"]