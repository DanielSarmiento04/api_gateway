FROM rust:1.72

# # Set the working directory
WORKDIR /rust_api

# Copy the source code
COPY ./src /rust_api/src

# Copy the Cargo.toml file
COPY ./Cargo.toml /rust_api/Cargo.toml

# Build the project
RUN cargo build --release

# # Copy the compiled binary
# COPY ./target/release/rust_api /code/rust_api

# EXPOSE the project
EXPOSE 8080

CMD ["/rust_api/target/release/rust_api"]