FROM rust:alpine
workdir /app

COPY ./* ./
RUN cargo build -r
CMD ["./target/release/server"]
