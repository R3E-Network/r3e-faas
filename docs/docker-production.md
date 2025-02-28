# Docker Production Guide for R3E FaaS

This guide provides detailed instructions for deploying the R3E FaaS platform in a production environment using Docker.

## Prerequisites

Before deploying the R3E FaaS platform in production, ensure you have the following prerequisites:

- **Docker**: Latest version
- **Docker Compose**: Latest version
- **Docker Swarm** or **Kubernetes**: For orchestration (optional but recommended)
- **SSL Certificate**: For secure communication
- **Domain Name**: For accessing the platform

## Production Deployment

### 1. Clone the Repository

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas
```

### 2. Configure the Platform

Create a `.env` file for production configuration:

```bash
cat > .env << EOL
# General Configuration
R3E_FAAS__GENERAL__ENVIRONMENT=production
R3E_FAAS__GENERAL__LOG_LEVEL=info

# API Configuration
R3E_FAAS__API__PORT=8080
R3E_FAAS__API__HOST=0.0.0.0
R3E_FAAS__API__CORS_ALLOWED_ORIGINS=https://your-domain.com
R3E_FAAS__API__REQUEST_TIMEOUT=30

# Storage Configuration
R3E_FAAS__STORAGE__TYPE=rocksdb
R3E_FAAS__STORAGE__PATH=/data/rocksdb

# Neo Configuration
R3E_FAAS__NEO__RPC_URL=https://mainnet.rpc.neo.org
R3E_FAAS__NEO__NETWORK=mainnet
R3E_FAAS__NEO__GAS_BANK_CONTRACT=0x1234567890abcdef1234567890abcdef12345678
R3E_FAAS__NEO__META_TX_CONTRACT=0x1234567890abcdef1234567890abcdef12345678

# Ethereum Configuration
R3E_FAAS__ETHEREUM__RPC_URL=https://mainnet.infura.io/v3/your-api-key
R3E_FAAS__ETHEREUM__NETWORK=mainnet
R3E_FAAS__ETHEREUM__GAS_BANK_CONTRACT=0x1234567890abcdef1234567890abcdef12345678
R3E_FAAS__ETHEREUM__META_TX_CONTRACT=0x1234567890abcdef1234567890abcdef12345678

# Worker Configuration
R3E_FAAS__WORKER__MAX_CONCURRENT_FUNCTIONS=20
R3E_FAAS__WORKER__FUNCTION_TIMEOUT=60
R3E_FAAS__WORKER__MEMORY_LIMIT=1024

# ZK Configuration
R3E_FAAS__ZK__PROVIDER=zokrates
R3E_FAAS__ZK__STORAGE_TYPE=rocksdb
R3E_FAAS__ZK__STORAGE_PATH=/data/zk
R3E_FAAS__ZK__MAX_CIRCUIT_SIZE=10485760
R3E_FAAS__ZK__TIMEOUT=300

# FHE Configuration
R3E_FAAS__FHE__SCHEME=tfhe
R3E_FAAS__FHE__STORAGE_TYPE=rocksdb
R3E_FAAS__FHE__STORAGE_PATH=/data/fhe
R3E_FAAS__FHE__MAX_CIPHERTEXT_SIZE=10485760
R3E_FAAS__FHE__TIMEOUT=300

# TEE Configuration
R3E_FAAS__TEE__PROVIDER=nitro
R3E_FAAS__TEE__ATTESTATION_URL=https://attestation.example.com
R3E_FAAS__TEE__ATTESTATION_TIMEOUT=30
EOL
```

### 3. Production Docker Compose Configuration

The repository includes a `docker-compose.prod.yml` file specifically configured for production:

```yaml
version: '3.8'

services:
  api:
    build:
      context: .
      dockerfile: docker/api/Dockerfile
      target: production
    image: r3e-faas/api:latest
    ports:
      - "8080:8080"
    volumes:
      - r3e-data:/data
    env_file:
      - .env
    restart: always
    deploy:
      replicas: 2
      update_config:
        parallelism: 1
        delay: 10s
      restart_policy:
        condition: on-failure
        max_attempts: 3
        window: 120s
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/v1/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 30s
    depends_on:
      - worker

  worker:
    build:
      context: .
      dockerfile: docker/worker/Dockerfile
      target: production
    image: r3e-faas/worker:latest
    volumes:
      - r3e-data:/data
    env_file:
      - .env
    restart: always
    deploy:
      replicas: 4
      update_config:
        parallelism: 1
        delay: 10s
      restart_policy:
        condition: on-failure
        max_attempts: 3
        window: 120s
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8081/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 30s

  nginx:
    image: nginx:latest
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/conf.d:/etc/nginx/conf.d
      - ./nginx/ssl:/etc/nginx/ssl
      - ./nginx/www:/var/www/html
    depends_on:
      - api
    restart: always
    deploy:
      replicas: 1
      update_config:
        parallelism: 1
        delay: 10s
      restart_policy:
        condition: on-failure
        max_attempts: 3
        window: 120s

