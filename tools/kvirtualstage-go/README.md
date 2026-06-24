# KVirtualStage Go Framework

Enterprise-grade Go framework for the KVirtualStage Agent-Computer Interface platform, providing comprehensive REST/GraphQL APIs, CLI tools, TUI applications, and WebSocket support for managing virtual desktop sessions and automation workflows.

## 🚀 Features

### 🌐 API Server
- **REST API**: Comprehensive endpoints for session, automation, and recording management
- **GraphQL API**: Advanced querying capabilities with playground support
- **WebSocket Server**: Real-time updates and session monitoring
- **Swagger Documentation**: Auto-generated API documentation
- **Authentication & Authorization**: JWT-based security with role-based access control

### 🛠️ CLI Tool
- **Session Management**: Create, start, stop, and monitor virtual desktop sessions
- **Automation Control**: Run and manage automation workflows
- **Recording Management**: Control video recording and streaming
- **Multi-format Output**: Table, JSON, and YAML output formats
- **Interactive Commands**: Rich command-line interface with auto-completion

### 📱 TUI Application
- **Interactive Interface**: Beautiful terminal user interface with Bubble Tea
- **Real-time Monitoring**: Live session status and metrics
- **Navigation**: Keyboard-driven navigation between different views
- **Session Control**: Direct session management from the terminal

### 🔧 Core Features
- **Concurrent Processing**: Goroutine-based parallel request handling
- **Middleware Stack**: Comprehensive middleware for logging, security, and metrics
- **Health Checks**: System health monitoring and metrics collection
- **Configuration Management**: YAML-based configuration with environment variable support
- **Testing Suite**: Comprehensive unit and integration tests

## 📋 Prerequisites

- Go 1.21 or later
- Docker (for container management)
- PostgreSQL (for data storage)
- Redis (for caching and session storage)

## 🚀 Quick Start

### 1. Clone and Setup

```bash
git clone <repository-url>
cd kvirtualstage-go

# Install dependencies
make deps

# Setup development environment
make dev-setup
```

### 2. Configuration

Copy and customize the configuration file:

```bash
cp configs/config.yaml configs/local.yaml
# Edit configs/local.yaml with your settings
```

### 3. Build

```bash
# Build all binaries
make build

# Or build specific components
make build-server    # API server
make build-cli       # CLI tool
make build-tui       # TUI application
```

### 4. Run

```bash
# Start the API server
make run-server

# Use the CLI tool
./build/kvs --help

# Launch the TUI
./build/kvs-tui
```

## 🏗️ Architecture

### Project Structure

```
kvirtualstage-go/
├── cmd/                    # Application entry points
│   ├── server/            # API server main
│   ├── cli/               # CLI tool main
│   └── tui/               # TUI application main
├── internal/              # Private application code
│   ├── api/               # API server implementation
│   │   ├── handlers/      # HTTP request handlers
│   │   └── server.go      # Server setup and routing
│   ├── cli/               # CLI implementation
│   ├── tui/               # TUI implementation
│   ├── config/            # Configuration management
│   └── middleware/        # HTTP middleware
├── pkg/                   # Public library code
│   ├── client/            # API client library
│   ├── types/             # Shared types and structs
│   └── utils/             # Utility functions
├── configs/               # Configuration files
├── test/                  # Integration tests
└── docs/                  # Documentation
```

### API Architecture

The API server follows a clean architecture pattern:

- **Handlers**: HTTP request processing and response formatting
- **Middleware**: Cross-cutting concerns (auth, logging, metrics)
- **Types**: Shared data structures and interfaces
- **Config**: Centralized configuration management

## 📖 API Documentation

### REST API Endpoints

#### Sessions
- `GET /api/v1/sessions` - List all sessions
- `POST /api/v1/sessions` - Create a new session
- `GET /api/v1/sessions/{id}` - Get session details
- `PUT /api/v1/sessions/{id}` - Update session
- `DELETE /api/v1/sessions/{id}` - Delete session
- `POST /api/v1/sessions/{id}/start` - Start session
- `POST /api/v1/sessions/{id}/stop` - Stop session
- `GET /api/v1/sessions/{id}/screenshot` - Take screenshot

#### Automation
- `GET /api/v1/automation/scripts` - List automation scripts
- `POST /api/v1/automation/scripts` - Create automation script
- `POST /api/v1/automation/executions` - Execute automation
- `GET /api/v1/automation/executions/{id}` - Get execution status

#### Authentication
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/refresh` - Refresh token
- `GET /api/v1/auth/profile` - Get user profile

### GraphQL API

Access the GraphQL playground at `/graphql` when enabled:

```graphql
query {
  sessions(limit: 10) {
    id
    name
    status
    config {
      desktopEnvironment {
        type
      }
    }
  }
}
```

### WebSocket API

Connect to `/ws` for real-time updates:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');
ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};
```

