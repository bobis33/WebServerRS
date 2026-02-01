FROM rust:alpine3.23 AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig curl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release --no-default-features

FROM alpine:3.23

ARG APP_NAME
ENV APP_NAME=${APP_NAME}

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/${APP_NAME} /app/${APP_NAME}

ENV RUST_LOG=info

EXPOSE ${API_PORT}

CMD ["sh", "-c", "./$APP_NAME"]
