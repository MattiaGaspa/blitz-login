FROM rust:1.85-slim-bookworm AS builder
WORKDIR /usr/src/blitz-login
COPY . .
RUN cargo install --profile release --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get upgrade -y
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/blitz-login /usr/local/bin/blitz-login
RUN mkdir -p /etc/blitz-login
COPY --from=builder /usr/src/blitz-login/config/configuration.yaml /etc/blitz-login/configuration.yaml
CMD ["blitz-login"]