package types

import (
	"time"
)

// Session represents a virtual desktop session
type Session struct {
	ID          string                 `json:"id" bson:"_id"`
	Name        string                 `json:"name" bson:"name"`
	Status      SessionStatus          `json:"status" bson:"status"`
	Config      SessionConfig          `json:"config" bson:"config"`
	ContainerID string                 `json:"container_id" bson:"container_id"`
	CreatedAt   time.Time              `json:"created_at" bson:"created_at"`
	UpdatedAt   time.Time              `json:"updated_at" bson:"updated_at"`
	ExpiresAt   *time.Time             `json:"expires_at,omitempty" bson:"expires_at,omitempty"`
	Metadata    map[string]interface{} `json:"metadata,omitempty" bson:"metadata,omitempty"`
	UserID      string                 `json:"user_id" bson:"user_id"`
}

// SessionStatus represents the current status of a session
type SessionStatus string

const (
	SessionStatusCreating  SessionStatus = "creating"
	SessionStatusStarting  SessionStatus = "starting"
	SessionStatusRunning   SessionStatus = "running"
	SessionStatusPaused    SessionStatus = "paused"
	SessionStatusStopping  SessionStatus = "stopping"
	SessionStatusStopped   SessionStatus = "stopped"
	SessionStatusError     SessionStatus = "error"
	SessionStatusDestroyed SessionStatus = "destroyed"
)

// SessionConfig defines the configuration for a virtual desktop session
type SessionConfig struct {
	DesktopEnvironment DesktopEnvironment `json:"desktop_environment" bson:"desktop_environment"`
	Resources          ResourceLimits     `json:"resources" bson:"resources"`
	Display            DisplayConfig      `json:"display" bson:"display"`
	Audio              AudioConfig        `json:"audio" bson:"audio"`
	Network            NetworkConfig      `json:"network" bson:"network"`
	Security           SecurityConfig     `json:"security" bson:"security"`
	Applications       []string           `json:"applications,omitempty" bson:"applications,omitempty"`
	AutoDestroy        bool               `json:"auto_destroy" bson:"auto_destroy"`
	MaxDuration        time.Duration      `json:"max_duration" bson:"max_duration"`
}

// DesktopEnvironment specifies the desktop environment configuration
type DesktopEnvironment struct {
	Type    string            `json:"type" bson:"type"` // ubuntu-xfce, ubuntu-gnome, kubuntu-kde, windows-10
	Version string            `json:"version" bson:"version"`
	Image   string            `json:"image" bson:"image"`
	Env     map[string]string `json:"env,omitempty" bson:"env,omitempty"`
}

// ResourceLimits defines resource constraints for a session
type ResourceLimits struct {
	CPUCores    float64 `json:"cpu_cores" bson:"cpu_cores"`
	MemoryMB    int64   `json:"memory_mb" bson:"memory_mb"`
	DiskSpaceGB int64   `json:"disk_space_gb" bson:"disk_space_gb"`
	NetworkMbps int64   `json:"network_mbps" bson:"network_mbps"`
}

// DisplayConfig defines virtual display settings
type DisplayConfig struct {
	Width       int    `json:"width" bson:"width"`
	Height      int    `json:"height" bson:"height"`
	Depth       int    `json:"depth" bson:"depth"`
	DPI         int    `json:"dpi" bson:"dpi"`
	VNCEnabled  bool   `json:"vnc_enabled" bson:"vnc_enabled"`
	VNCPort     int    `json:"vnc_port" bson:"vnc_port"`
	VNCPassword string `json:"vnc_password,omitempty" bson:"vnc_password,omitempty"`
	WebRTC      bool   `json:"webrtc" bson:"webrtc"`
}

