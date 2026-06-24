package main

import (
	"context"
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/kvirtualstage/kvirtualstage-go/internal/api"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers"
	"github.com/kvirtualstage/kvirtualstage-go/internal/config"
	"github.com/sirupsen/logrus"
)

var (
	configPath = flag.String("config", "", "Path to configuration file")
	version    = "dev"
	commit     = "unknown"
	date       = "unknown"
)

func main() {
	flag.Parse()

	// Initialize logger
	logger := logrus.New()
	logger.SetFormatter(&logrus.JSONFormatter{})

	// Load configuration
	cfg, err := config.Load(*configPath)
	if err != nil {
		logger.Fatalf("Failed to load configuration: %v", err)
	}

	// Validate configuration
	if err := cfg.Validate(); err != nil {
		logger.Fatalf("Invalid configuration: %v", err)
	}

	// Set log level
	level, err := logrus.ParseLevel(cfg.Logging.Level)
	if err != nil {
		logger.Warnf("Invalid log level '%s', using 'info'", cfg.Logging.Level)
		level = logrus.InfoLevel
	}
	logger.SetLevel(level)

	// Log startup information
	logger.WithFields(logrus.Fields{
		"version": version,
		"commit":  commit,
		"date":    date,
		"config":  *configPath,
	}).Info("Starting KVirtualStage Go API Server")

	// Initialize handlers
	handlers := handlers.NewHandlers(cfg, logger)

	// Create API server
	server := api.NewServer(cfg, logger, handlers)

	// Start server in a goroutine
	serverErr := make(chan error, 1)
	go func() {
		if err := server.Start(); err != nil {
			serverErr <- fmt.Errorf("server failed to start: %w", err)
		}
	}()

	// Wait for interrupt signal
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)

	select {
	case err := <-serverErr:
		logger.Errorf("Server error: %v", err)
		os.Exit(1)
	case sig := <-quit:
		logger.Infof("Received signal: %v", sig)
	}

	// Graceful shutdown
	logger.Info("Shutting down server...")
	
	ctx, cancel := context.WithTimeout(context.Background(), cfg.Server.ShutdownTimeout)
	defer cancel()

	if err := server.Stop(ctx); err != nil {
		logger.Errorf("Server forced to shutdown: %v", err)
		os.Exit(1)
	}

	logger.Info("Server exited gracefully")
}