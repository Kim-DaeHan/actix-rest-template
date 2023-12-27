# Set the base image to the official Rust image
FROM rust:latest

# Create a new directory to work in
WORKDIR /usr/src/my_actix_app

# Copy the source code into the container
COPY . .

# Build the dependencies (this step is separate to make use of Docker cache)
RUN cargo build --release

# Expose the required port
EXPOSE 8080

# Command to run the application
CMD ["cargo", "run"]
