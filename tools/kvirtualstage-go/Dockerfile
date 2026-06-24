# Multi-stage build for KVirtualStage Go Framework
FROM golang:1.21-alpine AS builder

# Install build dependencies
RUN apk add --no-cache git ca-certificates tzdata

# Set working directory
WORKDIR /app

# Copy go mod files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy source code
COPY . .

# Build the application
RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-w -s" -o kvs-server ./cmd/server
RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-w -s" -o kvs ./cmd/cli

# Final stage
FROM alpine:latest

# Install runtime dependencies
RUN apk --no-cache add ca-certificates ffmpeg

# Create non-root user
RUN addgroup -g 1001 -S kvs && \
    adduser -u 1001 -S kvs -G kvs

# Set working directory
WORKDIR /app

# Copy built binaries
COPY --from=builder /app/kvs-server /app/kvs ./
COPY --from=builder /app/configs ./configs

# Create necessary directories
RUN mkdir -p /app/logs /app/recordings /app/data && \
    chown -R kvs:kvs /app

# Switch to non-root user
USER kvs

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD ./kvs system info --api-url http://localhost:8080 || exit 1

# Default command
CMD ["./kvs-server", "--config", "configs/config.yaml"]