# Docker Production Deployment

This guide explains how to deploy the R3E FaaS platform in a production environment using Docker.

## Prerequisites

- Docker 20.10 or later
- Docker Compose 2.0 or later
- A server with at least 2GB RAM and 2 CPU cores

## Deployment Options

There are several ways to deploy the R3E FaaS platform:

1. **Single Container**: Simple deployment with all components in one container
2. **Docker Compose**: Multi-container deployment with separate services
3. **Kubernetes**: Scalable deployment with orchestration

## Single Container Deployment

### Building the Docker Image

```bash
# Clone the repository
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas

# Build the Docker image
docker build -t r3e-faas:latest .
```

### Running the Container

```bash
# Create a data directory
mkdir -p /var/lib/r3e-faas

# Run the container
docker run -d \
  --name r3e-faas \
  -p 8080:8080 \
  -v /var/lib/r3e-faas:/data \
  -e R3E_FAAS__GENERAL__ENVIRONMENT=production \
  -e R3E_FAAS__STORAGE__STORAGE_TYPE=rocksdb \
  -e R3E_FAAS__STORAGE__ROCKSDB_PATH=/data/db \
  r3e-faas:latest
```

### Updating the Container

```bash
# Pull the latest image
docker pull r3e-faas:latest

# Stop and remove the old container
docker stop r3e-faas
docker rm r3e-faas

# Run the new container
docker run -d \
  --name r3e-faas \
  -p 8080:8080 \
  -v /var/lib/r3e-faas:/data \
  -e R3E_FAAS__GENERAL__ENVIRONMENT=production \
  -e R3E_FAAS__STORAGE__STORAGE_TYPE=rocksdb \
  -e R3E_FAAS__STORAGE__ROCKSDB_PATH=/data/db \
  r3e-faas:latest
```

## Docker Compose Deployment

### Creating the Docker Compose File

Create a `docker-compose.yml` file:

```yaml
version: '3.8'

services:
  api:
    image: r3e-faas:latest
    command: ["r3e", "api", "--config", "/config/r3e-faas.yaml"]
    ports:
      - "8080:8080"
    volumes:
      - ./data:/data
      - ./config:/config
    environment:
      - R3E_FAAS__GENERAL__ENVIRONMENT=production
      - R3E_FAAS__STORAGE__STORAGE_TYPE=rocksdb
      - R3E_FAAS__STORAGE__ROCKSDB_PATH=/data/db
    restart: unless-stopped
    depends_on:
      - worker

  worker:
    image: r3e-faas:latest
    command: ["r3e", "worker", "--config", "/config/r3e-faas.yaml"]
    volumes:
      - ./data:/data
      - ./config:/config
    environment:
      - R3E_FAAS__GENERAL__ENVIRONMENT=production
      - R3E_FAAS__STORAGE__STORAGE_TYPE=rocksdb
      - R3E_FAAS__STORAGE__ROCKSDB_PATH=/data/db
    restart: unless-stopped
    deploy:
      replicas: 2

  scheduler:
    image: r3e-faas:latest
    command: ["r3e", "scheduler", "--config", "/config/r3e-faas.yaml"]
    volumes:
      - ./data:/data
      - ./config:/config
    environment:
      - R3E_FAAS__GENERAL__ENVIRONMENT=production
      - R3E_FAAS__STORAGE__STORAGE_TYPE=rocksdb
      - R3E_FAAS__STORAGE__ROCKSDB_PATH=/data/db
    restart: unless-stopped
```

### Running Docker Compose

```bash
# Create the necessary directories
mkdir -p data config

# Create a configuration file
cp config/r3e-faas.example.yaml config/r3e-faas.yaml

# Edit the configuration file
nano config/r3e-faas.yaml

# Start the services
docker-compose up -d
```

### Scaling Workers

```bash
# Scale up workers
docker-compose up --scale worker=4 -d
```

## Kubernetes Deployment

For production deployments at scale, Kubernetes is recommended.

### Prerequisites

- Kubernetes cluster
- kubectl configured
- Helm (optional)

### Deployment Steps

1. Create a namespace:

```bash
kubectl create namespace r3e-faas
```

2. Create a ConfigMap for configuration:

```bash
kubectl create configmap r3e-faas-config --from-file=config/r3e-faas.yaml -n r3e-faas
```

3. Create a PersistentVolumeClaim for data:

```bash
kubectl apply -f kubernetes/pvc.yaml
```

4. Deploy the services:

```bash
kubectl apply -f kubernetes/deployment.yaml
```

5. Expose the API service:

```bash
kubectl apply -f kubernetes/service.yaml
```

### Example Kubernetes Files

