package api

import (
	"context"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/kvirtualstage/kvirtualstage-go/internal/config"
	"github.com/kvirtualstage/kvirtualstage-go/internal/middleware"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/types"
	"github.com/sirupsen/logrus"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
)

// Server represents the API server
type Server struct {
	config     *config.Config
	router     *gin.Engine
	httpServer *http.Server
	logger     *logrus.Logger
	handlers   *handlers.Handlers
}

// NewServer creates a new API server instance
func NewServer(cfg *config.Config, logger *logrus.Logger, handlers *handlers.Handlers) *Server {
	if !cfg.Logging.Structured {
		gin.SetMode(gin.ReleaseMode)
	}

	router := gin.New()
	
	server := &Server{
		config:   cfg,
		router:   router,
		logger:   logger,
		handlers: handlers,
	}

	server.setupMiddleware()
	server.setupRoutes()
	server.setupSwagger()

	server.httpServer = &http.Server{
		Addr:         cfg.GetServerAddr(),
		Handler:      router,
		ReadTimeout:  cfg.Server.ReadTimeout,
		WriteTimeout: cfg.Server.WriteTimeout,
		IdleTimeout:  cfg.Server.IdleTimeout,
	}

	return server
}

// setupMiddleware configures middleware
func (s *Server) setupMiddleware() {
	// Recovery middleware
	s.router.Use(gin.Recovery())

	// Logger middleware
	s.router.Use(middleware.Logger(s.logger))

	// CORS middleware
	s.router.Use(middleware.CORS(s.config.Server.CORS))

	// Rate limiting middleware
	if s.config.Server.RateLimit.Enabled {
		s.router.Use(middleware.RateLimit(s.config.Server.RateLimit))
	}

	// Security headers middleware
	s.router.Use(middleware.SecurityHeaders())

	// Request ID middleware
	s.router.Use(middleware.RequestID())

	// Metrics middleware
	if s.config.Metrics.Enabled {
		s.router.Use(middleware.Metrics())
	}

	// Authentication middleware (applied to protected routes)
	authMiddleware := middleware.Authentication(s.config.Security.JWT)
	
	// Authorization middleware
	authzMiddleware := middleware.Authorization()

	// Apply auth middleware to protected route groups
	s.setupAuthRoutes(authMiddleware, authzMiddleware)
}

// setupAuthRoutes sets up authentication for protected routes
func (s *Server) setupAuthRoutes(authMiddleware, authzMiddleware gin.HandlerFunc) {
	// This will be used when setting up protected route groups
	s.router.Use(func(c *gin.Context) {
		// Skip auth for public endpoints
		if isPublicEndpoint(c.Request.URL.Path) {
			c.Next()
			return
		}
		authMiddleware(c)
		if c.IsAborted() {
			return
		}
		authzMiddleware(c)
	})
}

// isPublicEndpoint checks if an endpoint is public
func isPublicEndpoint(path string) bool {
	publicPaths := []string{
		"/health",
		"/metrics",
		"/swagger/",
		"/api/v1/auth/login",
		"/api/v1/auth/register",
		"/api/v1/auth/refresh",
		"/api/v1/system/info",
	}

	for _, publicPath := range publicPaths {
		if len(path) >= len(publicPath) && path[:len(publicPath)] == publicPath {
			return true
		}
	}
	return false
}

