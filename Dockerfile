FROM rust:alpine
workdir /app

expose 7878

COPY . .

RUN apk add --no-cache openssl-dev musl-dev

RUN cargo build -r
CMD ["./target/release/server"]
