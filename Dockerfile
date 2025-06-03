FROM rust:1.77 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust_job_server .
COPY cert.pem key.pem ./
EXPOSE 8443
CMD ["./rust_job_server"]

