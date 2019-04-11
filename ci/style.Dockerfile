FROM rustlang/rust:nightly

RUN rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu
RUN cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy
ADD . ./

CMD cargo fmt -- --check && cargo clippy -- -D warnings