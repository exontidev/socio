# build stage
FROM rust:1.88 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release


# runtime stage (минимальный образ)
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend

EXPOSE 8080

CMD ["./backend"]