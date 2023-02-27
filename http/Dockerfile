FROM scratch

COPY target/wasm32-wasi/release/{{project-name}}.wasm /

# Run the web service on container startup.
CMD ["/{{project-name}}.wasm"]