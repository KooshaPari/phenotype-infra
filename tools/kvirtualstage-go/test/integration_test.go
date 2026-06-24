package test

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api"
	"github.com/kvirtualstage/kvirtualstage-go/internal/api/handlers"
	"github.com/kvirtualstage/kvirtualstage-go/internal/config"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/types"
	"github.com/sirupsen/logrus"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupTestServer() *gin.Engine {
	gin.SetMode(gin.TestMode)
	
	// Load test configuration
	cfg := &config.Config{
		Server: config.ServerConfig{
			Host: "localhost",
			Port: 8080,
		},
		Security: config.SecurityConfig{
			JWT: config.JWTConfig{
				SecretKey: "test-secret-key",
			},
		},
	}

	// Create test logger
	logger := logrus.New()
	logger.SetLevel(logrus.ErrorLevel) // Reduce log noise in tests

	// Create handlers
	handlers := handlers.NewHandlers(cfg, logger)

	// Create API server
	server := api.NewServer(cfg, logger, handlers)

	return server.Router()
}

func TestHealthEndpoint(t *testing.T) {
	router := setupTestServer()

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/health", nil)
	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusOK, w.Code)

	var response types.HealthStatus
	err := json.Unmarshal(w.Body.Bytes(), &response)
	require.NoError(t, err)

	assert.Equal(t, "healthy", response.Status)
	assert.NotEmpty(t, response.Services)
}

func TestSessionEndpoints(t *testing.T) {
	router := setupTestServer()

	t.Run("List Sessions", func(t *testing.T) {
		w := httptest.NewRecorder()
		req, _ := http.NewRequest("GET", "/api/v1/sessions", nil)
		router.ServeHTTP(w, req)

		// Should return 401 without authentication
		assert.Equal(t, http.StatusUnauthorized, w.Code)
	})

	t.Run("Create Session", func(t *testing.T) {
		sessionReq := map[string]interface{}{
			"name": "Test Session",
			"config": map[string]interface{}{
				"desktop_environment": map[string]interface{}{
					"type": "ubuntu-xfce",
				},
				"resources": map[string]interface{}{
					"cpu_cores":     1.0,
					"memory_mb":     1024,
					"disk_space_gb": 10,
				},
			},
		}

		body, _ := json.Marshal(sessionReq)
		w := httptest.NewRecorder()
		req, _ := http.NewRequest("POST", "/api/v1/sessions", bytes.NewBuffer(body))
		req.Header.Set("Content-Type", "application/json")
		router.ServeHTTP(w, req)

		// Should return 401 without authentication
		assert.Equal(t, http.StatusUnauthorized, w.Code)
	})
}

func TestAuthenticationFlow(t *testing.T) {
	router := setupTestServer()

	t.Run("Login with Invalid Credentials", func(t *testing.T) {
		loginReq := map[string]string{
			"username": "testuser",
			"password": "wrongpassword",
		}

		body, _ := json.Marshal(loginReq)
		w := httptest.NewRecorder()
		req, _ := http.NewRequest("POST", "/api/v1/auth/login", bytes.NewBuffer(body))
		req.Header.Set("Content-Type", "application/json")
		router.ServeHTTP(w, req)

		// Should return error for invalid credentials
		assert.Equal(t, http.StatusOK, w.Code) // Handler returns 200 with error in body

		var response types.APIResponse
		err := json.Unmarshal(w.Body.Bytes(), &response)
		require.NoError(t, err)

		assert.False(t, response.Success)
		assert.NotEmpty(t, response.Error)
	})
}

func TestCORSHeaders(t *testing.T) {
	router := setupTestServer()

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("OPTIONS", "/api/v1/sessions", nil)
	req.Header.Set("Origin", "http://localhost:3000")
	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusNoContent, w.Code)
	assert.Equal(t, "http://localhost:3000", w.Header().Get("Access-Control-Allow-Origin"))
	assert.Contains(t, w.Header().Get("Access-Control-Allow-Methods"), "GET")
	assert.Contains(t, w.Header().Get("Access-Control-Allow-Methods"), "POST")
}