#### pvc.yaml

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: r3e-faas-data
  namespace: r3e-faas
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 10Gi
```

#### deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: r3e-faas-api
  namespace: r3e-faas
spec:
  replicas: 1
  selector:
    matchLabels:
      app: r3e-faas
      component: api
  template:
    metadata:
      labels:
        app: r3e-faas
        component: api
    spec:
      containers:
      - name: api
        image: r3e-faas:latest
        command: ["r3e", "api", "--config", "/config/r3e-faas.yaml"]
        ports:
        - containerPort: 8080
        volumeMounts:
        - name: config
          mountPath: /config
        - name: data
          mountPath: /data
        env:
        - name: R3E_FAAS__GENERAL__ENVIRONMENT
          value: production
        - name: R3E_FAAS__STORAGE__STORAGE_TYPE
          value: rocksdb
        - name: R3E_FAAS__STORAGE__ROCKSDB_PATH
          value: /data/db
      volumes:
      - name: config
        configMap:
          name: r3e-faas-config
      - name: data
        persistentVolumeClaim:
          claimName: r3e-faas-data
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: r3e-faas-worker
  namespace: r3e-faas
spec:
  replicas: 2
  selector:
    matchLabels:
      app: r3e-faas
      component: worker
  template:
    metadata:
      labels:
        app: r3e-faas
        component: worker
    spec:
      containers:
      - name: worker
        image: r3e-faas:latest
        command: ["r3e", "worker", "--config", "/config/r3e-faas.yaml"]
        volumeMounts:
        - name: config
          mountPath: /config
        - name: data
          mountPath: /data
        env:
        - name: R3E_FAAS__GENERAL__ENVIRONMENT
          value: production
        - name: R3E_FAAS__STORAGE__STORAGE_TYPE
          value: rocksdb
        - name: R3E_FAAS__STORAGE__ROCKSDB_PATH
          value: /data/db
      volumes:
      - name: config
        configMap:
          name: r3e-faas-config
      - name: data
        persistentVolumeClaim:
          claimName: r3e-faas-data
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: r3e-faas-scheduler
  namespace: r3e-faas
spec:
  replicas: 1
  selector:
    matchLabels:
      app: r3e-faas
      component: scheduler
  template:
    metadata:
      labels:
        app: r3e-faas
        component: scheduler
    spec:
      containers:
      - name: scheduler
        image: r3e-faas:latest
        command: ["r3e", "scheduler", "--config", "/config/r3e-faas.yaml"]
        volumeMounts:
        - name: config
          mountPath: /config
        - name: data
          mountPath: /data
        env:
        - name: R3E_FAAS__GENERAL__ENVIRONMENT
          value: production
        - name: R3E_FAAS__STORAGE__STORAGE_TYPE
          value: rocksdb
        - name: R3E_FAAS__STORAGE__ROCKSDB_PATH
          value: /data/db
      volumes:
      - name: config
        configMap:
          name: r3e-faas-config
      - name: data
        persistentVolumeClaim:
          claimName: r3e-faas-data
```

#### service.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: r3e-faas-api
  namespace: r3e-faas
spec:
  selector:
    app: r3e-faas
    component: api
  ports:
  - port: 8080
    targetPort: 8080
  type: LoadBalancer
```

## Monitoring and Logging

### Prometheus Metrics

The R3E FaaS platform exposes Prometheus metrics at `/metrics`. You can configure Prometheus to scrape these metrics.

### Logging

Logs are written to stdout/stderr and can be collected by Docker's logging driver or Kubernetes' logging system.

### Health Checks

The API service exposes health check endpoints:

- `/health`: Overall health check
- `/health/liveness`: Liveness probe
- `/health/readiness`: Readiness probe

## Security Considerations

### Network Security

- Use a reverse proxy (e.g., Nginx, Traefik) to handle TLS termination
- Restrict access to the API service using network policies
- Use a private Docker registry for production images

### Authentication and Authorization

- Enable authentication in the configuration
- Set a strong JWT secret
- Use HTTPS for all communications

### Secrets Management

- Use Docker secrets or Kubernetes secrets for sensitive information
- Do not hardcode secrets in Docker images or configuration files
- Rotate secrets regularly

## Backup and Recovery

### Data Backup

Regularly backup the data directory:

```bash
# For Docker
docker run --rm -v r3e-faas-data:/data -v $(pwd)/backup:/backup alpine tar -czf /backup/r3e-faas-backup-$(date +%Y%m%d).tar.gz -C /data .

# For Kubernetes
kubectl exec -n r3e-faas $(kubectl get pod -n r3e-faas -l app=r3e-faas,component=api -o jsonpath='{.items[0].metadata.name}') -- tar -czf /tmp/backup.tar.gz -C /data .
kubectl cp r3e-faas/$(kubectl get pod -n r3e-faas -l app=r3e-faas,component=api -o jsonpath='{.items[0].metadata.name}'):/tmp/backup.tar.gz ./backup/r3e-faas-backup-$(date +%Y%m%d).tar.gz
```

### Recovery

To restore from a backup:

```bash
# For Docker
docker run --rm -v r3e-faas-data:/data -v $(pwd)/backup:/backup alpine sh -c "rm -rf /data/* && tar -xzf /backup/r3e-faas-backup-20230101.tar.gz -C /data"

# For Kubernetes
kubectl cp ./backup/r3e-faas-backup-20230101.tar.gz r3e-faas/$(kubectl get pod -n r3e-faas -l app=r3e-faas,component=api -o jsonpath='{.items[0].metadata.name}'):/tmp/backup.tar.gz
kubectl exec -n r3e-faas $(kubectl get pod -n r3e-faas -l app=r3e-faas,component=api -o jsonpath='{.items[0].metadata.name}') -- sh -c "rm -rf /data/* && tar -xzf /tmp/backup.tar.gz -C /data"
```

## Troubleshooting

### Common Issues

- **Container won't start**: Check the logs with `docker logs r3e-faas` or `kubectl logs -n r3e-faas <pod-name>`
- **API not accessible**: Check if the container is running and the port is exposed
- **Database errors**: Check if the data directory is properly mounted and has the correct permissions
- **High memory usage**: Adjust the memory limits in the configuration

### Getting Support

- Check the [GitHub repository](https://github.com/R3E-Network/r3e-faas) for issues and solutions
- Join the community Discord server for real-time support
- Contact the R3E Network team for enterprise support
