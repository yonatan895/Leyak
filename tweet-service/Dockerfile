# Stage 1: Build
FROM rust:1.84 as builder
WORKDIR /app

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV GIT_HTTP_VERSION=HTTP/1.1
ENV GIT_SSL_NO_VERIFY=true

RUN git config --global http.version HTTP/1.1 && \
    git config --global http.postBuffer 524288000 && \
    git config --global http.sslVerify false

# Copy Cargo configuration.
COPY .cargo/config.toml .cargo/config.toml

# Copy the shared common directory.
COPY common/ common/

# Copy the tweet-service source code.
COPY tweet-service/ tweet-service/

# Set the working directory to tweet-service.
WORKDIR /app/tweet-service

# Pre-fetch dependencies.
RUN cargo fetch

# Build the tweet-service binary.
RUN cargo build --release --bin tweet-service

# Stage 2: Create the runtime image.
FROM debian:bookworm-slim
COPY --from=builder /app/tweet-service/target/release/tweet-service /usr/local/bin/tweet-service
EXPOSE 8002
CMD ["tweet-service"]

