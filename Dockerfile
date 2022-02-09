####################################################################################################
## Builder
####################################################################################################
FROM rust:1.56 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release --all-features

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myapp

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/shortenurl ./

RUN ls

CMD ["ls && /app/shortenurl"]
