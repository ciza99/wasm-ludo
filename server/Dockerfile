FROM rust:latest as builder

WORKDIR /usr/src/server

COPY . .

RUN cargo build --release

FROM ubuntu:latest

COPY --from=builder /usr/src/server/target /usr/src/server

CMD ["/usr/src/server/release/server"]
