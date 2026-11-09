# Kubernetes with Helm

## Install

```sh
helm upgrade --install pixie infrastructure/helm/pixie \
  --namespace pixie --create-namespace
```

## Check

```sh
kubectl -n pixie port-forward service/pixie-pixie 8080:8080
curl http://localhost:8080/hello
```
