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

The commands above worked, you can test it yourself, but apparently something is wrong.

The documentation says https://www.jaegertracing.io/docs/1.63/client-features/

```
JAEGER_ENDPOINT defines the URL for reporting spans via HTTP/Thrift.
This setting allows for a deployment mode where spans are submitted directly to the collector.
```

And we are using grpc.


trying to deploy to minikube

(fish)
```
eval $(minikube docker-env)
docker build -t otel-actix-example:latest .
kubectl apply -f deployment.yml
```
