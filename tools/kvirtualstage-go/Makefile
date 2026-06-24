# KVirtualStage Go Framework Makefile

# Go parameters
GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean
GOTEST=$(GOCMD) test
GOGET=$(GOCMD) get
GOMOD=$(GOCMD) mod

# Binary names
SERVER_BINARY=kvs-server
CLI_BINARY=kvs
TUI_BINARY=kvs-tui

# Build directories
BUILD_DIR=build
DIST_DIR=dist

# Version info
VERSION?=1.0.0
COMMIT?=$(shell git rev-parse --short HEAD)
DATE?=$(shell date -u '+%Y-%m-%d_%H:%M:%S')

# Build flags
LDFLAGS=-ldflags "-X main.version=$(VERSION) -X main.commit=$(COMMIT) -X main.date=$(DATE)"

# Docker settings
DOCKER_IMAGE=kvirtualstage/go-framework
DOCKER_TAG?=latest

.PHONY: all build clean test deps docker help

# Default target
all: deps test build

# Build all binaries
build: build-server build-cli build-tui

# Build server binary
build-server:
	@echo "Building server binary..."
	@mkdir -p $(BUILD_DIR)
	$(GOBUILD) $(LDFLAGS) -o $(BUILD_DIR)/$(SERVER_BINARY) ./cmd/server

# Build CLI binary
build-cli:
	@echo "Building CLI binary..."
	@mkdir -p $(BUILD_DIR)
	$(GOBUILD) $(LDFLAGS) -o $(BUILD_DIR)/$(CLI_BINARY) ./cmd/cli

# Build TUI binary
build-tui:
	@echo "Building TUI binary..."
	@mkdir -p $(BUILD_DIR)
	$(GOBUILD) $(LDFLAGS) -o $(BUILD_DIR)/$(TUI_BINARY) ./cmd/tui

# Cross-compile for multiple platforms
build-cross:
	@echo "Cross-compiling for multiple platforms..."
	@mkdir -p $(DIST_DIR)
	
	# Linux AMD64
	GOOS=linux GOARCH=amd64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(SERVER_BINARY)-linux-amd64 ./cmd/server
	GOOS=linux GOARCH=amd64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(CLI_BINARY)-linux-amd64 ./cmd/cli
	
	# Linux ARM64
	GOOS=linux GOARCH=arm64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(SERVER_BINARY)-linux-arm64 ./cmd/server
	GOOS=linux GOARCH=arm64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(CLI_BINARY)-linux-arm64 ./cmd/cli
	
	# macOS AMD64
	GOOS=darwin GOARCH=amd64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(SERVER_BINARY)-darwin-amd64 ./cmd/server
	GOOS=darwin GOARCH=amd64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(CLI_BINARY)-darwin-amd64 ./cmd/cli
	
	# macOS ARM64 (Apple Silicon)
	GOOS=darwin GOARCH=arm64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(SERVER_BINARY)-darwin-arm64 ./cmd/server
	GOOS=darwin GOARCH=arm64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(CLI_BINARY)-darwin-arm64 ./cmd/cli
	
	# Windows AMD64
	GOOS=windows GOARCH=amd64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(SERVER_BINARY)-windows-amd64.exe ./cmd/server
	GOOS=windows GOARCH=amd64 $(GOBUILD) $(LDFLAGS) -o $(DIST_DIR)/$(CLI_BINARY)-windows-amd64.exe ./cmd/cli

# Install dependencies
deps:
	@echo "Installing dependencies..."
	$(GOMOD) download
	$(GOMOD) tidy

# Update dependencies
deps-update:
	@echo "Updating dependencies..."
	$(GOGET) -u ./...
	$(GOMOD) tidy

# Run tests
test:
	@echo "Running tests..."
	$(GOTEST) -v -race -coverprofile=coverage.out ./...

# Run tests with coverage report
test-coverage:
	@echo "Running tests with coverage..."
	$(GOTEST) -v -race -coverprofile=coverage.out ./...
	$(GOCMD) tool cover -html=coverage.out -o coverage.html
	@echo "Coverage report generated: coverage.html"

# Run benchmarks
benchmark:
	@echo "Running benchmarks..."
	$(GOTEST) -bench=. -benchmem ./...

# Lint code
lint:
	@echo "Running linter..."
	golangci-lint run

# Format code
fmt:
	@echo "Formatting code..."
	$(GOCMD) fmt ./...

# Vet code
vet:
	@echo "Vetting code..."
	$(GOCMD) vet ./...

# Generate code (Swagger docs, etc.)
generate:
	@echo "Generating code..."
	$(GOCMD) generate ./...

# Clean build artifacts
clean:
	@echo "Cleaning..."
	$(GOCLEAN)
	rm -rf $(BUILD_DIR)
	rm -rf $(DIST_DIR)
	rm -f coverage.out coverage.html

# Run server
run-server:
	@echo "Running server..."
	$(GOBUILD) $(LDFLAGS) -o $(BUILD_DIR)/$(SERVER_BINARY) ./cmd/server
	./$(BUILD_DIR)/$(SERVER_BINARY)

# Run server with config
run-server-config:
	@echo "Running server with config..."
	$(GOBUILD) $(LDFLAGS) -o $(BUILD_DIR)/$(SERVER_BINARY) ./cmd/server
	./$(BUILD_DIR)/$(SERVER_BINARY) -config configs/config.yaml