## 🛠️ CLI Usage

### Session Management

```bash
# Create a new session
kvs session create --name "demo" --desktop ubuntu-xfce --memory 2048

# List all sessions
kvs session list

# Get session details
kvs session get <session-id>

# Start/stop sessions
kvs session start <session-id>
kvs session stop <session-id>

# Take a screenshot
kvs session screenshot <session-id> --output screenshot.png
```

### Automation

```bash
# List automation scripts
kvs automation scripts list

# Execute an automation
kvs automation execute --script <script-id> --session <session-id>
```

### Configuration

```bash
# Set API endpoint
kvs config set api-url http://localhost:8080

# Set authentication token
kvs config set token <your-token>
```

## 🖥️ TUI Usage

Launch the TUI with:

```bash
kvs tui
```

### Navigation
- `1-4`: Switch between different views (Sessions, Automation, Recordings, Settings)
- `Tab`: Switch focus between components
- `Enter`: Select/view details
- `Esc`: Go back
- `r`: Refresh current view
- `q`: Quit application

## 🧪 Testing

### Run Tests

```bash
# Run all tests
make test

# Run with coverage
make test-coverage

# Run benchmarks
make benchmark

# Run integration tests
make test-integration
```

### Test Categories

- **Unit Tests**: Test individual functions and methods
- **Integration Tests**: Test API endpoints and component interactions
- **Benchmark Tests**: Performance testing for critical paths

## 🔧 Development

### Development Server

Start the development server with hot reload:

```bash
make dev
```

### Code Quality

```bash
# Format code
make fmt

# Run linter
make lint

# Check code quality
make quality

# Generate code (Swagger docs, etc.)
make generate
```

### Building for Production

```bash
# Build for multiple platforms
make build-cross

# Create release archives
make release

# Build Docker image
make docker
```

## 📊 Monitoring & Metrics

### Health Checks

- `/health` - Application health status
- `/metrics` - Prometheus metrics

### Logging

Structured logging with configurable levels:

```yaml
logging:
  level: "info"
  format: "json"
  output: ["stdout", "file"]
```

### Metrics

Prometheus metrics collection:

- HTTP request duration and count
- Active WebSocket connections
- Session statistics
- Resource usage

## 🔒 Security

### Authentication

JWT-based authentication with configurable expiration:

```yaml
security:
  jwt:
    secret_key: "your-secret-key"
    token_duration: "24h"
    refresh_duration: "168h"
```

### Authorization

Role-based access control:

- **Admin**: Full system access
- **Operator**: Session and automation management
- **User**: Own sessions and recordings
- **Viewer**: Read-only access

### Security Headers

Automatic security headers:

- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Strict-Transport-Security`

## 🐳 Docker Deployment

### Build Image

```bash
make docker
```

### Run Container

```bash
docker run -p 8080:8080 kvirtualstage/go-framework:latest
```

### Docker Compose

```yaml
version: '3.8'
services:
  kvs-server:
    image: kvirtualstage/go-framework:latest
    ports:
      - "8080:8080"
    environment:
      - KVS_DATABASE_HOST=postgres
      - KVS_REDIS_HOST=redis
    depends_on:
      - postgres
      - redis
```

## 📈 Performance

### Optimizations

- **Goroutine-based concurrency**: Efficient handling of concurrent requests
- **Connection pooling**: Database and Redis connection reuse
- **Middleware optimization**: Minimal overhead for cross-cutting concerns
- **Response caching**: Redis-based caching for frequently accessed data

### Benchmarks

```bash
# Run performance benchmarks
make benchmark

# Load testing
ab -n 1000 -c 10 http://localhost:8080/health
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run the test suite (`make test`)
6. Commit your changes (`git commit -am 'Add new feature'`)
7. Push to the branch (`git push origin feature/new-feature`)
8. Create a Pull Request

### Code Style

- Follow Go conventions and best practices
- Use `gofmt` for code formatting
- Add comprehensive tests for new features
- Update documentation for API changes

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Integration with KVirtualStage

This Go framework seamlessly integrates with the main KVirtualStage Rust core:

- **FFI Integration**: Direct integration with Rust core components
- **Shared Types**: Compatible data structures across language boundaries
- **Unified API**: Consistent interface for both Rust and Go components
- **Container Orchestration**: Manages Docker containers created by Rust core

## 📞 Support

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](../../issues)
- **Discussions**: [GitHub Discussions](../../discussions)

---

**KVirtualStage Go Framework** - Enterprise-grade virtual desktop automation platform built with Go's concurrency and performance in mind.