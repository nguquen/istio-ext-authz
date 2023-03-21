1. Start local k8s with minibute:
```
minikube start --memory 8192
```

2. Install Istio:
```
brew install istioctl
istioctl install --set profile=demo -y
kubectl label namespace default istio-injection=enabled
kubectl apply -f gateway.yaml
```

3. deploy ext-authz service
```
cd ext-authz
eval $(minikube docker-env)
docker build -t ext-authz -f Dockerfile .
kubectl apply -f deployment.yaml
```

4. Build & deploy world service:
```
eval $(minikube docker-env)
cd world
docker build -t world -f Dockerfile .
kubectl apply -f deployment.yaml
```

5. Define the external authorizer
```
kubectl edit configmap istio -n istio-system
...
data:
  mesh: |-
    # Add the following content to define the external authorizers.
    extensionProviders:
    - name: "ext-authz-grpc"
      envoyExtAuthzGrpc:
        service: "ext-authz.default.svc.cluster.local"
        port: "5005"
```

6. Enable with external authorization
```
kubectl apply -f ext-authz.yaml
```

7. Testing commands
```
minikube tunnel
curl http://localhost/world
curl -H "token: secured" http://localhost/world
```