# Run CLI
run-cli:
	@echo "Running CLI..."
	$(GOBUILD) $(LDFLAGS) -o $(BUILD_DIR)/$(CLI_BINARY) ./cmd/cli
	./$(BUILD_DIR)/$(CLI_BINARY)

# Run TUI
run-tui:
	@echo "Running TUI..."
	$(GOBUILD) $(LDFLAGS) -o $(BUILD_DIR)/$(TUI_BINARY) ./cmd/tui
	./$(BUILD_DIR)/$(TUI_BINARY)

# Docker build
docker:
	@echo "Building Docker image..."
	docker build -t $(DOCKER_IMAGE):$(DOCKER_TAG) .
	docker build -t $(DOCKER_IMAGE):$(VERSION) .

# Docker push
docker-push:
	@echo "Pushing Docker image..."
	docker push $(DOCKER_IMAGE):$(DOCKER_TAG)
	docker push $(DOCKER_IMAGE):$(VERSION)

# Docker run
docker-run:
	@echo "Running Docker container..."
	docker run -p 8080:8080 $(DOCKER_IMAGE):$(DOCKER_TAG)

# Install binaries to system
install: build
	@echo "Installing binaries..."
	sudo cp $(BUILD_DIR)/$(SERVER_BINARY) /usr/local/bin/
	sudo cp $(BUILD_DIR)/$(CLI_BINARY) /usr/local/bin/
	sudo cp $(BUILD_DIR)/$(TUI_BINARY) /usr/local/bin/

# Uninstall binaries from system
uninstall:
	@echo "Uninstalling binaries..."
	sudo rm -f /usr/local/bin/$(SERVER_BINARY)
	sudo rm -f /usr/local/bin/$(CLI_BINARY)
	sudo rm -f /usr/local/bin/$(TUI_BINARY)

# Create release archives
release: build-cross
	@echo "Creating release archives..."
	@mkdir -p $(DIST_DIR)/archives
	
	# Linux AMD64
	tar -czf $(DIST_DIR)/archives/kvirtualstage-go-$(VERSION)-linux-amd64.tar.gz -C $(DIST_DIR) $(SERVER_BINARY)-linux-amd64 $(CLI_BINARY)-linux-amd64
	
	# Linux ARM64
	tar -czf $(DIST_DIR)/archives/kvirtualstage-go-$(VERSION)-linux-arm64.tar.gz -C $(DIST_DIR) $(SERVER_BINARY)-linux-arm64 $(CLI_BINARY)-linux-arm64
	
	# macOS AMD64
	tar -czf $(DIST_DIR)/archives/kvirtualstage-go-$(VERSION)-darwin-amd64.tar.gz -C $(DIST_DIR) $(SERVER_BINARY)-darwin-amd64 $(CLI_BINARY)-darwin-amd64
	
	# macOS ARM64
	tar -czf $(DIST_DIR)/archives/kvirtualstage-go-$(VERSION)-darwin-arm64.tar.gz -C $(DIST_DIR) $(SERVER_BINARY)-darwin-arm64 $(CLI_BINARY)-darwin-arm64
	
	# Windows AMD64
	cd $(DIST_DIR) && zip -r archives/kvirtualstage-go-$(VERSION)-windows-amd64.zip $(SERVER_BINARY)-windows-amd64.exe $(CLI_BINARY)-windows-amd64.exe

# Development setup
dev-setup:
	@echo "Setting up development environment..."
	$(GOGET) -u github.com/golangci/golangci-lint/cmd/golangci-lint
	$(GOGET) -u github.com/swaggo/swag/cmd/swag
	$(GOGET) -u github.com/cosmtrek/air
	$(GOMOD) download

# Start development server with hot reload
dev:
	@echo "Starting development server with hot reload..."
	air -c .air.toml

# Database migration
migrate-up:
	@echo "Running database migrations..."
	# Add your migration command here

migrate-down:
	@echo "Rolling back database migrations..."
	# Add your migration rollback command here

# Check code quality
quality: fmt vet lint test

# CI pipeline
ci: deps quality test-coverage build

# Help
help:
	@echo "Available targets:"
	@echo "  build         - Build all binaries"
	@echo "  build-server  - Build server binary"
	@echo "  build-cli     - Build CLI binary"
	@echo "  build-tui     - Build TUI binary"
	@echo "  build-cross   - Cross-compile for multiple platforms"
	@echo "  deps          - Install dependencies"
	@echo "  deps-update   - Update dependencies"
	@echo "  test          - Run tests"
	@echo "  test-coverage - Run tests with coverage report"
	@echo "  benchmark     - Run benchmarks"
	@echo "  lint          - Run linter"
	@echo "  fmt           - Format code"
	@echo "  vet           - Vet code"
	@echo "  generate      - Generate code"
	@echo "  clean         - Clean build artifacts"
	@echo "  run-server    - Run server"
	@echo "  run-cli       - Run CLI"
	@echo "  run-tui       - Run TUI"
	@echo "  docker        - Build Docker image"
	@echo "  docker-push   - Push Docker image"
	@echo "  docker-run    - Run Docker container"
	@echo "  install       - Install binaries to system"
	@echo "  uninstall     - Uninstall binaries from system"
	@echo "  release       - Create release archives"
	@echo "  dev-setup     - Setup development environment"
	@echo "  dev           - Start development server with hot reload"
	@echo "  migrate-up    - Run database migrations"
	@echo "  migrate-down  - Rollback database migrations"
	@echo "  quality       - Check code quality"
	@echo "  ci            - Run CI pipeline"
	@echo "  help          - Show this help message"