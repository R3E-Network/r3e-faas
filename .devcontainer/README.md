# Development Container for R3E FaaS

This directory contains configuration for setting up a development container for the R3E FaaS platform. The development container provides a consistent environment for all developers working on the project.

## Features

- Rust 1.75 with all necessary components (rustfmt, clippy)
- All system dependencies pre-installed
- VS Code extensions for Rust development
- Docker Compose integration
- Debugging tools

## Getting Started

### Prerequisites

- [Visual Studio Code](https://code.visualstudio.com/)
- [Docker](https://www.docker.com/products/docker-desktop)
- [VS Code Remote - Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### Opening the Project in a Dev Container

1. Clone the repository:
   ```bash
   git clone https://github.com/R3E-Network/r3e-faas.git
   cd r3e-faas
   ```

2. Open the project in VS Code:
   ```bash
   code .
   ```

3. When prompted to "Reopen in Container", click "Reopen in Container". Alternatively, you can:
   - Press F1 to open the command palette
   - Type "Remote-Containers: Reopen in Container" and press Enter

4. Wait for the container to build and start. This may take a few minutes the first time.

### Development Workflow

Once the container is running, you can:

- Build the project: `cargo build`
- Run tests: `cargo test -- --nocapture`
- Format code: `cargo fmt`
- Run linter: `cargo clippy`

The development container is configured to forward ports 8080 and 8081, so you can access the API and other services from your host machine.

## Customization

You can customize the development container by modifying the `devcontainer.json` file. For example, you can:

- Add more VS Code extensions
- Change environment variables
- Modify the Docker Compose configuration

For more information, see the [VS Code Remote - Containers documentation](https://code.visualstudio.com/docs/remote/containers).
