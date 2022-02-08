# Builder
FROM rust:1.56 AS builder

WORKDIR /app

COPY ./ ./

RUN cargo build --release --all-features

# Final Image
FROM alpine:latest

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

COPY --from=builder /app/target/release/shortenurl ./

RUN adduser -D stan
USER stan

CMD ["/app/shortenurl"]
