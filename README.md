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

# install jaeger
helm upgrade --install --dependency-update -n $NAMESPACE infra charts/infra --values charts/infra/values.yaml --set-string jaeger.allInOne.ingress.hosts[0]=j
aeger$APP_DOMAIN --set tracing.enabled=true

# build app image within minikube
minikube image build -t otel-actix -f Containerfile .

# Get the jaeger link
kubectl get ingress -n $NAMESPACE -o=jsonpath='{range .items[*]}{.metadata.name}{" -> http://"}{.spec.rules[0].host}{"\n"}{end}'

# install the app
helm install otel-actix ./charts/app --set otlpEndpoint="http://jaeger.192.168.39.78.nip.io"

# port forward
kubectl port-forward svc/otel-actix-service 8080:80

# open a new terminal
curl localhost:8080
```

## :eyes: logs:

```
kubectl logs -l app=otel-actix --tail=100
```

