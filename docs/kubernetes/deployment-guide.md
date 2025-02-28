# Kubernetes Deployment Guide for R3E FaaS Platform

This guide provides detailed instructions for deploying the R3E FaaS platform on Kubernetes, with a focus on security best practices, especially for managing sensitive information like database credentials.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Deployment Options](#deployment-options)
- [Security Best Practices](#security-best-practices)
- [Basic Deployment](#basic-deployment)
- [Advanced Deployment with Helm](#advanced-deployment-with-helm)
- [Monitoring and Maintenance](#monitoring-and-maintenance)
- [Troubleshooting](#troubleshooting)

## Prerequisites

Before deploying the R3E FaaS platform on Kubernetes, ensure you have:

- Kubernetes cluster (v1.19+)
- kubectl CLI tool configured to access your cluster
- Helm v3+ (for Helm-based deployments)
- Access to container registries for R3E FaaS images
- Persistent storage provisioner in your cluster

## Deployment Options

The R3E FaaS platform can be deployed using:

1. **Basic Kubernetes manifests** - Located in the `k8s/` directory
2. **Helm charts** - Located in the `helm/r3e-faas/` directory

## Security Best Practices

### Managing Sensitive Information

The R3E FaaS platform requires several sensitive credentials, including database passwords. **Never store these credentials directly in your YAML files or version control.**

Instead, use Kubernetes Secrets:

```bash
# Create a secret for database credentials
kubectl create secret generic postgres-secret \
  --from-literal=username=postgres \
  --from-literal=password=$(openssl rand -base64 20) \
  --from-literal=database=r3e_faas
```

### Secret Management Best Practices

1. **Use external secret management systems** when possible:
   - AWS Secrets Manager with External Secrets Operator
   - HashiCorp Vault
   - Azure Key Vault
   - Google Secret Manager

2. **Rotate secrets regularly**:
   - Implement automated secret rotation
   - Use short-lived credentials when possible

3. **Limit access to secrets**:
   - Use RBAC to restrict which pods can access which secrets
   - Implement network policies to restrict pod-to-pod communication

4. **Encrypt secrets at rest**:
   - Enable etcd encryption
   - Use sealed secrets for GitOps workflows

## Basic Deployment

To deploy using basic Kubernetes manifests:

1. Create the namespace:
   ```bash
   kubectl create namespace r3e-faas
   ```

2. Create secrets (never commit these to version control):
   ```bash
   kubectl create secret generic postgres-secret -n r3e-faas \
     --from-literal=host=postgres \
     --from-literal=port=5432 \
     --from-literal=username=postgres \
     --from-literal=password=$(openssl rand -base64 20) \
     --from-literal=database=r3e_faas
   ```

3. Apply the manifests:
   ```bash
   kubectl apply -f k8s/postgres-pvc.yaml -n r3e-faas
   kubectl apply -f k8s/postgres.yaml -n r3e-faas
   kubectl apply -f k8s/redis.yaml -n r3e-faas
   kubectl apply -f k8s/api-deployment.yaml -n r3e-faas
   kubectl apply -f k8s/worker-deployment.yaml -n r3e-faas
   kubectl apply -f k8s/api-service.yaml -n r3e-faas
   ```

## Advanced Deployment with Helm

Helm provides a more flexible and maintainable way to deploy the R3E FaaS platform.

### Preparing Values

1. Create a `values.override.yaml` file (do not commit to version control):

```yaml
global:
  environment: production
  logLevel: info

postgres:
  auth:
    # Do NOT include actual passwords here
    # The password will be provided via external secrets or generated during installation
    existingSecret: "postgres-secret"
    existingSecretKey: "password"
```

2. Create the required secrets:

```bash
kubectl create namespace r3e-faas

# Generate a secure password
DB_PASSWORD=$(openssl rand -base64 20)

# Create the secret
kubectl create secret generic postgres-secret -n r3e-faas \
  --from-literal=password=$DB_PASSWORD
```

3. Install the Helm chart:

```bash
helm install r3e-faas ./helm/r3e-faas \
  --namespace r3e-faas \
  --values values.override.yaml
```

### Upgrading

To upgrade an existing deployment:

```bash
helm upgrade r3e-faas ./helm/r3e-faas \
  --namespace r3e-faas \
  --values values.override.yaml
```

## Monitoring and Maintenance

### Health Checks

The R3E FaaS platform includes health and readiness endpoints:

- API Service: `/health` and `/ready`
- Worker Service: `/health` and `/ready`

### Scaling

Scale components based on your workload:

```bash
kubectl scale deployment r3e-faas-api --replicas=3 -n r3e-faas
kubectl scale deployment r3e-faas-worker --replicas=5 -n r3e-faas
```

## Troubleshooting

### Common Issues

1. **Database connection failures**:
   - Verify secrets are correctly mounted
   - Check network policies allow pod-to-database communication
   - Ensure database service is running

2. **Worker scaling issues**:
   - Check resource constraints
   - Verify Redis connection for task distribution

3. **Security-related issues**:
   - Review pod security contexts
   - Check RBAC permissions
   - Verify secret mounting

### Logs

Access logs for troubleshooting:

```bash
# API logs
kubectl logs -f deployment/r3e-faas-api -n r3e-faas

# Worker logs
kubectl logs -f deployment/r3e-faas-worker -n r3e-faas

# Database logs
kubectl logs -f deployment/postgres -n r3e-faas
```
