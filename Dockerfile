FROM rust:1.51 as builder
WORKDIR /usr/src/stellar
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
COPY --from=builder /usr/src/stellar/target/release/stellar /usr/bin
CMD [ "stellar" ]
ENTRYPOINT [ "stellar" ]