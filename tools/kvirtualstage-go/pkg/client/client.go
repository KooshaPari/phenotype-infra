package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"path"
	"time"

	"github.com/kvirtualstage/kvirtualstage-go/pkg/types"
)

// Client represents the KVirtualStage API client
type Client struct {
	baseURL    string
	httpClient *http.Client
	authToken  string
}

// NewClient creates a new API client
func NewClient(baseURL, authToken string) *Client {
	return &Client{
		baseURL: baseURL,
		httpClient: &http.Client{
			Timeout: 30 * time.Second,
		},
		authToken: authToken,
	}
}

// SetAuthToken sets the authentication token
func (c *Client) SetAuthToken(token string) {
	c.authToken = token
}

// SetTimeout sets the HTTP client timeout
func (c *Client) SetTimeout(timeout time.Duration) {
	c.httpClient.Timeout = timeout
}

// Session API methods

// ListSessions retrieves all sessions
func (c *Client) ListSessions() ([]types.Session, error) {
	var response struct {
		Success bool             `json:"success"`
		Data    []types.Session  `json:"data"`
		Error   string           `json:"error,omitempty"`
	}

	err := c.makeRequest("GET", "/api/v1/sessions", nil, &response)
	if err != nil {
		return nil, err
	}

	if !response.Success {
		return nil, fmt.Errorf("API error: %s", response.Error)
	}

	return response.Data, nil
}

// GetSession retrieves a specific session
func (c *Client) GetSession(sessionID string) (*types.Session, error) {
	var response struct {
		Success bool           `json:"success"`
		Data    types.Session  `json:"data"`
		Error   string         `json:"error,omitempty"`
	}

	endpoint := fmt.Sprintf("/api/v1/sessions/%s", sessionID)
	err := c.makeRequest("GET", endpoint, nil, &response)
	if err != nil {
		return nil, err
	}

	if !response.Success {
		return nil, fmt.Errorf("API error: %s", response.Error)
	}

	return &response.Data, nil
}

// CreateSession creates a new session
func (c *Client) CreateSession(name string, config types.SessionConfig) (*types.Session, error) {
	request := struct {
		Name   string                `json:"name"`
		Config types.SessionConfig  `json:"config"`
	}{
		Name:   name,
		Config: config,
	}

	var response struct {
		Success bool           `json:"success"`
		Data    types.Session  `json:"data"`
		Error   string         `json:"error,omitempty"`
	}

	err := c.makeRequest("POST", "/api/v1/sessions", request, &response)
	if err != nil {
		return nil, err
	}

	if !response.Success {
		return nil, fmt.Errorf("API error: %s", response.Error)
	}

	return &response.Data, nil
}

// StartSession starts a session
func (c *Client) StartSession(sessionID string) error {
	var response struct {
		Success bool   `json:"success"`
		Error   string `json:"error,omitempty"`
	}

	endpoint := fmt.Sprintf("/api/v1/sessions/%s/start", sessionID)
	err := c.makeRequest("POST", endpoint, nil, &response)
	if err != nil {
		return err
	}

	if !response.Success {
		return fmt.Errorf("API error: %s", response.Error)
	}

	return nil
}

// StopSession stops a session
func (c *Client) StopSession(sessionID string) error {
	var response struct {
		Success bool   `json:"success"`
		Error   string `json:"error,omitempty"`
	}

	endpoint := fmt.Sprintf("/api/v1/sessions/%s/stop", sessionID)
	err := c.makeRequest("POST", endpoint, nil, &response)
	if err != nil {
		return err
	}

	if !response.Success {
		return fmt.Errorf("API error: %s", response.Error)
	}

	return nil
}

// DeleteSession deletes a session
func (c *Client) DeleteSession(sessionID string) error {
	var response struct {
		Success bool   `json:"success"`
		Error   string `json:"error,omitempty"`
	}

	endpoint := fmt.Sprintf("/api/v1/sessions/%s", sessionID)
	err := c.makeRequest("DELETE", endpoint, nil, &response)
	if err != nil {
		return err
	}

	if !response.Success {
		return fmt.Errorf("API error: %s", response.Error)
	}

	return nil
}

// TakeScreenshot takes a screenshot of a session
func (c *Client) TakeScreenshot(sessionID string) ([]byte, error) {
	endpoint := fmt.Sprintf("/api/v1/sessions/%s/screenshot", sessionID)
	
	req, err := c.buildRequest("GET", endpoint, nil)
	if err != nil {
		return nil, err
	}

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("HTTP request failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("HTTP error: %d", resp.StatusCode)
	}

	return io.ReadAll(resp.Body)
}

// GetVNCInfo gets VNC connection information for a session
func (c *Client) GetVNCInfo(sessionID string) (map[string]interface{}, error) {
	var response struct {
		Success bool                   `json:"success"`
		Data    map[string]interface{} `json:"data"`
		Error   string                 `json:"error,omitempty"`
	}

	endpoint := fmt.Sprintf("/api/v1/sessions/%s/vnc", sessionID)
	err := c.makeRequest("GET", endpoint, nil, &response)
	if err != nil {
		return nil, err
	}

	if !response.Success {
		return nil, fmt.Errorf("API error: %s", response.Error)
	}

	return response.Data, nil
}

