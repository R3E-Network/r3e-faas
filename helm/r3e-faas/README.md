# R3E FaaS Helm Chart

This Helm chart deploys the R3E FaaS platform on a Kubernetes cluster.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.2.0+
- PV provisioner support in the underlying infrastructure
- Ingress controller (if using ingress)

## Installing the Chart

To install the chart with the release name `r3e-faas`:

```bash
helm install r3e-faas ./helm/r3e-faas
```

The command deploys R3E FaaS on the Kubernetes cluster with the default configuration. The [Parameters](#parameters) section lists the parameters that can be configured during installation.

## Uninstalling the Chart

To uninstall/delete the `r3e-faas` deployment:

```bash
helm uninstall r3e-faas
```

## Parameters

### Global Parameters

| Name                | Description                                     | Value       |
| ------------------- | ----------------------------------------------- | ----------- |
| `global.environment`| Environment (production, staging, development)  | `production`|
| `global.logLevel`   | Log level (debug, info, warn, error)            | `info`      |

### API Parameters

| Name                           | Description                                      | Value                |
| ------------------------------ | ------------------------------------------------ | -------------------- |
| `api.replicaCount`             | Number of API replicas                           | `2`                  |
| `api.image.repository`         | API image repository                             | `r3e-faas/api`       |
| `api.image.tag`                | API image tag                                    | `latest`             |
| `api.image.pullPolicy`         | API image pull policy                            | `IfNotPresent`       |
| `api.service.type`             | API service type                                 | `ClusterIP`          |
| `api.service.port`             | API service port                                 | `80`                 |
| `api.service.targetPort`       | API container port                               | `8080`               |
| `api.resources.limits.cpu`     | API CPU limit                                    | `1`                  |
| `api.resources.limits.memory`  | API memory limit                                 | `1Gi`                |
| `api.resources.requests.cpu`   | API CPU request                                  | `500m`               |
| `api.resources.requests.memory`| API memory request                               | `512Mi`              |
| `api.nodeSelector`             | Node labels for API pods assignment              | `{}`                 |
| `api.tolerations`              | Tolerations for API pods assignment              | `[]`                 |
| `api.affinity`                 | Affinity for API pods assignment                 | `{}`                 |

### Worker Parameters

| Name                              | Description                                      | Value                |
| --------------------------------- | ------------------------------------------------ | -------------------- |
| `worker.replicaCount`             | Number of Worker replicas                        | `3`                  |
| `worker.image.repository`         | Worker image repository                          | `r3e-faas/worker`    |
| `worker.image.tag`                | Worker image tag                                 | `latest`             |
| `worker.image.pullPolicy`         | Worker image pull policy                         | `IfNotPresent`       |
| `worker.resources.limits.cpu`     | Worker CPU limit                                 | `2`                  |
| `worker.resources.limits.memory`  | Worker memory limit                              | `2Gi`                |
| `worker.resources.requests.cpu`   | Worker CPU request                               | `1`                  |
| `worker.resources.requests.memory`| Worker memory request                            | `1Gi`                |
| `worker.nodeSelector`             | Node labels for Worker pods assignment           | `{}`                 |
| `worker.tolerations`              | Tolerations for Worker pods assignment           | `[]`                 |
| `worker.affinity`                 | Affinity for Worker pods assignment              | `{}`                 |
| `worker.maxConcurrentFunctions`   | Maximum number of concurrent functions           | `10`                 |
| `worker.functionTimeoutSeconds`   | Function execution timeout in seconds            | `30`                 |

### Redis Parameters

| Name                            | Description                                      | Value                |
| ------------------------------- | ------------------------------------------------ | -------------------- |
| `redis.enabled`                 | Deploy Redis                                     | `true`               |
| `redis.image.repository`        | Redis image repository                           | `redis`              |
| `redis.image.tag`               | Redis image tag                                  | `6.2-alpine`         |
| `redis.image.pullPolicy`        | Redis image pull policy                          | `IfNotPresent`       |
| `redis.service.port`            | Redis service port                               | `6379`               |
| `redis.resources.limits.cpu`    | Redis CPU limit                                  | `500m`               |
| `redis.resources.limits.memory` | Redis memory limit                               | `512Mi`              |
| `redis.resources.requests.cpu`  | Redis CPU request                                | `200m`               |
| `redis.resources.requests.memory`| Redis memory request                            | `256Mi`              |
| `redis.persistence.enabled`     | Enable persistence using PVC                     | `true`               |
| `redis.persistence.size`        | PVC Storage Request for Redis volume             | `10Gi`               |
| `redis.persistence.storageClass`| PVC Storage Class for Redis volume               | `r3e-storage`        |

### PostgreSQL Parameters

| Name                               | Description                                      | Value                |
| ---------------------------------- | ------------------------------------------------ | -------------------- |
| `postgres.enabled`                 | Deploy PostgreSQL                                | `true`               |
| `postgres.image.repository`        | PostgreSQL image repository                      | `postgres`           |
| `postgres.image.tag`               | PostgreSQL image tag                             | `14-alpine`          |
| `postgres.image.pullPolicy`        | PostgreSQL image pull policy                     | `IfNotPresent`       |
| `postgres.service.port`            | PostgreSQL service port                          | `5432`               |
| `postgres.resources.limits.cpu`    | PostgreSQL CPU limit                             | `1`                  |
| `postgres.resources.limits.memory` | PostgreSQL memory limit                          | `1Gi`                |
| `postgres.resources.requests.cpu`  | PostgreSQL CPU request                           | `500m`               |
| `postgres.resources.requests.memory`| PostgreSQL memory request                       | `512Mi`              |
| `postgres.persistence.enabled`     | Enable persistence using PVC                     | `true`               |
| `postgres.persistence.size`        | PVC Storage Request for PostgreSQL volume        | `20Gi`               |
| `postgres.persistence.storageClass`| PVC Storage Class for PostgreSQL volume          | `r3e-storage`        |
| `postgres.env.POSTGRES_USER`       | PostgreSQL username                              | `postgres`           |
| `postgres.env.POSTGRES_PASSWORD`   | PostgreSQL password                              | `postgres`           |
| `postgres.env.POSTGRES_DB`         | PostgreSQL database name                         | `r3e_faas`           |

### Storage Parameters

| Name                        | Description                                      | Value                |
| --------------------------- | ------------------------------------------------ | -------------------- |
| `storage.storageClass`      | Storage class name                               | `r3e-storage`        |
| `storage.rocksdb.enabled`   | Enable RocksDB storage                           | `true`               |
| `storage.rocksdb.size`      | PVC Storage Request for RocksDB volume           | `50Gi`               |

### Ingress Parameters

| Name                        | Description                                      | Value                |
| --------------------------- | ------------------------------------------------ | -------------------- |
| `ingress.enabled`           | Enable ingress                                   | `true`               |
| `ingress.className`         | Ingress class name                               | `nginx`              |
| `ingress.annotations`       | Ingress annotations                              | See values.yaml      |
| `ingress.hosts`             | Ingress hosts                                    | See values.yaml      |
| `ingress.tls`               | Ingress TLS configuration                        | See values.yaml      |

### Configuration Parameters

| Name                        | Description                                      | Value                |
| --------------------------- | ------------------------------------------------ | -------------------- |
| `config.neo.rpcUrl`         | Neo N3 RPC URL                                   | `http://neo-node:10332` |
| `config.ethereum.rpcUrl`    | Ethereum RPC URL                                 | `http://ethereum-node:8545` |

## Configuration

The following table lists the configurable parameters of the R3E FaaS chart and their default values.

Specify each parameter using the `--set key=value[,key=value]` argument to `helm install`.

Alternatively, a YAML file that specifies the values for the parameters can be provided while installing the chart. For example:

```bash
helm install r3e-faas ./helm/r3e-faas -f values.yaml
```

## Persistence

The R3E FaaS chart mounts persistent volumes for RocksDB, Redis, and PostgreSQL. The volumes are created using dynamic volume provisioning. Persistent Volume Claims are used to claim the required volumes from the underlying infrastructure.

### Adjust permissions of persistent volume mountpoint

As the images run as non-root by default, it is necessary to adjust the ownership of the persistent volumes so that the containers can write data into them.

By default, the chart is configured to use Kubernetes Security Context to automatically change the ownership of the volumes. However, this feature does not work in all Kubernetes distributions.

As an alternative, this chart supports using an initContainer to change the ownership of the volumes before mounting them. You can enable this initContainer by setting `volumePermissions.enabled` to `true`.

## Upgrading

### To 1.0.0

This is the first version of the chart.