// AudioConfig defines virtual audio settings
type AudioConfig struct {
	Enabled     bool   `json:"enabled" bson:"enabled"`
	Driver      string `json:"driver" bson:"driver"`
	SampleRate  int    `json:"sample_rate" bson:"sample_rate"`
	Channels    int    `json:"channels" bson:"channels"`
	BufferSize  int    `json:"buffer_size" bson:"buffer_size"`
	PulseAudio  bool   `json:"pulse_audio" bson:"pulse_audio"`
}

// NetworkConfig defines network settings
type NetworkConfig struct {
	InternetAccess bool     `json:"internet_access" bson:"internet_access"`
	AllowedDomains []string `json:"allowed_domains,omitempty" bson:"allowed_domains,omitempty"`
	BlockedDomains []string `json:"blocked_domains,omitempty" bson:"blocked_domains,omitempty"`
	DNSServers     []string `json:"dns_servers,omitempty" bson:"dns_servers,omitempty"`
}

// SecurityConfig defines security settings
type SecurityConfig struct {
	Isolation        bool     `json:"isolation" bson:"isolation"`
	ReadOnlyRootFS   bool     `json:"readonly_rootfs" bson:"readonly_rootfs"`
	NoNewPrivileges  bool     `json:"no_new_privileges" bson:"no_new_privileges"`
	AppArmorProfile  string   `json:"apparmor_profile,omitempty" bson:"apparmor_profile,omitempty"`
	SeccompProfile   string   `json:"seccomp_profile,omitempty" bson:"seccomp_profile,omitempty"`
	Capabilities     []string `json:"capabilities,omitempty" bson:"capabilities,omitempty"`
}

// AutomationScript represents an automation workflow
type AutomationScript struct {
	ID          string                 `json:"id" bson:"_id"`
	Name        string                 `json:"name" bson:"name"`
	Description string                 `json:"description" bson:"description"`
	Steps       []AutomationStep       `json:"steps" bson:"steps"`
	Variables   map[string]interface{} `json:"variables,omitempty" bson:"variables,omitempty"`
	CreatedAt   time.Time              `json:"created_at" bson:"created_at"`
	UpdatedAt   time.Time              `json:"updated_at" bson:"updated_at"`
	UserID      string                 `json:"user_id" bson:"user_id"`
}

// AutomationStep represents a single step in an automation workflow
type AutomationStep struct {
	ID          int                    `json:"id" bson:"id"`
	Type        string                 `json:"type" bson:"type"` // click, type, wait, screenshot, etc.
	Selector    string                 `json:"selector,omitempty" bson:"selector,omitempty"`
	Value       string                 `json:"value,omitempty" bson:"value,omitempty"`
	Coordinates *Coordinates           `json:"coordinates,omitempty" bson:"coordinates,omitempty"`
	Options     map[string]interface{} `json:"options,omitempty" bson:"options,omitempty"`
	WaitTime    time.Duration          `json:"wait_time,omitempty" bson:"wait_time,omitempty"`
	Retry       *RetryConfig           `json:"retry,omitempty" bson:"retry,omitempty"`
}

// Coordinates represents x,y coordinates
type Coordinates struct {
	X int `json:"x" bson:"x"`
	Y int `json:"y" bson:"y"`
}

// RetryConfig defines retry behavior for automation steps
type RetryConfig struct {
	MaxAttempts int           `json:"max_attempts" bson:"max_attempts"`
	Delay       time.Duration `json:"delay" bson:"delay"`
	BackoffRate float64       `json:"backoff_rate" bson:"backoff_rate"`
}

// AutomationResult represents the result of an automation execution
type AutomationResult struct {
	ID          string                 `json:"id" bson:"_id"`
	SessionID   string                 `json:"session_id" bson:"session_id"`
	ScriptID    string                 `json:"script_id" bson:"script_id"`
	Status      ExecutionStatus        `json:"status" bson:"status"`
	StartedAt   time.Time              `json:"started_at" bson:"started_at"`
	CompletedAt *time.Time             `json:"completed_at,omitempty" bson:"completed_at,omitempty"`
	StepResults []StepResult           `json:"step_results" bson:"step_results"`
	Error       string                 `json:"error,omitempty" bson:"error,omitempty"`
	Screenshots []string               `json:"screenshots,omitempty" bson:"screenshots,omitempty"`
	Metadata    map[string]interface{} `json:"metadata,omitempty" bson:"metadata,omitempty"`
}

