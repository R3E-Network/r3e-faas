# Trusted Execution Environment (TEE) Service Deployment Guide

This guide provides detailed instructions for deploying the Trusted Execution Environment (TEE) service on Kubernetes as part of the R3E FaaS platform.

## Table of Contents

- [Introduction](#introduction)
- [Prerequisites](#prerequisites)
- [TEE Provider Configuration](#tee-provider-configuration)
- [Deployment Architecture](#deployment-architecture)
- [Security Considerations](#security-considerations)
- [Deployment Steps](#deployment-steps)
- [Verification and Testing](#verification-and-testing)
- [Troubleshooting](#troubleshooting)

## Introduction

The TEE service in the R3E FaaS platform provides secure computing environments for sensitive operations. It supports multiple TEE providers, including Intel SGX and AWS Nitro Enclaves.

## Prerequisites

Before deploying the TEE service, ensure you have:

- Kubernetes cluster with nodes that support the required TEE technology
- For Intel SGX:
  - Nodes with SGX-enabled CPUs
  - SGX device plugin installed
- For AWS Nitro:
  - EKS cluster with Nitro-enabled instances
  - Nitro device plugin installed
- Helm v3+ (for Helm-based deployments)
- Access to container registries for R3E FaaS images

## TEE Provider Configuration

### Intel SGX Configuration

1. Install the Intel SGX device plugin:

```bash
kubectl apply -f https://raw.githubusercontent.com/intel/intel-device-plugins-for-kubernetes/main/deployments/sgx_plugin/base/intel-sgx-plugin.yaml
```

2. Verify the plugin is working:

```bash
kubectl get nodes -o json | jq '.items[].status.allocatable | select(."sgx.intel.com/epc" != null)'
```

### AWS Nitro Configuration

1. Create an EKS cluster with Nitro-enabled instances:

```bash
eksctl create cluster \
  --name r3e-faas-cluster \
  --region us-east-1 \
  --node-type c5.xlarge \
  --nodes 3
```

2. Install the AWS Nitro device plugin:

```bash
kubectl apply -f https://raw.githubusercontent.com/aws/aws-nitro-enclaves-k8s-device-plugin/main/deployments/nitro-enclaves-k8s-device-plugin.yaml
```

## Deployment Architecture

The TEE service deployment consists of:

1. **TEE Service Pods**:
   - Run the TEE service implementation
   - Require access to TEE hardware
   - Handle attestation and key management

2. **TEE Worker Pods**:
   - Execute user functions in secure enclaves
   - Scale based on demand
   - Isolated from other components

3. **Attestation Service**:
   - Verifies the integrity of TEE environments
   - Manages attestation reports
   - Provides cryptographic proof of TEE integrity

## Security Considerations

### Hardware Requirements

- Ensure nodes have the necessary TEE hardware
- Configure node selectors to schedule TEE pods on appropriate nodes

### Attestation

- Configure attestation services
- Set up secure communication channels
- Implement attestation verification

### Key Management

- Securely provision keys to enclaves
- Implement key rotation policies
- Use hardware-backed key storage when available

## Deployment Steps

### 1. Create Namespace

```bash
kubectl create namespace r3e-faas
```

### 2. Create TEE Configuration

Create a ConfigMap for TEE service configuration:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: tee-config
  namespace: r3e-faas
data:
  tee-config.yaml: |
    providers:
      - name: sgx
        enabled: true
        attestation:
          service: "https://api.trustedservices.intel.com/sgx/attestation/v4"
          apiKey: "${SGX_API_KEY}"
      - name: nitro
        enabled: true
        attestation:
          service: "https://vsock.nitro-enclaves.amazonaws.com"
```

### 3. Create Secrets

Create secrets for attestation keys:

```bash
kubectl create secret generic tee-attestation-keys \
  --from-file=sgx-key.pem=/path/to/sgx-key.pem \
  --from-file=nitro-key.pem=/path/to/nitro-key.pem \
  --namespace r3e-faas
```

### 4. Deploy TEE Service

#### Using Helm:

```bash
helm install r3e-tee ./helm/r3e-faas \
  --namespace r3e-faas \
  --values values.yaml \
  --set tee.enabled=true \
  --set tee.providers.sgx.enabled=true \
  --set tee.providers.nitro.enabled=true
```

#### Using kubectl:

```bash
kubectl apply -f k8s/tee-service.yaml -n r3e-faas
```

### 5. Configure Node Selectors

Ensure TEE pods are scheduled on nodes with the required hardware:

```yaml
nodeSelector:
  tee.intel.com/sgx: "true"  # For SGX nodes
  # or
  tee.aws.com/nitro: "true"  # For Nitro nodes
```

### 6. Configure Resource Limits

Set appropriate resource limits for TEE pods:

```yaml
resources:
  limits:
    cpu: 2
    memory: 4Gi
    sgx.intel.com/epc: 512Mi  # For SGX
    # or
    aws.amazon.com/nitro_enclaves: 1  # For Nitro
```

## Verification and Testing

### 1. Verify Pod Status

```bash
kubectl get pods -n r3e-faas -l app=r3e-tee
```

### 2. Check TEE Service Logs

```bash
kubectl logs -f deployment/r3e-tee-service -n r3e-faas
```

### 3. Run Attestation Test

```bash
kubectl exec -it deployment/r3e-tee-service -n r3e-faas -- /bin/sh -c "r3e-tee-cli attestation-test"
```

### 4. Verify Enclave Creation

```bash
kubectl exec -it deployment/r3e-tee-service -n r3e-faas -- /bin/sh -c "r3e-tee-cli list-enclaves"
```

## Troubleshooting

### Common Issues

1. **TEE Hardware Not Detected**:
   - Verify node has the required hardware
   - Check device plugin installation
   - Inspect node labels and taints

2. **Attestation Failures**:
   - Check attestation service connectivity
   - Verify attestation keys are correctly mounted
   - Review attestation service logs

3. **Enclave Creation Failures**:
   - Check resource limits
   - Verify enclave memory allocation
   - Inspect TEE service logs for detailed errors

### Debugging Commands

```bash
# Check TEE device plugin status
kubectl get pods -n kube-system -l name=intel-sgx-plugin

# Verify node capabilities
kubectl describe node <node-name> | grep -A 10 Capacity

# Check TEE service configuration
kubectl describe configmap tee-config -n r3e-faas

# Test enclave creation manually
kubectl exec -it deployment/r3e-tee-service -n r3e-faas -- /bin/sh -c "r3e-tee-cli create-enclave --size 128M --provider sgx"
```
