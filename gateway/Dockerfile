FROM rust:1.68 as builder
WORKDIR /app
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV GIT_HTTP_VERSION=HTTP/1.1
ENV GIT_SSL_NO_VERIFY=true
RUN git config --global http.version HTTP/1.1 && \
    git config --global http.postBuffer 524288000 && \
    git config --global http.sslVerify false
COPY .cargo/config.toml .cargo/config.toml
COPY common/ common/
COPY gateway/ gateway/
WORKDIR /app/gateway
RUN cargo fetch
RUN cargo build --release --bin gateway
FROM debian:buster-slim
COPY --from=builder /app/gateway/target/release/gateway /usr/local/bin/gateway
EXPOSE 8000
CMD ["gateway"]

