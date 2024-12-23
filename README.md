# otel-actix-example

This is a test repo, code extracted from https://github.com/trustification/trustify

```
podman network create jaeger-net
podman compose up
```

```
podman build -t otel-actix-example .
podman run -p 8080:8080 --network jaeger-net bee24148ee9d5f2b90964f0857578b2002069038457e7c5bdf01f6bf790ffa08 (image hash here)
```

trying to deploy to minikube

(fish)
```
eval $(minikube docker-env)
docker build -t otel-actix-example:latest .
kubectl apply -f deployment.yml
```