// setupRoutes configures API routes
func (s *Server) setupRoutes() {
	// Health check endpoint
	s.router.GET("/health", s.handlers.Health.Check)
	
	// Metrics endpoint
	if s.config.Metrics.Enabled {
		s.router.GET(s.config.Metrics.Path, s.handlers.Metrics.Handler())
	}

	// API v1 routes
	v1 := s.router.Group("/api/v1")
	{
		// Authentication routes
		auth := v1.Group("/auth")
		{
			auth.POST("/login", s.handlers.Auth.Login)
			auth.POST("/register", s.handlers.Auth.Register)
			auth.POST("/refresh", s.handlers.Auth.RefreshToken)
			auth.POST("/logout", s.handlers.Auth.Logout)
			auth.GET("/profile", s.handlers.Auth.GetProfile)
			auth.PUT("/profile", s.handlers.Auth.UpdateProfile)
			auth.POST("/change-password", s.handlers.Auth.ChangePassword)
		}

		// Session management routes
		sessions := v1.Group("/sessions")
		{
			sessions.GET("", s.handlers.Session.List)
			sessions.POST("", s.handlers.Session.Create)
			sessions.GET("/:id", s.handlers.Session.Get)
			sessions.PUT("/:id", s.handlers.Session.Update)
			sessions.DELETE("/:id", s.handlers.Session.Delete)
			sessions.POST("/:id/start", s.handlers.Session.Start)
			sessions.POST("/:id/stop", s.handlers.Session.Stop)
			sessions.POST("/:id/pause", s.handlers.Session.Pause)
			sessions.POST("/:id/resume", s.handlers.Session.Resume)
			sessions.POST("/:id/restart", s.handlers.Session.Restart)
			sessions.GET("/:id/status", s.handlers.Session.GetStatus)
			sessions.GET("/:id/logs", s.handlers.Session.GetLogs)
			sessions.GET("/:id/screenshot", s.handlers.Session.Screenshot)
			sessions.GET("/:id/vnc", s.handlers.Session.VNCInfo)
			sessions.GET("/:id/ws", s.handlers.Session.WebSocket)
		}

		// Automation routes
		automation := v1.Group("/automation")
		{
			// Scripts
			scripts := automation.Group("/scripts")
			{
				scripts.GET("", s.handlers.Automation.ListScripts)
				scripts.POST("", s.handlers.Automation.CreateScript)
				scripts.GET("/:id", s.handlers.Automation.GetScript)
				scripts.PUT("/:id", s.handlers.Automation.UpdateScript)
				scripts.DELETE("/:id", s.handlers.Automation.DeleteScript)
				scripts.POST("/:id/validate", s.handlers.Automation.ValidateScript)
			}

			// Executions
			executions := automation.Group("/executions")
			{
				executions.GET("", s.handlers.Automation.ListExecutions)
				executions.POST("", s.handlers.Automation.Execute)
				executions.GET("/:id", s.handlers.Automation.GetExecution)
				executions.POST("/:id/cancel", s.handlers.Automation.CancelExecution)
				executions.GET("/:id/steps", s.handlers.Automation.GetExecutionSteps)
				executions.GET("/:id/screenshots", s.handlers.Automation.GetExecutionScreenshots)
			}

			// Templates
			templates := automation.Group("/templates")
			{
				templates.GET("", s.handlers.Automation.ListTemplates)
				templates.GET("/:name", s.handlers.Automation.GetTemplate)
			}
		}

		// Recording routes
		recordings := v1.Group("/recordings")
		{
			recordings.GET("", s.handlers.Recording.List)
			recordings.POST("", s.handlers.Recording.Start)
			recordings.GET("/:id", s.handlers.Recording.Get)
			recordings.DELETE("/:id", s.handlers.Recording.Delete)
			recordings.POST("/:id/stop", s.handlers.Recording.Stop)
			recordings.GET("/:id/download", s.handlers.Recording.Download)
			recordings.GET("/:id/stream", s.handlers.Recording.Stream)
			recordings.POST("/:id/convert", s.handlers.Recording.Convert)
		}

		// User management routes (admin only)
		users := v1.Group("/users")
		{
			users.GET("", s.handlers.User.List)
			users.POST("", s.handlers.User.Create)
			users.GET("/:id", s.handlers.User.Get)
			users.PUT("/:id", s.handlers.User.Update)
			users.DELETE("/:id", s.handlers.User.Delete)
			users.POST("/:id/activate", s.handlers.User.Activate)
			users.POST("/:id/deactivate", s.handlers.User.Deactivate)
			users.POST("/:id/reset-password", s.handlers.User.ResetPassword)
		}

		// System management routes
		system := v1.Group("/system")
		{
			system.GET("/info", s.handlers.System.Info)
			system.GET("/status", s.handlers.System.Status)
			system.GET("/metrics", s.handlers.System.Metrics)
			system.GET("/resources", s.handlers.System.Resources)
			system.POST("/cleanup", s.handlers.System.Cleanup)
			system.GET("/logs", s.handlers.System.Logs)
			system.POST("/restart", s.handlers.System.Restart)
		}

		// Events and notifications
		events := v1.Group("/events")
		{
			events.GET("", s.handlers.Events.List)
			events.GET("/stream", s.handlers.Events.Stream)
			events.POST("/webhook", s.handlers.Events.Webhook)
		}
	}

	// WebSocket endpoint for real-time updates
	s.router.GET("/ws", s.handlers.WebSocket.Handle)

	// GraphQL endpoint (if enabled)
	if s.config.Server.GraphQL.Enabled {
		s.router.POST("/graphql", s.handlers.GraphQL.Handle)
		if s.config.Server.GraphQL.Playground {
			s.router.GET("/graphql", s.handlers.GraphQL.Playground)
		}
	}

	// Static file serving for recordings and assets
	s.router.Static("/recordings", s.config.Recording.OutputPath)
	s.router.Static("/assets", "./web/assets")
	
	// Serve web UI (if available)
	s.router.Static("/ui", "./web/dist")
	s.router.NoRoute(func(c *gin.Context) {
		// Serve index.html for SPA routes
		if c.Request.Method == "GET" && !gin.IsDebugging() {
			c.File("./web/dist/index.html")
			return
		}
		c.JSON(http.StatusNotFound, types.APIResponse{
			Success: false,
			Error:   "Not Found",
			Message: "The requested resource was not found",
		})
	})
}

// setupSwagger configures Swagger documentation
func (s *Server) setupSwagger() {
	// Swagger endpoint
	s.router.GET("/swagger/*any", ginSwagger.WrapHandler(swaggerFiles.Handler))
}

// Start starts the HTTP server
func (s *Server) Start() error {
	s.logger.Infof("Starting API server on %s", s.config.GetServerAddr())
	
	if s.config.Server.TLS.Enabled {
		return s.httpServer.ListenAndServeTLS(
			s.config.Server.TLS.CertFile,
			s.config.Server.TLS.KeyFile,
		)
	}
	
	return s.httpServer.ListenAndServe()
}

// Stop gracefully stops the HTTP server
func (s *Server) Stop(ctx context.Context) error {
	s.logger.Info("Shutting down API server...")
	
	// Create shutdown context with timeout
	shutdownCtx, cancel := context.WithTimeout(ctx, s.config.Server.ShutdownTimeout)
	defer cancel()
	
	return s.httpServer.Shutdown(shutdownCtx)
}

// Router returns the Gin router (for testing)
func (s *Server) Router() *gin.Engine {
	return s.router
}

// HTTPServer returns the HTTP server instance
func (s *Server) HTTPServer() *http.Server {
	return s.httpServer
}