volumes:
  r3e-data:
    driver: local
```

### 4. NGINX Configuration

Create an NGINX configuration for SSL termination and reverse proxy:

```bash
mkdir -p nginx/conf.d
cat > nginx/conf.d/r3e-faas.conf << EOL
server {
    listen 80;
    server_name your-domain.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name your-domain.com;

    ssl_certificate /etc/nginx/ssl/fullchain.pem;
    ssl_certificate_key /etc/nginx/ssl/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;

    location / {
        proxy_pass http://api:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /api/v1/docs {
        proxy_pass http://api:8080/api/v1/docs;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /api/v1/health {
        proxy_pass http://api:8080/api/v1/health;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
EOL
```

### 5. SSL Certificate

Place your SSL certificate files in the `nginx/ssl` directory:

```bash
mkdir -p nginx/ssl
# Copy your SSL certificate files to nginx/ssl/fullchain.pem and nginx/ssl/privkey.pem
```

### 6. Start the Production Environment

```bash
# Start the production environment
docker-compose -f docker-compose.prod.yml up -d
```

### 7. Verify the Deployment

```bash
# Check if the services are running
docker-compose -f docker-compose.prod.yml ps

# Check the logs
docker-compose -f docker-compose.prod.yml logs -f

# Check the health of the API
curl https://your-domain.com/api/v1/health
```

## Docker Swarm Deployment

For high availability and scalability, you can deploy the R3E FaaS platform using Docker Swarm:

### 1. Initialize Docker Swarm

```bash
# Initialize Docker Swarm
docker swarm init

# Or specify the advertise address
docker swarm init --advertise-addr <IP_ADDRESS>
```

### 2. Create a Docker Stack File

```bash
cat > docker-stack.yml << EOL
version: '3.8'

services:
  api:
    image: r3e-faas/api:latest
    ports:
      - "8080:8080"
    volumes:
      - r3e-data:/data
    env_file:
      - .env
    deploy:
      replicas: 2
      update_config:
        parallelism: 1
        delay: 10s
      restart_policy:
        condition: on-failure
        max_attempts: 3
        window: 120s
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/v1/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 30s
    depends_on:
      - worker

  worker:
    image: r3e-faas/worker:latest
    volumes:
      - r3e-data:/data
    env_file:
      - .env
    deploy:
      replicas: 4
      update_config:
        parallelism: 1
        delay: 10s
      restart_policy:
        condition: on-failure
        max_attempts: 3
        window: 120s
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8081/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 30s

  nginx:
    image: nginx:latest
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/conf.d:/etc/nginx/conf.d
      - ./nginx/ssl:/etc/nginx/ssl
      - ./nginx/www:/var/www/html
    deploy:
      replicas: 1
      update_config:
        parallelism: 1
        delay: 10s
      restart_policy:
        condition: on-failure
        max_attempts: 3
        window: 120s

volumes:
  r3e-data:
    driver: local
EOL
```

### 3. Deploy the Stack

```bash
# Build the images
docker-compose -f docker-compose.prod.yml build

# Deploy the stack
docker stack deploy -c docker-stack.yml r3e-faas
```

### 4. Verify the Deployment

```bash
# Check the stack
docker stack ps r3e-faas

# Check the services
docker service ls

# Check the logs
docker service logs r3e-faas_api
```

## Kubernetes Deployment

For enterprise-grade deployment, you can use Kubernetes:

### 1. Create Kubernetes Manifests

Create a directory for Kubernetes manifests:

```bash
mkdir -p k8s
```

#### Namespace

```bash
cat > k8s/namespace.yaml << EOL
apiVersion: v1
kind: Namespace
metadata:
  name: r3e-faas
EOL
```

#### ConfigMap

```bash
cat > k8s/configmap.yaml << EOL
apiVersion: v1
kind: ConfigMap
metadata:
  name: r3e-faas-config
  namespace: r3e-faas
data:
  R3E_FAAS__GENERAL__ENVIRONMENT: "production"
  R3E_FAAS__GENERAL__LOG_LEVEL: "info"
  R3E_FAAS__API__PORT: "8080"
  R3E_FAAS__API__HOST: "0.0.0.0"
  R3E_FAAS__API__CORS_ALLOWED_ORIGINS: "https://your-domain.com"
  R3E_FAAS__API__REQUEST_TIMEOUT: "30"
  R3E_FAAS__STORAGE__TYPE: "rocksdb"
  R3E_FAAS__STORAGE__PATH: "/data/rocksdb"
  R3E_FAAS__WORKER__MAX_CONCURRENT_FUNCTIONS: "20"
  R3E_FAAS__WORKER__FUNCTION_TIMEOUT: "60"
  R3E_FAAS__WORKER__MEMORY_LIMIT: "1024"
EOL
```

#### Secret

```bash
cat > k8s/secret.yaml << EOL
apiVersion: v1
kind: Secret
metadata:
  name: r3e-faas-secret
  namespace: r3e-faas
type: Opaque
data:
  # Base64 encoded values
  R3E_FAAS__NEO__RPC_URL: "aHR0cHM6Ly9tYWlubmV0LnJwYy5uZW8ub3Jn"
  R3E_FAAS__NEO__NETWORK: "bWFpbm5ldA=="
  R3E_FAAS__NEO__GAS_BANK_CONTRACT: "MHgxMjM0NTY3ODkwYWJjZGVmMTIzNDU2Nzg5MGFiY2RlZjEyMzQ1Njc4"
  R3E_FAAS__NEO__META_TX_CONTRACT: "MHgxMjM0NTY3ODkwYWJjZGVmMTIzNDU2Nzg5MGFiY2RlZjEyMzQ1Njc4"
  R3E_FAAS__ETHEREUM__RPC_URL: "aHR0cHM6Ly9tYWlubmV0LmluZnVyYS5pby92My95b3VyLWFwaS1rZXk="
  R3E_FAAS__ETHEREUM__NETWORK: "bWFpbm5ldA=="
  R3E_FAAS__ETHEREUM__GAS_BANK_CONTRACT: "MHgxMjM0NTY3ODkwYWJjZGVmMTIzNDU2Nzg5MGFiY2RlZjEyMzQ1Njc4"
  R3E_FAAS__ETHEREUM__META_TX_CONTRACT: "MHgxMjM0NTY3ODkwYWJjZGVmMTIzNDU2Nzg5MGFiY2RlZjEyMzQ1Njc4"
EOL
```

#### Persistent Volume Claim

```bash
cat > k8s/pvc.yaml << EOL
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
EOL
```

#### API Deployment

```bash
cat > k8s/api-deployment.yaml << EOL
apiVersion: apps/v1
kind: Deployment
metadata:
  name: r3e-faas-api
  namespace: r3e-faas
spec:
  replicas: 2
  selector:
    matchLabels:
      app: r3e-faas-api
  template:
    metadata:
      labels:
        app: r3e-faas-api
    spec:
      containers:
      - name: api
        image: r3e-faas/api:latest
        ports:
        - containerPort: 8080
        volumeMounts:
        - name: r3e-faas-data
          mountPath: /data
        envFrom:
        - configMapRef:
            name: r3e-faas-config
        - secretRef:
            name: r3e-faas-secret
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 30
          timeoutSeconds: 10
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
      volumes:
      - name: r3e-faas-data
        persistentVolumeClaim:
          claimName: r3e-faas-data
EOL
```

#### Worker Deployment

```bash
cat > k8s/worker-deployment.yaml << EOL
apiVersion: apps/v1
kind: Deployment
metadata:
  name: r3e-faas-worker
  namespace: r3e-faas
spec:
  replicas: 4
  selector:
    matchLabels:
      app: r3e-faas-worker
  template:
    metadata:
      labels:
        app: r3e-faas-worker
    spec:
      containers:
      - name: worker
        image: r3e-faas/worker:latest
        volumeMounts:
        - name: r3e-faas-data
          mountPath: /data
        envFrom:
        - configMapRef:
            name: r3e-faas-config
        - secretRef:
            name: r3e-faas-secret
        resources:
          requests:
            memory: "1Gi"
            cpu: "1000m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8081
          initialDelaySeconds: 30
          periodSeconds: 30
          timeoutSeconds: 10
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /health
            port: 8081
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
      volumes:
      - name: r3e-faas-data
        persistentVolumeClaim:
          claimName: r3e-faas-data
EOL
```

#### API Service

```bash
cat > k8s/api-service.yaml << EOL
apiVersion: v1
kind: Service
metadata:
  name: r3e-faas-api
  namespace: r3e-faas
spec:
  selector:
    app: r3e-faas-api
  ports:
  - port: 8080
    targetPort: 8080
  type: ClusterIP
EOL
```

#### Ingress

```bash
cat > k8s/ingress.yaml << EOL
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: r3e-faas-ingress
  namespace: r3e-faas
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - your-domain.com
    secretName: r3e-faas-tls
  rules:
  - host: your-domain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: r3e-faas-api
            port:
              number: 8080
EOL
```

### 2. Apply the Kubernetes Manifests

```bash
# Create the namespace
kubectl apply -f k8s/namespace.yaml

# Apply the manifests
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secret.yaml
kubectl apply -f k8s/pvc.yaml
kubectl apply -f k8s/api-deployment.yaml
kubectl apply -f k8s/worker-deployment.yaml
kubectl apply -f k8s/api-service.yaml
kubectl apply -f k8s/ingress.yaml
```

### 3. Verify the Deployment

```bash
# Check the pods
kubectl get pods -n r3e-faas

# Check the services
kubectl get services -n r3e-faas

# Check the ingress
kubectl get ingress -n r3e-faas

# Check the logs
kubectl logs -n r3e-faas deployment/r3e-faas-api
```

## Production Best Practices

### Security

1. **SSL/TLS**: Always use SSL/TLS for production deployments
2. **Secrets Management**: Use Docker Secrets or Kubernetes Secrets for sensitive information
3. **Network Security**: Use network policies to restrict traffic
4. **Container Security**: Use minimal base images and scan for vulnerabilities
5. **Access Control**: Implement proper authentication and authorization

### Scalability

1. **Horizontal Scaling**: Scale the API and worker services horizontally
2. **Load Balancing**: Use a load balancer to distribute traffic
3. **Resource Limits**: Set appropriate resource limits for containers
4. **Auto Scaling**: Implement auto-scaling based on CPU and memory usage
5. **Database Scaling**: Consider using a distributed database for large-scale deployments

### Reliability

1. **Health Checks**: Implement health checks for all services
2. **Restart Policies**: Configure appropriate restart policies
3. **Backup and Recovery**: Regularly backup data and test recovery procedures
4. **Monitoring**: Implement comprehensive monitoring
5. **Logging**: Implement centralized logging

### Monitoring

1. **Prometheus**: Use Prometheus for metrics collection
2. **Grafana**: Use Grafana for visualization
3. **Alerting**: Configure alerts for critical issues
4. **Tracing**: Implement distributed tracing
5. **Log Aggregation**: Use a log aggregation solution like ELK stack

### Continuous Deployment

1. **CI/CD Pipeline**: Implement a CI/CD pipeline for automated deployments
2. **Blue-Green Deployment**: Use blue-green deployment for zero-downtime updates
3. **Canary Releases**: Implement canary releases for gradual rollouts
4. **Rollback Strategy**: Have a rollback strategy in place
5. **Testing**: Implement comprehensive testing in the CI/CD pipeline

## Troubleshooting

### Common Issues

#### Container Startup Issues

If containers fail to start:

```bash
# Check the logs
docker-compose -f docker-compose.prod.yml logs api
docker-compose -f docker-compose.prod.yml logs worker

# Check for errors in the configuration
docker-compose -f docker-compose.prod.yml config
```

#### Network Issues

If services cannot communicate:

```bash
# Check the network
docker network ls
docker network inspect r3e-faas_default

# Check if the services are running
docker-compose -f docker-compose.prod.yml ps
```

#### Storage Issues

If there are storage issues:

```bash
# Check the volumes
docker volume ls
docker volume inspect r3e-faas_r3e-data

# Check the permissions
docker-compose -f docker-compose.prod.yml run --rm --user root api ls -la /data
```

#### Performance Issues

If there are performance issues:

```bash
# Check the resource usage
docker stats

# Check the logs for slow operations
docker-compose -f docker-compose.prod.yml logs | grep -i slow
```

### Getting Help

If you encounter issues not covered in this guide:

- Check the logs for error messages
- Open an issue on GitHub
- Join the community chat

## Next Steps

After deploying the R3E FaaS platform in production, you can:

- Set up monitoring and alerting
- Implement a backup and recovery strategy
- Configure auto-scaling
- Implement a CI/CD pipeline
- Explore advanced deployment options
