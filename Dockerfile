FROM rust:1.61-alpine3.16 as builder
RUN apk update
RUN apk add libc-dev openssl-dev openssl github-cli

RUN USER=root cargo new --bin crom
WORKDIR /crom
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release --features gh-cli
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/crom*
RUN cargo run --release --features gh-cli -- write-version next-release
RUN cargo run --release  --features gh-cli -- gh --help
RUN cargo build --release --features gh-cli

FROM alpine:3.16 as release

RUN apk add --no-cache ca-certificates openssl tzdata github-cli
COPY --from=builder /crom/target/release/crom /usr/bin/crom

WORKDIR /target

ENTRYPOINT ["/usr/bin/crom"]

# Run basic test
RUN crom --help && crom gh --help
