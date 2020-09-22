FROM ekidd/rust-musl-builder:1.46.0

ADD . ./
RUN sudo chown -R rust:rust .

CMD cargo run --release -- update-version --pre-release release && cargo test --release && cargo build --release