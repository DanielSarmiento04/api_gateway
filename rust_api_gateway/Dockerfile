FROM rust:1.72

# # Set the working directory
WORKDIR /rust_api_gateway

# Copy the source code
COPY ./src /rust_api_gateway/src

# Copy the Cargo.toml file
COPY ./Cargo.toml /rust_api_gateway/Cargo.toml

# Build the project
RUN cargo build --release

# # Copy the compiled binary
# COPY ./target/release/rust_api /code/rust_api

# EXPOSE the project
EXPOSE 8080

CMD ["/rust_api_gateway/target/release/rust_api_gateway"]