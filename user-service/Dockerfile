# Stage 1: Build
FROM rust:1.84 as builder
WORKDIR /app

# Set environment variables to force Cargo to use the Git CLI and HTTP/1.1.
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV GIT_HTTP_VERSION=HTTP/1.1
ENV GIT_SSL_NO_VERIFY=true

# Configure Git globally to use HTTP/1.1 and increase the postBuffer.
RUN git config --global http.version HTTP/1.1 && \
    git config --global http.postBuffer 524288000 && \
    git config --global http.sslVerify false

# Copy the Cargo configuration from the repository root.
COPY .cargo/config.toml .cargo/config.toml

# Copy the shared "common" crate.
COPY common/ common/

# Copy the user-service source code.
COPY user-service/ user-service/

# Change working directory to the user-service folder.
WORKDIR /app/user-service

# Pre-fetch dependencies (this caches the crates.io index and dependencies).
RUN cargo fetch

# Build the user-service binary.
RUN cargo build --release --bin user-service

# Stage 2: Create a minimal runtime image.
FROM debian:buster-slim
COPY --from=builder /app/user-service/target/release/user-service /usr/local/bin/user-service
EXPOSE 8001
CMD ["user-service"]
