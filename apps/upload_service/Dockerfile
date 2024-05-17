# Base stage for compiling source code
FROM rust:latest as builder
WORKDIR /usr/src/upload_service
COPY . .

RUN cargo build --release

# Development image
FROM rust:latest as dev
WORKDIR /usr/src/upload_service
ARG ENV=dev
COPY --from=builder /usr/src/upload_service/target /usr/src/upload_service/target
COPY config.${ENV}.toml /usr/src/upload_service/config.toml
COPY . .
RUN cargo install cargo-watch
RUN useradd -m -d /home/rustuser rustuser && \
    chown -R rustuser:rustuser /usr/src/upload_service && \
    chmod -R 755 /usr/src/upload_service && \
    chmod 664 /usr/src/upload_service/*.toml
RUN mkdir -p /usr/local/cargo && \
    chown -R rustuser:rustuser /usr/local/cargo    
USER rustuser
CMD ["cargo", "watch", "-x", "run"]

# Production image
FROM debian:testing-slim as prod
WORKDIR /usr/src/upload_service
ARG ENV=prod
COPY --from=builder /usr/src/upload_service/target/release/upload_service .
COPY config.${ENV}.toml /usr/src/upload_service/config.toml

RUN useradd -m -d /home/uploadserviceuser uploadserviceuser && \
    chown -R uploadserviceuser:uploadserviceuser /usr/src/upload_service && \
    chmod -R 755 /usr/src/upload_service && \
    chmod 664 /usr/src/upload_service/*.toml
RUN mkdir -p /usr/local/cargo && \
    chown -R uploadserviceuser:uploadserviceuser /usr/local/cargo   

USER uploadserviceuser
EXPOSE 8000
CMD ["./upload_service"]
