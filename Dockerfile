FROM rust:1.51 as builder
WORKDIR /usr/src/catsh
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
COPY --from=builder /usr/src/catsh/target/release/catsh /usr/bin
CMD [ "catsh", "start" ]
ENTRYPOINT [ "catsh" ]