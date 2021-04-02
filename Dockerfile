FROM rust:1.51 as builder
WORKDIR /usr/src/stellar
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
COPY --from=builder /usr/src/steller/target/release/steller /usr/bin/steller
CMD [ "steller" ]
ENTRYPOINT [ "steller" ]