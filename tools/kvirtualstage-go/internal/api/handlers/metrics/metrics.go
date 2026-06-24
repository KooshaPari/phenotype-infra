package metrics

import (
	"github.com/gin-gonic/gin"
	"github.com/kvirtualstage/kvirtualstage-go/internal/config"
	"github.com/kvirtualstage/kvirtualstage-go/internal/middleware"
	"github.com/sirupsen/logrus"
)

// Handler handles metrics requests
type Handler struct {
	config *config.Config
	logger *logrus.Logger
}

// NewHandler creates a new metrics handler
func NewHandler(cfg *config.Config, logger *logrus.Logger) *Handler {
	return &Handler{
		config: cfg,
		logger: logger,
	}
}

// Handler returns the Prometheus metrics handler
func (h *Handler) Handler() gin.HandlerFunc {
	return middleware.PrometheusHandler()
}