# Rust WASI HTTP Function

A knative func template project for WASI and Cloud Events

Welcome to your new Rust WASI function project! The boilerplate web server is in
[`src/main.rs`](./src/main.rs). It's configured to invoke the `index`
function in [`src/handler.rs`](./src/handler.rs) in response to both
GET and POST requests. You should put your desired behavior inside
that `index` function.

In case you need to configure some resources for your function, you can do that in the [`configure` function](./src/config.rs).

The app will expose three endpoints:

  * `/` Triggers the `index` function, for either GET or POST methods
  * `/health/readiness` The endpoint for a readiness health check **Not Available**
  * `/health/liveness` The endpoint for a liveness health check **Not Available**

## Development

To get started you will need the following

1. [install WASMEdge](https://wasmedge.org/book/en/quick_start/install.html)

2. Install `cargo wasi` with `cargo install cargo-wasi`

3. Set `CARGO_TARGET_WASM32_WASI_RUNNER=wasmedge` in your `.profile`.


Once the setup is complete you should be able to run the following commands successfully
```shell script
cargo wasi build # build your code in debug mode for the wasi target.

cargo wasi build --release # build the optimized version of your *.wasm.

cargo wasi run # execute a binary.

cargo wasi test # run your tests in wasm32-wasi.
# OR
cargo test --target wasm32-wasi -- --nocapture # If you want more verbose output

cargo wasi bench # run your benchmarks in wasm32-wasi.

cargo clippy --target wasm32-wasi # There is no wasi wrapper for clippy as yet
```

Once running, the function is available at <http://localhost:8080> and
the health checks are at <http://localhost:8080/health/readiness> and
<http://localhost:8080/health/liveness>. To POST data to the function,
a utility such as `curl` may be used:

```console
curl -d '{"hello": "world"}' \
  -H'content-type: application/json' \
  http://localhost:8080
```

## build

```
docker build -t {{container_reg}}/{{container_repo}}/{{project-name}} .
```
or
```
podman build -t {{container_reg}}/{{container_repo}}/{{project-name}} .
```

## push

```
docker push {{container_reg}}/{{container_repo}}/{{project-name}}
```
or
```
podman push {{container_reg}}/{{container_repo}}/{{project-name}}
```

## deploy

```
kubectl apply -f service.yaml
```
or
```
oc apply -f service.yaml
```
For OpenShift you may want to consider using the knative service build!

Have fun!
