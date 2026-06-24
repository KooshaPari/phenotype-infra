package middleware

import (
	"net/http"
	"strings"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
	"github.com/google/uuid"
	"github.com/kvirtualstage/kvirtualstage-go/internal/config"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/types"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"github.com/sirupsen/logrus"
	"golang.org/x/time/rate"
)

// Logger middleware for structured logging
func Logger(logger *logrus.Logger) gin.HandlerFunc {
	return func(c *gin.Context) {
		start := time.Now()
		path := c.Request.URL.Path
		raw := c.Request.URL.RawQuery

		// Process request
		c.Next()

		// Calculate latency
		latency := time.Since(start)

		// Build log entry
		entry := logger.WithFields(logrus.Fields{
			"status":     c.Writer.Status(),
			"method":     c.Request.Method,
			"path":       path,
			"ip":         c.ClientIP(),
			"user_agent": c.Request.UserAgent(),
			"latency":    latency,
			"request_id": c.GetString("request_id"),
		})

		if raw != "" {
			entry = entry.WithField("query", raw)
		}

		if len(c.Errors) > 0 {
			entry = entry.WithField("errors", c.Errors.String())
		}

		// Log based on status code
		switch {
		case c.Writer.Status() >= 500:
			entry.Error("Server error")
		case c.Writer.Status() >= 400:
			entry.Warn("Client error")
		default:
			entry.Info("Request processed")
		}
	}
}

// CORS middleware for cross-origin resource sharing
func CORS(corsConfig config.CORSConfig) gin.HandlerFunc {
	return func(c *gin.Context) {
		origin := c.Request.Header.Get("Origin")
		
		// Check if origin is allowed
		if isOriginAllowed(origin, corsConfig.AllowedOrigins) {
			c.Header("Access-Control-Allow-Origin", origin)
		}

		c.Header("Access-Control-Allow-Methods", strings.Join(corsConfig.AllowedMethods, ", "))
		c.Header("Access-Control-Allow-Headers", strings.Join(corsConfig.AllowedHeaders, ", "))
		c.Header("Access-Control-Expose-Headers", strings.Join(corsConfig.ExposedHeaders, ", "))
		
		if corsConfig.AllowCredentials {
			c.Header("Access-Control-Allow-Credentials", "true")
		}
		
		if corsConfig.MaxAge > 0 {
			c.Header("Access-Control-Max-Age", string(rune(corsConfig.MaxAge)))
		}

		if c.Request.Method == "OPTIONS" {
			c.AbortWithStatus(http.StatusNoContent)
			return
		}

		c.Next()
	}
}

// isOriginAllowed checks if origin is in allowed list
func isOriginAllowed(origin string, allowedOrigins []string) bool {
	for _, allowed := range allowedOrigins {
		if allowed == "*" || allowed == origin {
			return true
		}
	}
	return false
}

// RateLimit middleware for rate limiting
func RateLimit(rateLimitConfig config.RateLimitConfig) gin.HandlerFunc {
	limiter := rate.NewLimiter(
		rate.Limit(rateLimitConfig.RequestsPerMinute)/60, // requests per second
		rateLimitConfig.BurstSize,
	)

	return func(c *gin.Context) {
		if !limiter.Allow() {
			c.JSON(http.StatusTooManyRequests, types.APIResponse{
				Success: false,
				Error:   "Rate limit exceeded",
				Message: "Too many requests. Please try again later.",
			})
			c.Abort()
			return
		}
		c.Next()
	}
}

// SecurityHeaders middleware adds security headers
func SecurityHeaders() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Header("X-Content-Type-Options", "nosniff")
		c.Header("X-Frame-Options", "DENY")
		c.Header("X-XSS-Protection", "1; mode=block")
		c.Header("Strict-Transport-Security", "max-age=31536000; includeSubDomains")
		c.Header("Content-Security-Policy", "default-src 'self'")
		c.Header("Referrer-Policy", "strict-origin-when-cross-origin")
		c.Next()
	}
}

