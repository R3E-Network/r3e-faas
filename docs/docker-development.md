# Docker Development Environment

This guide explains how to set up a development environment for the R3E FaaS platform using Docker.

## Prerequisites

- Docker 20.10 or later
- Docker Compose 2.0 or later
- Git

## Getting Started

1. Clone the repository:

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas
```

2. Build the development Docker image:

```bash
docker build -f Dockerfile.dev -t r3e-faas-dev .
```

3. Start the development environment:

```bash
docker-compose -f docker-compose.dev.yml up -d
```

## Development Workflow

### Building the Project

You can build the project inside the Docker container:

```bash
docker exec -it r3e-faas-dev cargo build
```

### Running Tests

To run tests inside the Docker container:

```bash
docker exec -it r3e-faas-dev cargo test
```

### Running the Application

To run the application inside the Docker container:

```bash
docker exec -it r3e-faas-dev cargo run --bin r3e -- worker
```

### Accessing the Application

The application is accessible at:

- HTTP API: http://localhost:8080
- WebSocket API: ws://localhost:8081

### Stopping the Environment

To stop the development environment:

```bash
docker-compose -f docker-compose.dev.yml down
```

## Development Container Structure

The development container includes:

- Rust toolchain
- RocksDB and other dependencies
- Development tools (git, curl, etc.)
- VS Code Remote Development support

## Using VS Code Remote Development

1. Install the "Remote - Containers" extension in VS Code.
2. Open the command palette (Ctrl+Shift+P) and select "Remote-Containers: Attach to Running Container".
3. Select the `r3e-faas-dev` container.
4. VS Code will open a new window connected to the container.

## Customizing the Development Environment

### Adding Dependencies

If you need to add new dependencies to the development environment, you can modify the `Dockerfile.dev` file:

```dockerfile
# Add new dependencies
RUN apt-get update && apt-get install -y \
    new-dependency-package \
    && rm -rf /var/lib/apt/lists/*
```

Then rebuild the Docker image:

```bash
docker build -f Dockerfile.dev -t r3e-faas-dev .
```

### Environment Variables

You can set environment variables in the `docker-compose.dev.yml` file:

```yaml
services:
  r3e-faas-dev:
    environment:
      - RUST_LOG=debug
      - R3E_FAAS__GENERAL__ENVIRONMENT=development
```

## Troubleshooting

### Container Won't Start

If the container won't start, check the Docker logs:

```bash
docker logs r3e-faas-dev
```

### Permission Issues

If you encounter permission issues, make sure the user inside the container has the correct permissions:

```bash
docker exec -it r3e-faas-dev chown -R $(id -u):$(id -g) /app
```

### Build Errors

If you encounter build errors, try cleaning the build cache:

```bash
docker exec -it r3e-faas-dev cargo clean
```

## Next Steps

- [Development Guide](./development.md)
- [API Reference](./api-reference.md)
- [Docker Production](./docker-production.md)
