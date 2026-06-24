package handlers

import (
	"github.com/kvirtualstage/kvirtualstage-go/internal/config"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/auth"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/session"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/automation"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/recording"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/user"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/system"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/events"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/websocket"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/graphql"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/health"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers/metrics"
	"github.com/sirupsen/logrus"
)

// Handlers contains all API handlers
type Handlers struct {
	Auth       *auth.Handler
	Session    *session.Handler
	Automation *automation.Handler
	Recording  *recording.Handler
	User       *user.Handler
	System     *system.Handler
	Events     *events.Handler
	WebSocket  *websocket.Handler
	GraphQL    *graphql.Handler
	Health     *health.Handler
	Metrics    *metrics.Handler
}

// NewHandlers creates a new instance of all handlers
func NewHandlers(cfg *config.Config, logger *logrus.Logger) *Handlers {
	return &Handlers{
		Auth:       auth.NewHandler(cfg, logger),
		Session:    session.NewHandler(cfg, logger),
		Automation: automation.NewHandler(cfg, logger),
		Recording:  recording.NewHandler(cfg, logger),
		User:       user.NewHandler(cfg, logger),
		System:     system.NewHandler(cfg, logger),
		Events:     events.NewHandler(cfg, logger),
		WebSocket:  websocket.NewHandler(cfg, logger),
		GraphQL:    graphql.NewHandler(cfg, logger),
		Health:     health.NewHandler(cfg, logger),
		Metrics:    metrics.NewHandler(cfg, logger),
	}
}