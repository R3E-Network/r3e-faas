# Kubernetes Deployment for R3E FaaS Platform

This directory contains Kubernetes manifests for deploying the R3E FaaS platform in a Kubernetes cluster.

## Prerequisites

- Kubernetes cluster (v1.19+)
- kubectl command-line tool
- kustomize command-line tool (optional, kubectl has built-in kustomize support)
- Persistent volume provisioner support in the underlying infrastructure

## Components

The deployment consists of the following components:

- **API Service**: Exposes the R3E FaaS API endpoints
- **Worker Service**: Processes function executions
- **Redis**: Used for caching and message queuing
- **PostgreSQL**: Used for persistent storage of metadata
- **RocksDB Storage**: Used for high-performance key-value storage

## Configuration

The platform is configured using a ConfigMap (`configmap.yaml`). You can modify the following settings:

- Database connection string
- Redis connection string
- Blockchain RPC URLs
- Log level
- Storage type
- Function execution limits

## Deployment

### Using kubectl with kustomize

```bash
# Deploy all resources
kubectl apply -k k8s/

# View deployed resources
kubectl get all -n r3e-faas
```

### Using individual manifests

```bash
# Create namespace
kubectl apply -f k8s/namespace.yaml

# Create storage resources
kubectl apply -f k8s/storage.yaml

# Create ConfigMap
kubectl apply -f k8s/configmap.yaml

# Deploy Redis
kubectl apply -f k8s/redis.yaml

# Deploy PostgreSQL
kubectl apply -f k8s/postgres.yaml

# Deploy API service
kubectl apply -f k8s/api-deployment.yaml
kubectl apply -f k8s/api-service.yaml

# Deploy Worker service
kubectl apply -f k8s/worker-deployment.yaml

# Deploy Ingress
kubectl apply -f k8s/ingress.yaml
```

## Accessing the Platform

Once deployed, the platform can be accessed through the Ingress at:

```
https://api.r3e-faas.example.com
```

You should update the host in `ingress.yaml` to match your domain.

## Monitoring

You can monitor the deployment using standard Kubernetes commands:

```bash
# Check pod status
kubectl get pods -n r3e-faas

# View logs for API service
kubectl logs -n r3e-faas -l app=r3e-api

# View logs for Worker service
kubectl logs -n r3e-faas -l app=r3e-worker
```

## Scaling

You can scale the API and Worker deployments based on your needs:

```bash
# Scale API service
kubectl scale deployment -n r3e-faas r3e-api --replicas=5

# Scale Worker service
kubectl scale deployment -n r3e-faas r3e-worker --replicas=10
```

## Customization

To customize the deployment for your environment:

1. Fork this repository
2. Modify the Kubernetes manifests as needed
3. Apply the modified manifests to your cluster

## Troubleshooting

If you encounter issues with the deployment:

1. Check pod status: `kubectl get pods -n r3e-faas`
2. Check pod logs: `kubectl logs -n r3e-faas <pod-name>`
3. Check events: `kubectl get events -n r3e-faas`
4. Check persistent volume claims: `kubectl get pvc -n r3e-faas`
