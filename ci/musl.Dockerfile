FROM ekidd/rust-musl-builder:nightly

ADD . ./
RUN sudo chown -R rust:rust .

CMD cargo run -- update-version --pre-release release && cargo test && cargo build --release