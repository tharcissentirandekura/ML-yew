# frontend/Dockerfile
FROM rust:1.56 as builder

WORKDIR /app

# Install Trunk
RUN cargo install trunk wasm-bindgen-cli

# Copy the frontend source code
COPY . .

# Build the frontend
RUN trunk build --release

# Use a lightweight web server to serve the built files
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]