// Automation API methods

// ListAutomationScripts retrieves all automation scripts
func (c *Client) ListAutomationScripts() ([]types.AutomationScript, error) {
	var response struct {
		Success bool                      `json:"success"`
		Data    []types.AutomationScript `json:"data"`
		Error   string                    `json:"error,omitempty"`
	}

	err := c.makeRequest("GET", "/api/v1/automation/scripts", nil, &response)
	if err != nil {
		return nil, err
	}

	if !response.Success {
		return nil, fmt.Errorf("API error: %s", response.Error)
	}

	return response.Data, nil
}

// ExecuteAutomation executes an automation script
func (c *Client) ExecuteAutomation(sessionID, scriptID string) (*types.AutomationResult, error) {
	request := struct {
		SessionID string `json:"session_id"`
		ScriptID  string `json:"script_id"`
	}{
		SessionID: sessionID,
		ScriptID:  scriptID,
	}

	var response struct {
		Success bool                   `json:"success"`
		Data    types.AutomationResult `json:"data"`
		Error   string                 `json:"error,omitempty"`
	}

	err := c.makeRequest("POST", "/api/v1/automation/executions", request, &response)
	if err != nil {
		return nil, err
	}

	if !response.Success {
		return nil, fmt.Errorf("API error: %s", response.Error)
	}

	return &response.Data, nil
}

// Recording API methods

// ListRecordings retrieves all recordings
func (c *Client) ListRecordings() ([]types.Recording, error) {
	var response struct {
		Success bool              `json:"success"`
		Data    []types.Recording `json:"data"`
		Error   string            `json:"error,omitempty"`
	}

	err := c.makeRequest("GET", "/api/v1/recordings", nil, &response)
	if err != nil {
		return nil, err
	}

	if !response.Success {
		return nil, fmt.Errorf("API error: %s", response.Error)
	}

	return response.Data, nil
}

// StartRecording starts recording a session
func (c *Client) StartRecording(sessionID, name, format string) (*types.Recording, error) {
	request := struct {
		SessionID string `json:"session_id"`
		Name      string `json:"name"`
		Format    string `json:"format"`
	}{
		SessionID: sessionID,
		Name:      name,
		Format:    format,
	}

	var response struct {
		Success bool            `json:"success"`
		Data    types.Recording `json:"data"`
		Error   string          `json:"error,omitempty"`
	}

	err := c.makeRequest("POST", "/api/v1/recordings", request, &response)
	if err != nil {
		return nil, err
	}

	if !response.Success {
		return nil, fmt.Errorf("API error: %s", response.Error)
	}

	return &response.Data, nil
}

// Authentication API methods

// Login authenticates a user
func (c *Client) Login(username, password string) (string, error) {
	request := struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}{
		Username: username,
		Password: password,
	}

	var response struct {
		Success bool   `json:"success"`
		Data    struct {
			Token string `json:"token"`
		} `json:"data"`
		Error string `json:"error,omitempty"`
	}

	err := c.makeRequest("POST", "/api/v1/auth/login", request, &response)
	if err != nil {
		return "", err
	}

	if !response.Success {
		return "", fmt.Errorf("API error: %s", response.Error)
	}

	c.authToken = response.Data.Token
	return response.Data.Token, nil
}

// System API methods

// GetSystemInfo retrieves system information
func (c *Client) GetSystemInfo() (map[string]interface{}, error) {
	var response struct {
		Success bool                   `json:"success"`
		Data    map[string]interface{} `json:"data"`
		Error   string                 `json:"error,omitempty"`
	}

	err := c.makeRequest("GET", "/api/v1/system/info", nil, &response)
	if err != nil {
		return nil, err
	}

	if !response.Success {
		return nil, fmt.Errorf("API error: %s", response.Error)
	}

	return response.Data, nil
}

// Helper methods

// makeRequest makes an HTTP request and decodes the JSON response
func (c *Client) makeRequest(method, endpoint string, body interface{}, result interface{}) error {
	req, err := c.buildRequest(method, endpoint, body)
	if err != nil {
		return err
	}

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return fmt.Errorf("HTTP request failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("HTTP error: %d", resp.StatusCode)
	}

	if result != nil {
		return json.NewDecoder(resp.Body).Decode(result)
	}

	return nil
}

// buildRequest builds an HTTP request
func (c *Client) buildRequest(method, endpoint string, body interface{}) (*http.Request, error) {
	u, err := url.Parse(c.baseURL)
	if err != nil {
		return nil, fmt.Errorf("invalid base URL: %w", err)
	}

	u.Path = path.Join(u.Path, endpoint)

	var bodyReader io.Reader
	if body != nil {
		jsonBody, err := json.Marshal(body)
		if err != nil {
			return nil, fmt.Errorf("failed to marshal request body: %w", err)
		}
		bodyReader = bytes.NewReader(jsonBody)
	}

	req, err := http.NewRequest(method, u.String(), bodyReader)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	if body != nil {
		req.Header.Set("Content-Type", "application/json")
	}

	if c.authToken != "" {
		req.Header.Set("Authorization", "Bearer "+c.authToken)
	}

	req.Header.Set("User-Agent", "KVirtualStage-CLI/1.0.0")

	return req, nil
}