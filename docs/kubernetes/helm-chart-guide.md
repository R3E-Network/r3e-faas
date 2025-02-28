# Helm Chart Guide for R3E FaaS Platform

This guide provides detailed information about the Helm chart for the R3E FaaS platform, focusing on advanced configuration options, security best practices, and customization.

## Table of Contents

- [Chart Overview](#chart-overview)
- [Security Configuration](#security-configuration)
- [Advanced Configuration](#advanced-configuration)
- [Custom Resource Requirements](#custom-resource-requirements)
- [Integration with External Services](#integration-with-external-services)
- [Multi-Environment Deployment](#multi-environment-deployment)
- [Upgrading and Rollbacks](#upgrading-and-rollbacks)

## Chart Overview

The R3E FaaS Helm chart deploys the following components:

- API Service
- Worker Service
- PostgreSQL Database
- Redis Cache
- Storage Volumes
- Network Policies
- RBAC Resources

## Security Configuration

### Secret Management

The chart is designed to handle sensitive information securely:

1. **Database Credentials**:
   - Never store actual passwords in `values.yaml`
   - Use Kubernetes Secrets or external secret management systems
   - The chart supports referencing existing secrets

```yaml
postgres:
  auth:
    # Reference an existing secret
    existingSecret: "postgres-secret"
    existingSecretKey: "password"
    # Or let Helm generate a random password (only for non-production)
    postgresPassword: null  # Will generate a random password if not provided
```

2. **TLS Configuration**:
   - Enable TLS for all services
   - Use cert-manager for certificate management

```yaml
ingress:
  tls:
    enabled: true
    secretName: "r3e-api-tls"
    # Use cert-manager
    annotations:
      cert-manager.io/cluster-issuer: "letsencrypt-prod"
```

### Pod Security

Configure pod security contexts:

```yaml
api:
  securityContext:
    runAsNonRoot: true
    runAsUser: 1000
    fsGroup: 1000
    capabilities:
      drop: ["ALL"]
```

## Advanced Configuration

### High Availability Setup

For production environments, enable high availability:

```yaml
api:
  replicaCount: 3
  podDisruptionBudget:
    enabled: true
    minAvailable: 2

worker:
  replicaCount: 5
  podDisruptionBudget:
    enabled: true
    minAvailable: 3

redis:
  sentinel:
    enabled: true
  cluster:
    enabled: true
```

### Autoscaling

Enable horizontal pod autoscaling:

```yaml
api:
  autoscaling:
    enabled: true
    minReplicas: 2
    maxReplicas: 10
    targetCPUUtilizationPercentage: 80
    targetMemoryUtilizationPercentage: 80

worker:
  autoscaling:
    enabled: true
    minReplicas: 3
    maxReplicas: 20
    targetCPUUtilizationPercentage: 70
```

## Custom Resource Requirements

Adjust resource requirements based on your workload:

```yaml
api:
  resources:
    limits:
      cpu: 2
      memory: 2Gi
    requests:
      cpu: 500m
      memory: 1Gi

worker:
  resources:
    limits:
      cpu: 4
      memory: 4Gi
    requests:
      cpu: 1
      memory: 2Gi
```

## Integration with External Services

### External Database

Connect to an external PostgreSQL database:

```yaml
postgres:
  enabled: false  # Disable the included PostgreSQL
  external:
    host: "my-external-postgres.example.com"
    port: 5432
    database: "r3e_faas"
    existingSecret: "external-postgres-secret"
```

### External Redis

Connect to an external Redis instance:

```yaml
redis:
  enabled: false  # Disable the included Redis
  external:
    host: "my-external-redis.example.com"
    port: 6379
    existingSecret: "external-redis-secret"
```

## Multi-Environment Deployment

Create environment-specific values files:

1. `values-dev.yaml`:
```yaml
global:
  environment: development
  logLevel: debug

api:
  replicaCount: 1
  resources:
    limits:
      cpu: 500m
      memory: 512Mi
```

2. `values-staging.yaml`:
```yaml
global:
  environment: staging
  logLevel: info

api:
  replicaCount: 2
```

3. `values-prod.yaml`:
```yaml
global:
  environment: production
  logLevel: info

api:
  replicaCount: 3
  resources:
    limits:
      cpu: 2
      memory: 2Gi
```

Deploy to different environments:

```bash
# Development
helm install r3e-faas-dev ./helm/r3e-faas \
  --namespace r3e-faas-dev \
  --values values-dev.yaml

# Staging
helm install r3e-faas-staging ./helm/r3e-faas \
  --namespace r3e-faas-staging \
  --values values-staging.yaml

# Production
helm install r3e-faas-prod ./helm/r3e-faas \
  --namespace r3e-faas-prod \
  --values values-prod.yaml
```

## Upgrading and Rollbacks

### Upgrading

```bash
helm upgrade r3e-faas ./helm/r3e-faas \
  --namespace r3e-faas \
  --values values-prod.yaml
```

### Rollbacks

If an upgrade fails, roll back to the previous release:

```bash
# List releases
helm history r3e-faas --namespace r3e-faas

# Roll back to a specific revision
helm rollback r3e-faas 2 --namespace r3e-faas
```

### Upgrade Strategies

Configure update strategies for zero-downtime upgrades:

```yaml
api:
  updateStrategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 25%
      maxSurge: 25%

worker:
  updateStrategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 25%
      maxSurge: 25%
```
