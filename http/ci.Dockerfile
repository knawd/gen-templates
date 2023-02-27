FROM quay.io/knawd/rust-base:1.67.0 as builder

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release --target wasm32-wasi

RUN wasmedgec target/wasm32-wasi/release/{{project-name}}.wasm {{project-name}}.wasm

FROM scratch

COPY --from=builder /usr/src/app/target/wasm32-wasi/release/{{project-name}}.wasm /

# Run the web service on container startup.
CMD ["/{{project-name}}.wasm"]