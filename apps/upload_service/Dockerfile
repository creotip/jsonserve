# Use the Rust base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/upload_service

# Copy the Rocket project files into the container
COPY . .

# Build the Rust project
RUN cargo build --release

# Start a new build stage
FROM debian:testing-slim

# Set the working directory inside the container
WORKDIR /usr/src/upload_service

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/upload_service/target/release/upload_service .

# Expose the port that your Rocket server listens on
EXPOSE 8000

# Command to run the Rocket server
CMD ["./upload_service"]