// RequestID middleware adds unique request ID
func RequestID() gin.HandlerFunc {
	return func(c *gin.Context) {
		requestID := c.GetHeader("X-Request-ID")
		if requestID == "" {
			requestID = uuid.New().String()
		}
		c.Set("request_id", requestID)
		c.Header("X-Request-ID", requestID)
		c.Next()
	}
}

// Metrics middleware for Prometheus metrics
func Metrics() gin.HandlerFunc {
	var (
		httpDuration = prometheus.NewHistogramVec(
			prometheus.HistogramOpts{
				Name: "http_request_duration_seconds",
				Help: "Duration of HTTP requests.",
			},
			[]string{"method", "endpoint", "status_code"},
		)
		httpRequests = prometheus.NewCounterVec(
			prometheus.CounterOpts{
				Name: "http_requests_total",
				Help: "Total number of HTTP requests.",
			},
			[]string{"method", "endpoint", "status_code"},
		)
		httpRequestsInFlight = prometheus.NewGauge(
			prometheus.GaugeOpts{
				Name: "http_requests_in_flight",
				Help: "Current number of HTTP requests in flight.",
			},
		)
	)

	prometheus.MustRegister(httpDuration, httpRequests, httpRequestsInFlight)

	return func(c *gin.Context) {
		start := time.Now()
		httpRequestsInFlight.Inc()

		c.Next()

		duration := time.Since(start).Seconds()
		statusCode := string(rune(c.Writer.Status()))
		endpoint := c.FullPath()
		if endpoint == "" {
			endpoint = "unknown"
		}

		httpDuration.WithLabelValues(c.Request.Method, endpoint, statusCode).Observe(duration)
		httpRequests.WithLabelValues(c.Request.Method, endpoint, statusCode).Inc()
		httpRequestsInFlight.Dec()
	}
}

// Authentication middleware for JWT token validation
func Authentication(jwtConfig config.JWTConfig) gin.HandlerFunc {
	return func(c *gin.Context) {
		tokenString := extractToken(c)
		if tokenString == "" {
			c.JSON(http.StatusUnauthorized, types.APIResponse{
				Success: false,
				Error:   "Unauthorized",
				Message: "Missing or invalid authentication token",
			})
			c.Abort()
			return
		}

		token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
			if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
				return nil, jwt.ErrSignatureInvalid
			}
			return []byte(jwtConfig.SecretKey), nil
		})

		if err != nil || !token.Valid {
			c.JSON(http.StatusUnauthorized, types.APIResponse{
				Success: false,
				Error:   "Unauthorized",
				Message: "Invalid authentication token",
			})
			c.Abort()
			return
		}

		if claims, ok := token.Claims.(jwt.MapClaims); ok {
			// Extract user information from claims
			userID, _ := claims["user_id"].(string)
			username, _ := claims["username"].(string)
			role, _ := claims["role"].(string)
			
			// Set user context
			c.Set("user_id", userID)
			c.Set("username", username)
			c.Set("role", role)
			c.Set("jwt_claims", claims)
		}

		c.Next()
	}
}

// Authorization middleware for role-based access control
func Authorization() gin.HandlerFunc {
	return func(c *gin.Context) {
		role := c.GetString("role")
		endpoint := c.FullPath()
		method := c.Request.Method

		if !isAuthorized(role, endpoint, method) {
			c.JSON(http.StatusForbidden, types.APIResponse{
				Success: false,
				Error:   "Forbidden",
				Message: "Insufficient permissions to access this resource",
			})
			c.Abort()
			return
		}

		c.Next()
	}
}

