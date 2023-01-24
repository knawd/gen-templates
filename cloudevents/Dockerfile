FROM rust:1.66.0 as builder

RUN rustup target add wasm32-wasi

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release --target wasm32-wasi

FROM scratch

COPY --from=builder /usr/src/app/target/wasm32-wasi/release/{{project-name}}.wasm /

# Run the web service on container startup.
CMD ["/{{project-name}}.wasm"]