FROM rust:1.36.0

RUN rustup component add rustfmt
RUN rustup component add clippy
ADD . ./

CMD cargo fmt -- --check && cargo clippy -- -D warnings