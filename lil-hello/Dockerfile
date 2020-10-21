FROM rust:1.31 as builder

WORKDIR .
COPY . .

RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/lil-hello /usr/local/bin/lil-hello

ENTRYPOINT ["lil-hello"]