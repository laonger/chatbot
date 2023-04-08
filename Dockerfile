FROM rust:alpine
workdir /app

expose 7878

COPY . .
RUN cargo build -r
CMD ["./target/release/server"]
