# Builder
FROM rust:1.56 AS builder

COPY ./ ./

RUN cargo build --release --all-features

CMD ["./target/release/shortenurl"]
