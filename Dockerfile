#Stage 1: Build the rust library
FROM rust:1.70 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

#Stage 2: Create a minimal runtime image
FROM debian:buster-slim

WORKDIR /app
COPY --from=builder /user/src/app/target/release/rust-backend .

EXPOSE 8080

CMD ["./rust-backend"]