// ExecutionStatus represents the status of an automation execution
type ExecutionStatus string

const (
	ExecutionStatusPending   ExecutionStatus = "pending"
	ExecutionStatusRunning   ExecutionStatus = "running"
	ExecutionStatusCompleted ExecutionStatus = "completed"
	ExecutionStatusFailed    ExecutionStatus = "failed"
	ExecutionStatusCancelled ExecutionStatus = "cancelled"
)

// StepResult represents the result of a single automation step
type StepResult struct {
	StepID      int           `json:"step_id" bson:"step_id"`
	Status      StepStatus    `json:"status" bson:"status"`
	StartedAt   time.Time     `json:"started_at" bson:"started_at"`
	CompletedAt *time.Time    `json:"completed_at,omitempty" bson:"completed_at,omitempty"`
	Duration    time.Duration `json:"duration" bson:"duration"`
	Error       string        `json:"error,omitempty" bson:"error,omitempty"`
	Screenshot  string        `json:"screenshot,omitempty" bson:"screenshot,omitempty"`
	Output      interface{}   `json:"output,omitempty" bson:"output,omitempty"`
	Attempts    int           `json:"attempts" bson:"attempts"`
}

// StepStatus represents the status of an automation step
type StepStatus string

const (
	StepStatusPending   StepStatus = "pending"
	StepStatusRunning   StepStatus = "running"
	StepStatusCompleted StepStatus = "completed"
	StepStatusFailed    StepStatus = "failed"
	StepStatusSkipped   StepStatus = "skipped"
)

// Recording represents a session recording
type Recording struct {
	ID        string          `json:"id" bson:"_id"`
	SessionID string          `json:"session_id" bson:"session_id"`
	Name      string          `json:"name" bson:"name"`
	Status    RecordingStatus `json:"status" bson:"status"`
	Format    string          `json:"format" bson:"format"`
	Quality   string          `json:"quality" bson:"quality"`
	FilePath  string          `json:"file_path" bson:"file_path"`
	FileSize  int64           `json:"file_size" bson:"file_size"`
	Duration  time.Duration   `json:"duration" bson:"duration"`
	StartedAt time.Time       `json:"started_at" bson:"started_at"`
	EndedAt   *time.Time      `json:"ended_at,omitempty" bson:"ended_at,omitempty"`
	UserID    string          `json:"user_id" bson:"user_id"`
}

// RecordingStatus represents the status of a recording
type RecordingStatus string

const (
	RecordingStatusStarting  RecordingStatus = "starting"
	RecordingStatusRecording RecordingStatus = "recording"
	RecordingStatusStopping  RecordingStatus = "stopping"
	RecordingStatusCompleted RecordingStatus = "completed"
	RecordingStatusFailed    RecordingStatus = "failed"
)

// User represents a system user
type User struct {
	ID        string    `json:"id" bson:"_id"`
	Username  string    `json:"username" bson:"username"`
	Email     string    `json:"email" bson:"email"`
	Role      UserRole  `json:"role" bson:"role"`
	CreatedAt time.Time `json:"created_at" bson:"created_at"`
	UpdatedAt time.Time `json:"updated_at" bson:"updated_at"`
	LastLogin *time.Time `json:"last_login,omitempty" bson:"last_login,omitempty"`
	Active    bool      `json:"active" bson:"active"`
}

// UserRole represents user role
type UserRole string

const (
	UserRoleAdmin    UserRole = "admin"
	UserRoleUser     UserRole = "user"
	UserRoleOperator UserRole = "operator"
	UserRoleViewer   UserRole = "viewer"
)

