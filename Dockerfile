# Use a Rust Docker image as the base image
FROM rust:1.67

# Clone the repository
COPY . ./ink-compiler-be

# Set the working directory
WORKDIR /ink-compiler-be

# Install Rust and compile the project
RUN cargo build --release \
    && rustup toolchain install nightly \
    && rustup default nightly \
    && rustup update \
    && rustup component add rust-src \
    && cargo install --force --locked cargo-contract

# Expose port 8000
EXPOSE 8000

# Set the environment variable
ENV CARGO /usr/local/cargo/bin/cargo

# Run the API
ENTRYPOINT [ "./target/release/compiler-be" ]
