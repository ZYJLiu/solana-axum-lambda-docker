# Docker build and run commands for reference
# docker buildx build --platform linux/amd64 -t testing:v0 --load .
# docker run --platform linux/amd64 -d -p 8080:8080 testing:v0

########################### BASE DEPENDENCIES INSTALLATION ######################
FROM ubuntu:22.04 as base

ARG DEBIAN_FRONTEND="noninteractive"
ENV HOME="/home/appuser"

# Install build dependencies
RUN apt-get update -qq && apt-get install -qq \
    build-essential curl pkg-config libssl-dev libudev-dev

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="${HOME}/.cargo/bin:${PATH}"

############################## BUILD SERVER BINARY #############################
FROM ghcr.io/cargo-lambda/cargo-lambda:latest as build

WORKDIR /build
COPY Cargo.* .
COPY src src
RUN cargo lambda build

################################# START SERVER #################################
FROM base

# Create a non-root user and set appropriate permissions
RUN groupadd -r -g 1000 appuser && \
    useradd -r -u 1000 -g appuser -m -d /home/appuser appuser

# Install Solana as root and then set appropriate permissions
ARG SOLANA_VERSION="1.18.8"
RUN mkdir -p /home/appuser/.local/share/solana && \
    curl -sSfL https://release.solana.com/v${SOLANA_VERSION}/install | sh && \
    chown -R appuser:appuser /home/appuser/.local/share/solana && \
    chmod -R 755 /home/appuser/.local/share/solana
ENV PATH="/home/appuser/.local/share/solana/install/active_release/bin:${PATH}"

# Nope not working, Lambda readonly file system
# Read-only file system (os error 30)

# Copy programs dir and set permissions
COPY programs /home/appuser/programs
# Build the default program
RUN cargo build-sbf --manifest-path /home/appuser/programs/Cargo.toml
RUN chown -R appuser:appuser /home/appuser/programs
RUN chmod -R 755 /home/appuser/programs

# Set ownership and permissions for the .cargo directory
RUN chown -R appuser:appuser /home/appuser/.cargo
RUN chmod -R 755 /home/appuser/.cargo

# Copy entrypoint script as root and set permissions
COPY copy.sh /home/appuser/copy.sh
RUN chmod +x /home/appuser/copy.sh

# # Set CARGO_TARGET_DIR to a temporary directory for Lambda
# ENV CARGO_TARGET_DIR=/tmp/target

# Switch to non-root user
USER appuser
WORKDIR /home/appuser

# Copy the built binary from the build stage
COPY --from=build /build/target/lambda/axum-solana/bootstrap .

# # Start server
# CMD ["./bootstrap"]

# Start server
CMD ["/home/appuser/copy.sh"]

EXPOSE 8080