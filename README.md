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

## minikube

```
eval $(minikube docker-env)
minikube image build -t otel-actix-example -f Containerfile .
kubectl run otel-actix-example --image=otel-actix-example --image-pull-policy=Never --restart=Never
kubectl expose pod otel-actix-example --type=ClusterIP --name=otel-actix-example-service --port=80 --target-port=8080 -n trustify
sed -i "s/[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}/$(minikube ip)/g" ingress.yml
kubectl apply -f ingress.yml
```

Get the links

```
kubectl get ingress -n $NAMESPACE -o=jsonpath='{range .items[*]}{.metadata.name}{" -> http://"}{.spec.rules[0].host}{"\n"}{end}'
```

Click on otel-actix-example and then see the logs

```
kubectl logs otel-actix-example
```

## minikube 

```
minikube start --cpus 6 --memory 16000 --disk-size 20gb --addons ingress
set NAMESPACE trustify
set APP_DOMAIN .(minikube ip).nip.io
kubectl create ns $NAMESPACE
kubectl config set-context --current --namespace=$NAMESPACE
minikube image build -t otel-actix -f Containerfile .
cd hc
helm install otel-actix
```


