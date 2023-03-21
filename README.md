1. Define the external authorizer
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

2. Enable with external authorization
```
kubectl apply -f ext-authz.yaml
```

3. Testing commands
```
minikube tunnel
curl http://localhost/world
curl -H "token: secured" http://localhost/world
```
