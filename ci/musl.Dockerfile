FROM ekidd/rust-musl-builder:1.36.0

ADD . ./
RUN sudo chown -R rust:rust .

CMD cargo run -- update-version --pre-release release && cargo test && cargo build --release