// APIResponse represents a standard API response
type APIResponse struct {
	Success bool        `json:"success"`
	Data    interface{} `json:"data,omitempty"`
	Error   string      `json:"error,omitempty"`
	Message string      `json:"message,omitempty"`
}

// PaginatedResponse represents a paginated API response
type PaginatedResponse struct {
	Success    bool        `json:"success"`
	Data       interface{} `json:"data"`
	Pagination Pagination `json:"pagination"`
	Error      string      `json:"error,omitempty"`
}

// Pagination represents pagination metadata
type Pagination struct {
	Page       int `json:"page"`
	Limit      int `json:"limit"`
	Total      int `json:"total"`
	TotalPages int `json:"total_pages"`
}

// HealthStatus represents system health status
type HealthStatus struct {
	Status    string                 `json:"status"`
	Timestamp time.Time              `json:"timestamp"`
	Services  map[string]ServiceHealth `json:"services"`
	Version   string                 `json:"version"`
	Uptime    time.Duration          `json:"uptime"`
}

// ServiceHealth represents individual service health
type ServiceHealth struct {
	Status    string        `json:"status"`
	Message   string        `json:"message,omitempty"`
	Duration  time.Duration `json:"duration"`
	Timestamp time.Time     `json:"timestamp"`
}

// Metrics represents system metrics
type Metrics struct {
	Timestamp      time.Time            `json:"timestamp"`
	Sessions       SessionMetrics       `json:"sessions"`
	Resources      ResourceMetrics      `json:"resources"`
	Automations    AutomationMetrics    `json:"automations"`
	Recordings     RecordingMetrics     `json:"recordings"`
	Performance    PerformanceMetrics   `json:"performance"`
}

// SessionMetrics represents session-related metrics
type SessionMetrics struct {
	Total   int `json:"total"`
	Active  int `json:"active"`
	Created int `json:"created_today"`
	Failed  int `json:"failed_today"`
}

// ResourceMetrics represents resource usage metrics
type ResourceMetrics struct {
	CPUUsage    float64 `json:"cpu_usage"`
	MemoryUsage float64 `json:"memory_usage"`
	DiskUsage   float64 `json:"disk_usage"`
	NetworkIO   NetworkIO `json:"network_io"`
}

// NetworkIO represents network I/O metrics
type NetworkIO struct {
	BytesIn  int64 `json:"bytes_in"`
	BytesOut int64 `json:"bytes_out"`
}

// AutomationMetrics represents automation-related metrics
type AutomationMetrics struct {
	ExecutionsToday int     `json:"executions_today"`
	SuccessRate     float64 `json:"success_rate"`
	AvgDuration     time.Duration `json:"avg_duration"`
	TotalSteps      int     `json:"total_steps"`
}

// RecordingMetrics represents recording-related metrics
type RecordingMetrics struct {
	ActiveRecordings int   `json:"active_recordings"`
	TotalRecordings  int   `json:"total_recordings"`
	TotalSize        int64 `json:"total_size"`
	TotalDuration    time.Duration `json:"total_duration"`
}

// PerformanceMetrics represents performance metrics
type PerformanceMetrics struct {
	ResponseTime    time.Duration `json:"response_time"`
	Throughput      float64       `json:"throughput"`
	ErrorRate       float64       `json:"error_rate"`
	Availability    float64       `json:"availability"`
}

// WebSocketMessage represents WebSocket message structure
type WebSocketMessage struct {
	Type      string      `json:"type"`
	SessionID string      `json:"session_id,omitempty"`
	Data      interface{} `json:"data"`
	Timestamp time.Time   `json:"timestamp"`
}

// Event represents a system event
type Event struct {
	ID        string                 `json:"id"`
	Type      string                 `json:"type"`
	Source    string                 `json:"source"`
	SessionID string                 `json:"session_id,omitempty"`
	UserID    string                 `json:"user_id,omitempty"`
	Data      map[string]interface{} `json:"data"`
	Timestamp time.Time              `json:"timestamp"`
}