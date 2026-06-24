package websocket

import (
	"encoding/json"
	"net/http"
	"sync"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/gorilla/websocket"
	"github.com/google/uuid"
	"github.com/kvirtualstage/kvirtualstage-go/internal/config"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/types"
	"github.com/sirupsen/logrus"
)

// Handler handles WebSocket connections
type Handler struct {
	config    *config.Config
	logger    *logrus.Logger
	upgrader  websocket.Upgrader
	clients   map[string]*Client
	clientsMu sync.RWMutex
	hub       *Hub
}

// Client represents a WebSocket client connection
type Client struct {
	ID         string
	Conn       *websocket.Conn
	Send       chan []byte
	UserID     string
	SessionID  string
	LastPing   time.Time
	Hub        *Hub
}

// Hub manages WebSocket connections and message broadcasting
type Hub struct {
	clients    map[*Client]bool
	register   chan *Client
	unregister chan *Client
	broadcast  chan []byte
	logger     *logrus.Logger
}

// NewHandler creates a new WebSocket handler
func NewHandler(cfg *config.Config, logger *logrus.Logger) *Handler {
	hub := &Hub{
		clients:    make(map[*Client]bool),
		register:   make(chan *Client),
		unregister: make(chan *Client),
		broadcast:  make(chan []byte),
		logger:     logger,
	}

	handler := &Handler{
		config: cfg,
		logger: logger,
		upgrader: websocket.Upgrader{
			ReadBufferSize:  cfg.Server.WebSocket.ReadBufferSize,
			WriteBufferSize: cfg.Server.WebSocket.WriteBufferSize,
			CheckOrigin: func(r *http.Request) bool {
				// TODO: Implement proper origin checking
				return true
			},
		},
		clients: make(map[string]*Client),
		hub:     hub,
	}

	// Start the hub
	go hub.run()

	return handler
}

// Handle handles WebSocket upgrade and connection
func (h *Handler) Handle(c *gin.Context) {
	// Upgrade HTTP connection to WebSocket
	conn, err := h.upgrader.Upgrade(c.Writer, c.Request, nil)
	if err != nil {
		h.logger.Errorf("WebSocket upgrade failed: %v", err)
		return
	}

	// Create client
	client := &Client{
		ID:        uuid.New().String(),
		Conn:      conn,
		Send:      make(chan []byte, 256),
		UserID:    c.GetString("user_id"),
		SessionID: c.Query("session_id"),
		LastPing:  time.Now(),
		Hub:       h.hub,
	}

	// Register client
	h.clientsMu.Lock()
	h.clients[client.ID] = client
	h.clientsMu.Unlock()

	h.hub.register <- client

	// Start goroutines for this client
	go client.writePump()
	go client.readPump()

	h.logger.Infof("WebSocket client connected: %s", client.ID)
}

// BroadcastToSession broadcasts a message to all clients connected to a session
func (h *Handler) BroadcastToSession(sessionID string, message types.WebSocketMessage) {
	data, err := json.Marshal(message)
	if err != nil {
		h.logger.Errorf("Failed to marshal WebSocket message: %v", err)
		return
	}

	h.clientsMu.RLock()
	defer h.clientsMu.RUnlock()

	for _, client := range h.clients {
		if client.SessionID == sessionID {
			select {
			case client.Send <- data:
			default:
				close(client.Send)
				delete(h.clients, client.ID)
			}
		}
	}
}

// BroadcastToUser broadcasts a message to all clients connected by a user
func (h *Handler) BroadcastToUser(userID string, message types.WebSocketMessage) {
	data, err := json.Marshal(message)
	if err != nil {
		h.logger.Errorf("Failed to marshal WebSocket message: %v", err)
		return
	}

	h.clientsMu.RLock()
	defer h.clientsMu.RUnlock()

	for _, client := range h.clients {
		if client.UserID == userID {
			select {
			case client.Send <- data:
			default:
				close(client.Send)
				delete(h.clients, client.ID)
			}
		}
	}
}

// BroadcastToAll broadcasts a message to all connected clients
func (h *Handler) BroadcastToAll(message types.WebSocketMessage) {
	data, err := json.Marshal(message)
	if err != nil {
		h.logger.Errorf("Failed to marshal WebSocket message: %v", err)
		return
	}

	h.hub.broadcast <- data
}

// Hub methods

// run runs the hub's main loop
func (h *Hub) run() {
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case client := <-h.register:
			h.clients[client] = true
			h.logger.Infof("Client registered: %s", client.ID)

			// Send welcome message
			welcome := types.WebSocketMessage{
				Type:      "welcome",
				Data:      map[string]string{"client_id": client.ID},
				Timestamp: time.Now(),
			}
			if data, err := json.Marshal(welcome); err == nil {
				select {
				case client.Send <- data:
				default:
					close(client.Send)
					delete(h.clients, client)
				}
			}

		case client := <-h.unregister:
			if _, ok := h.clients[client]; ok {
				delete(h.clients, client)
				close(client.Send)
				h.logger.Infof("Client unregistered: %s", client.ID)
			}

		case message := <-h.broadcast:
			for client := range h.clients {
				select {
				case client.Send <- message:
				default:
					close(client.Send)
					delete(h.clients, client)
				}
			}

		case <-ticker.C:
			// Ping all clients to check if they're still alive
			h.pingClients()
		}
	}
}

