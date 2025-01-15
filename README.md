# otel-actix-example

This is a test repo, code extracted from https://github.com/trustification/trustify

```
podman network create jaeger-net
podman compose up
podman build -t otel-actix-example .
podman run -p 8080:8080 --network jaeger-net bee24148ee9d5f2b90964f0857578b2002069038457e7c5bdf01f6bf790ffa08 (image hash here)
```

## minikube with helm 

```
# start
minikube start --cpus 6 --memory 16000 --disk-size 20gb --addons ingress

# config
set NAMESPACE trustify
set APP_DOMAIN .(minikube ip).nip.io
kubectl create ns $NAMESPACE
kubectl config set-context --current --namespace=$NAMESPACE
```

```
# install jaeger

helm upgrade --install --dependency-update -n $NAMESPACE infra charts/infra --values charts/infra/values.yaml --set-string jaeger.allInOne.ingress.hosts[0]=jaeger$APP_DOMAIN --set tracing.enabled=true
Release "infra" does not exist. Installing it now.
coalesce.go:286: warning: cannot overwrite table with non table for infra.opentelemetry-collector.config.receivers.otlp.protocols.grpc (map[endpoint:${env:MY_POD_IP}:4317])
NAME: infra
LAST DEPLOYED: Wed Jan 15 13:27:43 2025
NAMESPACE: trustify
STATUS: deployed
REVISION: 1
TEST SUITE: None
```
```
# build app image within minikube
minikube image build -t otel-actix -f Containerfile .

# Get the jaeger link
kubectl get ingress -n $NAMESPACE -o=jsonpath='{range .items[*]}{.metadata.name}{" -> http://"}{.spec.rules[0].host}{"\n"}{end}'

# Get the collector name
kubectl get svc

# install the app
helm install otel-actix ./charts/app --set collector="http://infra-otelcol:4317"

# port forward
kubectl port-forward svc/otel-actix-service 8080:80

# open a new terminal
curl localhost:8080
```

## :eyes: logs:

```
kubectl logs -l app=otel-actix --tail=100
```
## OTEL Collector

```
podman compose -f podman-compose-otel-col.yaml up
```

Run the app, go to localhost:8080 and we can see the collector logs:

```
collector-1  | Span #143
collector-1  |     Trace ID       : 9bc1552f4fb61efdde43dc28277bd1d4
collector-1  |     Parent ID      : f977fb1974e0ad73
collector-1  |     ID             : c12a79fcb9f3a95c
collector-1  |     Name           : poll
collector-1  |     Kind           : Internal
collector-1  |     Start time     : 2025-01-07 16:05:52.150855739 +0000 UTC
collector-1  |     End time       : 2025-01-07 16:05:52.151399019 +0000 UTC
collector-1  |     Status code    : Unset
collector-1  |     Status message :
collector-1  | Attributes:
collector-1  |      -> code.filepath: Str(/home/heliofrota/.cargo/registry/src/index.crates.io-6f17d22bba15001f/h2-0.4.7/src/proto/connection.rs)
collector-1  |      -> code.namespace: Str(h2::proto::connection)
collector-1  |      -> code.lineno: Int(264)
collector-1  |      -> thread.id: Int(1)
collector-1  |      -> thread.name: Str(main)
collector-1  |      -> busy_ns: Int(536415)
collector-1  |      -> idle_ns: Int(11434)
collector-1  | Events:
collector-1  | SpanEvent #0
collector-1  |      -> Name:
collector-1  |      -> Timestamp: 2025-01-07 16:05:52.150866041 +0000 UTC
collector-1  |      -> DroppedAttributesCount: 0
collector-1  |      -> Attributes::
collector-1  |           -> level: Str(TRACE)
collector-1  |           -> target: Str(h2::proto::connection)
collector-1  |           -> connection.state: Str(Open)
collector-1  |           -> code.filepath: Str(/home/heliofrota/.cargo/registry/src/index.crates.io-6f17d22bba15001f/h2-0.4.7/src/proto/connection.rs)
collector-1  |           -> code.namespace: Str(h2::proto::connection)
collector-1  |           -> code.lineno: Int(268)
collector-1  | SpanEvent #1
collector-1  |      -> Name: poll_complete
collector-1  |      -> Timestamp: 2025-01-07 16:05:52.15113298 +0000 UTC
collector-1  |      -> DroppedAttributesCount: 0
collector-1  |      -> Attributes::
collector-1  |           -> level: Str(TRACE)
collector-1  |           -> target: Str(h2::proto::streams::prioritize)
collector-1  |           -> code.filepath: Str(/home/heliofrota/.cargo/registry/src/index.crates.io-6f17d22bba15001f/h2-0.4.7/src/proto/streams/prioritize.rs)
collector-1  |           -> code.namespace: Str(h2::proto::streams::prioritize)
collector-1  |           -> code.lineno: Int(528)
collector-1  | SpanEvent #2
collector-1  |      -> Name: schedule_pending_open
collector-1  |      -> Timestamp: 2025-01-07 16:05:52.151181488 +0000 UTC
collector-1  |      -> DroppedAttributesCount: 0
collector-1  |      -> Attributes::
collector-1  |           -> level: Str(TRACE)
collector-1  |           -> target: Str(h2::proto::streams::prioritize)
collector-1  |           -> code.filepath: Str(/home/heliofrota/.cargo/registry/src/index.crates.io-6f17d22bba15001f/h2-0.4.7/src/proto/streams/prioritize.rs)
collector-1  |           -> code.namespace: Str(h2::proto::streams::prioritize)
collector-1  |           -> code.lineno: Int(890)
collector-1  |  {"kind": "exporter", "data_type": "traces", "name": "debug"}
```
