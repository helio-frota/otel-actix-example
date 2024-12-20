# otel-actix-example

This is a test repo, code extracted from https://github.com/trustification/trustify

```
podman network create jaeger-net
podman compose up
```

```
podman build -t otel-actix-example .
podman run --rm --network jaeger-net -e JAEGER_ENDPOINT=http://jaeger:4317 -p 8080:8080 image-hash-here
```