// pingClients sends ping to all clients
func (h *Hub) pingClients() {
	ping := types.WebSocketMessage{
		Type:      "ping",
		Timestamp: time.Now(),
	}

	if data, err := json.Marshal(ping); err == nil {
		for client := range h.clients {
			select {
			case client.Send <- data:
			default:
				close(client.Send)
				delete(h.clients, client)
			}
		}
	}
}

// Client methods

// readPump pumps messages from the WebSocket connection to the hub
func (c *Client) readPump() {
	defer func() {
		c.Hub.unregister <- c
		c.Conn.Close()
	}()

	c.Conn.SetReadLimit(512)
	c.Conn.SetReadDeadline(time.Now().Add(60 * time.Second))
	c.Conn.SetPongHandler(func(string) error {
		c.Conn.SetReadDeadline(time.Now().Add(60 * time.Second))
		c.LastPing = time.Now()
		return nil
	})

	for {
		_, message, err := c.Conn.ReadMessage()
		if err != nil {
			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
				c.Hub.logger.Errorf("WebSocket error: %v", err)
			}
			break
		}

		// Handle incoming message
		c.handleMessage(message)
	}
}

// writePump pumps messages from the hub to the WebSocket connection
func (c *Client) writePump() {
	ticker := time.NewTicker(54 * time.Second)
	defer func() {
		ticker.Stop()
		c.Conn.Close()
	}()

	for {
		select {
		case message, ok := <-c.Send:
			c.Conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
			if !ok {
				c.Conn.WriteMessage(websocket.CloseMessage, []byte{})
				return
			}

			w, err := c.Conn.NextWriter(websocket.TextMessage)
			if err != nil {
				return
			}
			w.Write(message)

			// Add queued messages to the current WebSocket message
			n := len(c.Send)
			for i := 0; i < n; i++ {
				w.Write([]byte{'\n'})
				w.Write(<-c.Send)
			}

			if err := w.Close(); err != nil {
				return
			}

		case <-ticker.C:
			c.Conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
			if err := c.Conn.WriteMessage(websocket.PingMessage, nil); err != nil {
				return
			}
		}
	}
}

// handleMessage handles incoming WebSocket messages
func (c *Client) handleMessage(data []byte) {
	var msg types.WebSocketMessage
	if err := json.Unmarshal(data, &msg); err != nil {
		c.Hub.logger.Errorf("Failed to unmarshal WebSocket message: %v", err)
		return
	}

	msg.Timestamp = time.Now()

	switch msg.Type {
	case "pong":
		c.LastPing = time.Now()

	case "subscribe_session":
		if sessionID, ok := msg.Data.(string); ok {
			c.SessionID = sessionID
			c.Hub.logger.Infof("Client %s subscribed to session %s", c.ID, sessionID)
		}

	case "unsubscribe_session":
		c.SessionID = ""
		c.Hub.logger.Infof("Client %s unsubscribed from session", c.ID)

	case "session_command":
		// Handle session commands (start, stop, etc.)
		c.handleSessionCommand(msg)

	default:
		c.Hub.logger.Warnf("Unknown WebSocket message type: %s", msg.Type)
	}
}

// handleSessionCommand handles session-related commands
func (c *Client) handleSessionCommand(msg types.WebSocketMessage) {
	command, ok := msg.Data.(map[string]interface{})
	if !ok {
		return
	}

	action, _ := command["action"].(string)
	sessionID, _ := command["session_id"].(string)

	switch action {
	case "take_screenshot":
		// TODO: Implement screenshot taking
		response := types.WebSocketMessage{
			Type:      "session_command_response",
			Data:      map[string]interface{}{"action": action, "status": "success"},
			Timestamp: time.Now(),
		}
		if data, err := json.Marshal(response); err == nil {
			c.Send <- data
		}

	case "get_status":
		// TODO: Implement status retrieval
		response := types.WebSocketMessage{
			Type: "session_status",
			Data: map[string]interface{}{
				"session_id": sessionID,
				"status":     "running",
				"uptime":     "00:15:30",
			},
			Timestamp: time.Now(),
		}
		if data, err := json.Marshal(response); err == nil {
			c.Send <- data
		}

	default:
		c.Hub.logger.Warnf("Unknown session command: %s", action)
	}
}

// GetConnectedClients returns the number of connected clients
func (h *Handler) GetConnectedClients() int {
	h.clientsMu.RLock()
	defer h.clientsMu.RUnlock()
	return len(h.clients)
}

// GetClientsBySession returns clients connected to a specific session
func (h *Handler) GetClientsBySession(sessionID string) []*Client {
	h.clientsMu.RLock()
	defer h.clientsMu.RUnlock()

	var clients []*Client
	for _, client := range h.clients {
		if client.SessionID == sessionID {
			clients = append(clients, client)
		}
	}
	return clients
}