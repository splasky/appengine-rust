# Use the official Rust image as the base image
FROM rust:1.80 as builder

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final stage
FROM ubuntu:22.04 

RUN apt-get update && apt-get install -y libssl-dev

# Set the working directory
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/appengine-rust .

# Expose the port on which the app will run
EXPOSE 8080

# Set the entry point to the application binary
CMD ["./appengine-rust"]
