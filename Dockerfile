FROM ekidd/rust-musl-builder as builder

RUN cargo new --bin hello-world

WORKDIR ./hello-world

RUN cargo build --release


FROM scratch

COPY --from=builder /home/rust/src/hello-world/target/x86_64-unknown-linux-musl/release/hello-world hello-world

CMD [ "./hello-world" ]