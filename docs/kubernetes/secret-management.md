# Secret Management Guide for R3E FaaS Platform

This guide provides comprehensive information on managing secrets securely in the R3E FaaS platform when deployed on Kubernetes.

## Table of Contents

- [Introduction](#introduction)
- [Types of Secrets](#types-of-secrets)
- [Secret Management Strategies](#secret-management-strategies)
- [Kubernetes Secrets](#kubernetes-secrets)
- [External Secret Management](#external-secret-management)
- [Secret Rotation](#secret-rotation)
- [Security Best Practices](#security-best-practices)
- [Monitoring and Auditing](#monitoring-and-auditing)

## Introduction

The R3E FaaS platform requires various secrets for secure operation, including database credentials, API keys, and encryption keys. Proper management of these secrets is critical for maintaining the security of the platform.

## Types of Secrets

The R3E FaaS platform uses several types of secrets:

1. **Database Credentials**:
   - PostgreSQL username and password
   - Redis authentication (if enabled)

2. **API Keys**:
   - Blockchain RPC endpoints
   - External service integrations

3. **Encryption Keys**:
   - TEE attestation keys
   - FHE encryption keys
   - ZK proving keys

4. **User Secrets**:
   - User-provided secrets for functions
   - Wallet private keys

## Secret Management Strategies

### Never in Version Control

**IMPORTANT**: Never store actual secrets in files that are committed to version control. This includes:

- Kubernetes YAML files
- Helm values files
- Docker Compose files
- Configuration files

### Environment-Specific Secrets

Use different secrets for different environments:

- Development
- Staging
- Production

## Kubernetes Secrets

### Basic Secret Creation

Create Kubernetes secrets using kubectl:

```bash
# Create a secret for database credentials
kubectl create secret generic postgres-secret \
  --from-literal=username=postgres \
  --from-literal=password=$(openssl rand -base64 20) \
  --from-literal=database=r3e_faas
```

### Using Secrets in Deployments

Reference secrets in your deployment manifests:

```yaml
env:
  - name: DB_USERNAME
    valueFrom:
      secretKeyRef:
        name: postgres-secret
        key: username
  - name: DB_PASSWORD
    valueFrom:
      secretKeyRef:
        name: postgres-secret
        key: password
```

### Secret Volumes

Mount secrets as files:

```yaml
volumes:
  - name: secret-volume
    secret:
      secretName: postgres-secret
volumeMounts:
  - name: secret-volume
    mountPath: /etc/secrets
    readOnly: true
```

## External Secret Management

For production environments, use external secret management systems:

### AWS Secrets Manager

1. Install the AWS Secrets Manager operator:
```bash
helm repo add external-secrets https://charts.external-secrets.io
helm install external-secrets external-secrets/external-secrets
```

2. Create a SecretStore:
```yaml
apiVersion: external-secrets.io/v1beta1
kind: SecretStore
metadata:
  name: aws-secretsmanager
spec:
  provider:
    aws:
      service: SecretsManager
      region: us-east-1
      auth:
        secretRef:
          accessKeyIDSecretRef:
            name: aws-credentials
            key: access-key-id
          secretAccessKeySecretRef:
            name: aws-credentials
            key: secret-access-key
```

3. Create an ExternalSecret:
```yaml
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: postgres-credentials
spec:
  refreshInterval: 1h
  secretStoreRef:
    name: aws-secretsmanager
    kind: SecretStore
  target:
    name: postgres-secret
  data:
    - secretKey: username
      remoteRef:
        key: r3e-faas/postgres
        property: username
    - secretKey: password
      remoteRef:
        key: r3e-faas/postgres
        property: password
```

### HashiCorp Vault

1. Install Vault:
```bash
helm repo add hashicorp https://helm.releases.hashicorp.com
helm install vault hashicorp/vault
```

2. Configure Vault integration:
```yaml
apiVersion: secrets.hashicorp.com/v1beta1
kind: VaultAuth
metadata:
  name: vault-auth
spec:
  method: kubernetes
  mount: kubernetes
  kubernetes:
    role: r3e-faas
    serviceAccount: r3e-faas-sa

---
apiVersion: secrets.hashicorp.com/v1beta1
kind: VaultStaticSecret
metadata:
  name: postgres-credentials
spec:
  vaultAuthRef: vault-auth
  mount: secret
  path: r3e-faas/postgres
  destination:
    name: postgres-secret
    create: true
  refreshAfter: 1h
```

## Secret Rotation

Implement regular secret rotation:

1. **Automated Rotation**:
   - Use external secret management systems with rotation capabilities
   - Implement custom rotation scripts

2. **Manual Rotation Process**:
   - Create new secrets
   - Update applications to use new secrets
   - Verify functionality
   - Remove old secrets

## Security Best Practices

1. **Principle of Least Privilege**:
   - Only give access to secrets that are needed
   - Use RBAC to restrict access

2. **Encryption at Rest**:
   - Enable etcd encryption
   - Use encrypted storage for persistent volumes

3. **Network Security**:
   - Implement network policies to restrict pod-to-pod communication
   - Use mTLS for service-to-service communication

4. **Secret Validation**:
   - Implement validation for secret formats
   - Use admission controllers to prevent insecure configurations

## Monitoring and Auditing

1. **Audit Logging**:
   - Enable Kubernetes audit logging
   - Monitor secret access

2. **Secret Detection**:
   - Use tools like GitGuardian to detect secrets in code
   - Implement pre-commit hooks to prevent secret leakage

3. **Compliance Monitoring**:
   - Regular security scans
   - Compliance reporting