// extractToken extracts JWT token from request
func extractToken(c *gin.Context) string {
	// Check Authorization header
	authHeader := c.GetHeader("Authorization")
	if authHeader != "" {
		parts := strings.Split(authHeader, " ")
		if len(parts) == 2 && parts[0] == "Bearer" {
			return parts[1]
		}
	}

	// Check query parameter
	token := c.Query("token")
	if token != "" {
		return token
	}

	// Check cookie
	cookie, err := c.Cookie("auth_token")
	if err == nil && cookie != "" {
		return cookie
	}

	return ""
}

// isAuthorized checks if role has permission for endpoint
func isAuthorized(role, endpoint, method string) bool {
	// Admin has access to everything
	if role == string(types.UserRoleAdmin) {
		return true
	}

	// Operator permissions
	if role == string(types.UserRoleOperator) {
		operatorEndpoints := []string{
			"/api/v1/sessions",
			"/api/v1/automation",
			"/api/v1/recordings",
			"/api/v1/system/metrics",
			"/api/v1/system/resources",
		}
		
		for _, allowed := range operatorEndpoints {
			if strings.HasPrefix(endpoint, allowed) {
				return true
			}
		}
	}

	// User permissions (default)
	if role == string(types.UserRoleUser) || role == "" {
		userEndpoints := []string{
			"/api/v1/auth/profile",
			"/api/v1/sessions",
			"/api/v1/automation/scripts",
			"/api/v1/automation/executions",
			"/api/v1/recordings",
		}
		
		for _, allowed := range userEndpoints {
			if strings.HasPrefix(endpoint, allowed) {
				// Users can only perform certain operations
				if method == "DELETE" && strings.Contains(endpoint, "/users/") {
					return false // Users can't delete other users
				}
				return true
			}
		}
	}

	// Viewer permissions (read-only)
	if role == string(types.UserRoleViewer) {
		if method == "GET" {
			viewerEndpoints := []string{
				"/api/v1/sessions",
				"/api/v1/automation",
				"/api/v1/recordings",
				"/api/v1/system/metrics",
			}
			
			for _, allowed := range viewerEndpoints {
				if strings.HasPrefix(endpoint, allowed) {
					return true
				}
			}
		}
	}

	return false
}

// PrometheusHandler returns Prometheus metrics handler
func PrometheusHandler() gin.HandlerFunc {
	handler := promhttp.Handler()
	return func(c *gin.Context) {
		handler.ServeHTTP(c.Writer, c.Request)
	}
}

// Recovery middleware with custom error handling
func Recovery() gin.HandlerFunc {
	return gin.CustomRecovery(func(c *gin.Context, recovered interface{}) {
		if err, ok := recovered.(string); ok {
			c.JSON(http.StatusInternalServerError, types.APIResponse{
				Success: false,
				Error:   "Internal Server Error",
				Message: err,
			})
		} else {
			c.JSON(http.StatusInternalServerError, types.APIResponse{
				Success: false,
				Error:   "Internal Server Error",
				Message: "An unexpected error occurred",
			})
		}
		c.Abort()
	})
}

// ValidateJSON middleware validates JSON input
func ValidateJSON(obj interface{}) gin.HandlerFunc {
	return func(c *gin.Context) {
		if err := c.ShouldBindJSON(obj); err != nil {
			c.JSON(http.StatusBadRequest, types.APIResponse{
				Success: false,
				Error:   "Invalid JSON",
				Message: err.Error(),
			})
			c.Abort()
			return
		}
		c.Next()
	}
}

// IPWhitelist middleware for IP-based access control
func IPWhitelist(allowedIPs []string) gin.HandlerFunc {
	return func(c *gin.Context) {
		clientIP := c.ClientIP()
		
		if len(allowedIPs) > 0 {
			allowed := false
			for _, ip := range allowedIPs {
				if ip == clientIP || ip == "*" {
					allowed = true
					break
				}
			}
			
			if !allowed {
				c.JSON(http.StatusForbidden, types.APIResponse{
					Success: false,
					Error:   "Forbidden",
					Message: "Access denied from this IP address",
				})
				c.Abort()
				return
			}
		}
		
		c.Next()
	}
}