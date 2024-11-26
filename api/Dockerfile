# backend/Dockerfile
FROM rust:1.56 as builder

WORKDIR /app

# Copy the backend source code
COPY . .

# Build the backend
RUN cargo build --release

# Use a lightweight image to run the backend
FROM debian:buster-slim
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8000
CMD ["backend"]