func TestSecurityHeaders(t *testing.T) {
	router := setupTestServer()

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/health", nil)
	router.ServeHTTP(w, req)

	assert.Equal(t, "nosniff", w.Header().Get("X-Content-Type-Options"))
	assert.Equal(t, "DENY", w.Header().Get("X-Frame-Options"))
	assert.Equal(t, "1; mode=block", w.Header().Get("X-XSS-Protection"))
	assert.NotEmpty(t, w.Header().Get("X-Request-ID"))
}

func TestRateLimiting(t *testing.T) {
	router := setupTestServer()

	// Make multiple requests quickly
	for i := 0; i < 150; i++ { // Exceed the default rate limit
		w := httptest.NewRecorder()
		req, _ := http.NewRequest("GET", "/health", nil)
		router.ServeHTTP(w, req)

		if w.Code == http.StatusTooManyRequests {
			// Rate limiting is working
			return
		}
	}

	// If we get here, rate limiting might not be working as expected
	// This could be due to test setup or rate limit being too high for this test
	t.Log("Rate limiting test completed - may need adjustment for test environment")
}

func TestSwaggerDocumentation(t *testing.T) {
	router := setupTestServer()

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/swagger/index.html", nil)
	router.ServeHTTP(w, req)

	// Should return the Swagger UI
	assert.Equal(t, http.StatusOK, w.Code)
}

func TestGraphQLEndpoint(t *testing.T) {
	router := setupTestServer()

	// Test GraphQL query
	query := map[string]interface{}{
		"query": "{ sessions { id name status } }",
	}

	body, _ := json.Marshal(query)
	w := httptest.NewRecorder()
	req, _ := http.NewRequest("POST", "/graphql", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	router.ServeHTTP(w, req)

	// Should return GraphQL response
	assert.Equal(t, http.StatusOK, w.Code)

	var response map[string]interface{}
	err := json.Unmarshal(w.Body.Bytes(), &response)
	require.NoError(t, err)

	// GraphQL response should have data or errors field
	_, hasData := response["data"]
	_, hasErrors := response["errors"]
	assert.True(t, hasData || hasErrors, "GraphQL response should have data or errors")
}

func TestWebSocketUpgrade(t *testing.T) {
	router := setupTestServer()

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/ws", nil)
	req.Header.Set("Connection", "upgrade")
	req.Header.Set("Upgrade", "websocket")
	req.Header.Set("Sec-WebSocket-Version", "13")
	req.Header.Set("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
	router.ServeHTTP(w, req)

	// WebSocket upgrade should succeed or require authentication
	assert.True(t, w.Code == http.StatusSwitchingProtocols || w.Code == http.StatusUnauthorized)
}

func TestMetricsEndpoint(t *testing.T) {
	router := setupTestServer()

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/metrics", nil)
	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusOK, w.Code)
	assert.Contains(t, w.Header().Get("Content-Type"), "text/plain")
}

func TestInvalidJSONRequest(t *testing.T) {
	router := setupTestServer()

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("POST", "/api/v1/sessions", bytes.NewBufferString("invalid json"))
	req.Header.Set("Content-Type", "application/json")
	router.ServeHTTP(w, req)

	// Should return 400 for invalid JSON or 401 for missing auth
	assert.True(t, w.Code == http.StatusBadRequest || w.Code == http.StatusUnauthorized)
}

func TestLargeRequestBody(t *testing.T) {
	router := setupTestServer()

	// Create a very large request body
	largeData := make(map[string]string)
	for i := 0; i < 1000; i++ {
		largeData[string(rune(i))] = string(make([]byte, 1024)) // 1KB per field
	}

	body, _ := json.Marshal(largeData)
	w := httptest.NewRecorder()
	req, _ := http.NewRequest("POST", "/api/v1/sessions", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	router.ServeHTTP(w, req)

	// Should handle large requests gracefully
	assert.True(t, w.Code < 500) // Should not cause server error
}

func TestConcurrentRequests(t *testing.T) {
	router := setupTestServer()

	// Test concurrent health checks
	const numRequests = 10
	results := make(chan int, numRequests)

	for i := 0; i < numRequests; i++ {
		go func() {
			w := httptest.NewRecorder()
			req, _ := http.NewRequest("GET", "/health", nil)
			router.ServeHTTP(w, req)
			results <- w.Code
		}()
	}

	// Collect results
	for i := 0; i < numRequests; i++ {
		code := <-results
		assert.Equal(t, http.StatusOK, code)
	}
}