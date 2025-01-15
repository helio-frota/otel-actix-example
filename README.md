# otel-actix-example

This is a test repo, code extracted from https://github.com/trustification/trustify

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
