# Builder
FROM rust:1.56 AS builder

COPY ./ ./

RUN cargo build --release --all-features

# Final Image
FROM alpine:latest

COPY --from=builder /target/release/shortenurl ./

RUN adduser -D stan
USER stan

CMD ["web", "shortenurl"]
