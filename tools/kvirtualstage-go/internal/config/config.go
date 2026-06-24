package config

import (
	"fmt"
	"strings"
	"time"

	"github.com/spf13/viper"
)

// Config represents the application configuration
type Config struct {
	Server    ServerConfig    `mapstructure:"server"`
	Database  DatabaseConfig  `mapstructure:"database"`
	Redis     RedisConfig     `mapstructure:"redis"`
	Docker    DockerConfig    `mapstructure:"docker"`
	Security  SecurityConfig  `mapstructure:"security"`
	Recording RecordingConfig `mapstructure:"recording"`
	Logging   LoggingConfig   `mapstructure:"logging"`
	Metrics   MetricsConfig   `mapstructure:"metrics"`
}

// ServerConfig defines server configuration
type ServerConfig struct {
	Host             string        `mapstructure:"host"`
	Port             int           `mapstructure:"port"`
	ReadTimeout      time.Duration `mapstructure:"read_timeout"`
	WriteTimeout     time.Duration `mapstructure:"write_timeout"`
	IdleTimeout      time.Duration `mapstructure:"idle_timeout"`
	ShutdownTimeout  time.Duration `mapstructure:"shutdown_timeout"`
	TLS              TLSConfig     `mapstructure:"tls"`
	CORS             CORSConfig    `mapstructure:"cors"`
	RateLimit        RateLimitConfig `mapstructure:"rate_limit"`
	GraphQL          GraphQLConfig `mapstructure:"graphql"`
	WebSocket        WebSocketConfig `mapstructure:"websocket"`
}

// TLSConfig defines TLS configuration
type TLSConfig struct {
	Enabled  bool   `mapstructure:"enabled"`
	CertFile string `mapstructure:"cert_file"`
	KeyFile  string `mapstructure:"key_file"`
}

// CORSConfig defines CORS configuration
type CORSConfig struct {
	AllowedOrigins   []string `mapstructure:"allowed_origins"`
	AllowedMethods   []string `mapstructure:"allowed_methods"`
	AllowedHeaders   []string `mapstructure:"allowed_headers"`
	ExposedHeaders   []string `mapstructure:"exposed_headers"`
	AllowCredentials bool     `mapstructure:"allow_credentials"`
	MaxAge           int      `mapstructure:"max_age"`
}

// RateLimitConfig defines rate limiting configuration
type RateLimitConfig struct {
	Enabled    bool          `mapstructure:"enabled"`
	RequestsPerMinute int    `mapstructure:"requests_per_minute"`
	BurstSize  int           `mapstructure:"burst_size"`
	CleanupInterval time.Duration `mapstructure:"cleanup_interval"`
}

// GraphQLConfig defines GraphQL configuration
type GraphQLConfig struct {
	Enabled          bool   `mapstructure:"enabled"`
	Playground       bool   `mapstructure:"playground"`
	Introspection    bool   `mapstructure:"introspection"`
	ComplexityLimit  int    `mapstructure:"complexity_limit"`
	DepthLimit       int    `mapstructure:"depth_limit"`
}

// WebSocketConfig defines WebSocket configuration
type WebSocketConfig struct {
	Enabled         bool          `mapstructure:"enabled"`
	ReadBufferSize  int           `mapstructure:"read_buffer_size"`
	WriteBufferSize int           `mapstructure:"write_buffer_size"`
	PingInterval    time.Duration `mapstructure:"ping_interval"`
	PongTimeout     time.Duration `mapstructure:"pong_timeout"`
	MaxConnections  int           `mapstructure:"max_connections"`
}

// DatabaseConfig defines database configuration
type DatabaseConfig struct {
	Type         string        `mapstructure:"type"`
	Host         string        `mapstructure:"host"`
	Port         int           `mapstructure:"port"`
	Database     string        `mapstructure:"database"`
	Username     string        `mapstructure:"username"`
	Password     string        `mapstructure:"password"`
	SSLMode      string        `mapstructure:"ssl_mode"`
	MaxOpenConns int           `mapstructure:"max_open_conns"`
	MaxIdleConns int           `mapstructure:"max_idle_conns"`
	ConnMaxLifetime time.Duration `mapstructure:"conn_max_lifetime"`
	ConnMaxIdleTime time.Duration `mapstructure:"conn_max_idle_time"`
	Migrations   MigrationsConfig `mapstructure:"migrations"`
}

// MigrationsConfig defines database migration configuration
type MigrationsConfig struct {
	Enabled bool   `mapstructure:"enabled"`
	Path    string `mapstructure:"path"`
}

// RedisConfig defines Redis configuration
type RedisConfig struct {
	Host         string        `mapstructure:"host"`
	Port         int           `mapstructure:"port"`
	Password     string        `mapstructure:"password"`
	Database     int           `mapstructure:"database"`
	PoolSize     int           `mapstructure:"pool_size"`
	DialTimeout  time.Duration `mapstructure:"dial_timeout"`
	ReadTimeout  time.Duration `mapstructure:"read_timeout"`
	WriteTimeout time.Duration `mapstructure:"write_timeout"`
	IdleTimeout  time.Duration `mapstructure:"idle_timeout"`
	MaxRetries   int           `mapstructure:"max_retries"`
}

// DockerConfig defines Docker configuration
type DockerConfig struct {
	Host           string            `mapstructure:"host"`
	APIVersion     string            `mapstructure:"api_version"`
	CertPath       string            `mapstructure:"cert_path"`
	TLSVerify      bool              `mapstructure:"tls_verify"`
	Timeout        time.Duration     `mapstructure:"timeout"`
	Registry       RegistryConfig    `mapstructure:"registry"`
	DefaultImages  map[string]string `mapstructure:"default_images"`
	NetworkName    string            `mapstructure:"network_name"`
	VolumeBasePath string            `mapstructure:"volume_base_path"`
	ResourceLimits ResourceLimitsConfig `mapstructure:"resource_limits"`
}

// RegistryConfig defines Docker registry configuration
type RegistryConfig struct {
	URL      string `mapstructure:"url"`
	Username string `mapstructure:"username"`
	Password string `mapstructure:"password"`
}

// ResourceLimitsConfig defines default resource limits
type ResourceLimitsConfig struct {
	DefaultCPU    float64 `mapstructure:"default_cpu"`
	DefaultMemory int64   `mapstructure:"default_memory"`
	DefaultDisk   int64   `mapstructure:"default_disk"`
	MaxCPU        float64 `mapstructure:"max_cpu"`
	MaxMemory     int64   `mapstructure:"max_memory"`
	MaxDisk       int64   `mapstructure:"max_disk"`
}

// SecurityConfig defines security configuration
type SecurityConfig struct {
	JWT          JWTConfig          `mapstructure:"jwt"`
	Encryption   EncryptionConfig   `mapstructure:"encryption"`
	OAuth        OAuthConfig        `mapstructure:"oauth"`
	Session      SessionConfig      `mapstructure:"session"`
	Audit        AuditConfig        `mapstructure:"audit"`
	IPWhitelist  []string           `mapstructure:"ip_whitelist"`
	IPBlacklist  []string           `mapstructure:"ip_blacklist"`
}

// JWTConfig defines JWT configuration
type JWTConfig struct {
	SecretKey      string        `mapstructure:"secret_key"`
	TokenDuration  time.Duration `mapstructure:"token_duration"`
	RefreshDuration time.Duration `mapstructure:"refresh_duration"`
	Issuer         string        `mapstructure:"issuer"`
	Algorithm      string        `mapstructure:"algorithm"`
}

// EncryptionConfig defines encryption configuration
type EncryptionConfig struct {
	Key       string `mapstructure:"key"`
	Algorithm string `mapstructure:"algorithm"`
}

// OAuthConfig defines OAuth configuration
type OAuthConfig struct {
	Enabled      bool                     `mapstructure:"enabled"`
	Providers    map[string]OAuthProvider `mapstructure:"providers"`
	RedirectURL  string                   `mapstructure:"redirect_url"`
}

// OAuthProvider defines OAuth provider configuration
type OAuthProvider struct {
	ClientID     string   `mapstructure:"client_id"`
	ClientSecret string   `mapstructure:"client_secret"`
	Scopes       []string `mapstructure:"scopes"`
	AuthURL      string   `mapstructure:"auth_url"`
	TokenURL     string   `mapstructure:"token_url"`
	UserInfoURL  string   `mapstructure:"user_info_url"`
}

// SessionConfig defines session configuration
type SessionConfig struct {
	DefaultDuration time.Duration `mapstructure:"default_duration"`
	MaxDuration     time.Duration `mapstructure:"max_duration"`
	MaxSessions     int           `mapstructure:"max_sessions"`
	CleanupInterval time.Duration `mapstructure:"cleanup_interval"`
	AutoDestroy     bool          `mapstructure:"auto_destroy"`
}

// AuditConfig defines audit logging configuration
type AuditConfig struct {
	Enabled    bool   `mapstructure:"enabled"`
	LogLevel   string `mapstructure:"log_level"`
	OutputPath string `mapstructure:"output_path"`
	Retention  time.Duration `mapstructure:"retention"`
}

// RecordingConfig defines recording configuration
type RecordingConfig struct {
	Enabled         bool              `mapstructure:"enabled"`
	OutputPath      string            `mapstructure:"output_path"`
	DefaultFormat   string            `mapstructure:"default_format"`
	DefaultQuality  string            `mapstructure:"default_quality"`
	MaxDuration     time.Duration     `mapstructure:"max_duration"`
	MaxFileSize     int64             `mapstructure:"max_file_size"`
	Cleanup         CleanupConfig     `mapstructure:"cleanup"`
	FFmpeg          FFmpegConfig      `mapstructure:"ffmpeg"`
	Formats         map[string]FormatConfig `mapstructure:"formats"`
}

// CleanupConfig defines recording cleanup configuration
type CleanupConfig struct {
	Enabled   bool          `mapstructure:"enabled"`
	Retention time.Duration `mapstructure:"retention"`
	Interval  time.Duration `mapstructure:"interval"`
}

// FFmpegConfig defines FFmpeg configuration
type FFmpegConfig struct {
	BinaryPath       string `mapstructure:"binary_path"`
	HardwareAccel    string `mapstructure:"hardware_accel"`
	Preset           string `mapstructure:"preset"`
	CRF              int    `mapstructure:"crf"`
	Framerate        int    `mapstructure:"framerate"`
	MaxBitrate       string `mapstructure:"max_bitrate"`
	AudioCodec       string `mapstructure:"audio_codec"`
	VideoCodec       string `mapstructure:"video_codec"`
}

// FormatConfig defines format-specific configuration
type FormatConfig struct {
	Extension   string            `mapstructure:"extension"`
	MimeType    string            `mapstructure:"mime_type"`
	VideoCodec  string            `mapstructure:"video_codec"`
	AudioCodec  string            `mapstructure:"audio_codec"`
	Options     map[string]string `mapstructure:"options"`
}

// LoggingConfig defines logging configuration
type LoggingConfig struct {
	Level      string        `mapstructure:"level"`
	Format     string        `mapstructure:"format"`
	Output     []string      `mapstructure:"output"`
	File       FileLogConfig `mapstructure:"file"`
	Structured bool          `mapstructure:"structured"`
}

// FileLogConfig defines file logging configuration
type FileLogConfig struct {
	Path       string `mapstructure:"path"`
	MaxSize    int    `mapstructure:"max_size"`
	MaxBackups int    `mapstructure:"max_backups"`
	MaxAge     int    `mapstructure:"max_age"`
	Compress   bool   `mapstructure:"compress"`
}

// MetricsConfig defines metrics configuration
type MetricsConfig struct {
	Enabled   bool          `mapstructure:"enabled"`
	Path      string        `mapstructure:"path"`
	Interval  time.Duration `mapstructure:"interval"`
	Namespace string        `mapstructure:"namespace"`
	Subsystem string        `mapstructure:"subsystem"`
}

// Load loads configuration from file and environment variables
func Load(configPath string) (*Config, error) {
	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	
	if configPath != "" {
		viper.SetConfigFile(configPath)
	} else {
		viper.AddConfigPath("./configs")
		viper.AddConfigPath(".")
	}

	// Environment variable settings
	viper.SetEnvPrefix("KVS")
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	viper.AutomaticEnv()

	// Set defaults
	setDefaults()

	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			return nil, fmt.Errorf("failed to read config file: %w", err)
		}
	}

	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		return nil, fmt.Errorf("failed to unmarshal config: %w", err)
	}

	return &config, nil
}

// setDefaults sets default configuration values
func setDefaults() {
	// Server defaults
	viper.SetDefault("server.host", "0.0.0.0")
	viper.SetDefault("server.port", 8080)
	viper.SetDefault("server.read_timeout", "30s")
	viper.SetDefault("server.write_timeout", "30s")
	viper.SetDefault("server.idle_timeout", "120s")
	viper.SetDefault("server.shutdown_timeout", "30s")
	
	// TLS defaults
	viper.SetDefault("server.tls.enabled", false)
	
	// CORS defaults
	viper.SetDefault("server.cors.allowed_origins", []string{"*"})
	viper.SetDefault("server.cors.allowed_methods", []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"})
	viper.SetDefault("server.cors.allowed_headers", []string{"*"})
	viper.SetDefault("server.cors.allow_credentials", true)
	viper.SetDefault("server.cors.max_age", 3600)
	
	// Rate limiting defaults
	viper.SetDefault("server.rate_limit.enabled", true)
	viper.SetDefault("server.rate_limit.requests_per_minute", 100)
	viper.SetDefault("server.rate_limit.burst_size", 10)
	viper.SetDefault("server.rate_limit.cleanup_interval", "1m")
	
	// GraphQL defaults
	viper.SetDefault("server.graphql.enabled", true)
	viper.SetDefault("server.graphql.playground", true)
	viper.SetDefault("server.graphql.introspection", true)
	viper.SetDefault("server.graphql.complexity_limit", 200)
	viper.SetDefault("server.graphql.depth_limit", 15)
	
	// WebSocket defaults
	viper.SetDefault("server.websocket.enabled", true)
	viper.SetDefault("server.websocket.read_buffer_size", 1024)
	viper.SetDefault("server.websocket.write_buffer_size", 1024)
	viper.SetDefault("server.websocket.ping_interval", "30s")
	viper.SetDefault("server.websocket.pong_timeout", "10s")
	viper.SetDefault("server.websocket.max_connections", 1000)
	
	// Database defaults
	viper.SetDefault("database.type", "postgres")
	viper.SetDefault("database.host", "localhost")
	viper.SetDefault("database.port", 5432)
	viper.SetDefault("database.database", "kvirtualstage")
	viper.SetDefault("database.username", "kvs")
	viper.SetDefault("database.ssl_mode", "disable")
	viper.SetDefault("database.max_open_conns", 25)
	viper.SetDefault("database.max_idle_conns", 25)
	viper.SetDefault("database.conn_max_lifetime", "1h")
	viper.SetDefault("database.conn_max_idle_time", "15m")
	viper.SetDefault("database.migrations.enabled", true)
	viper.SetDefault("database.migrations.path", "./migrations")
	
	// Redis defaults
	viper.SetDefault("redis.host", "localhost")
	viper.SetDefault("redis.port", 6379)
	viper.SetDefault("redis.database", 0)
	viper.SetDefault("redis.pool_size", 10)
	viper.SetDefault("redis.dial_timeout", "5s")
	viper.SetDefault("redis.read_timeout", "3s")
	viper.SetDefault("redis.write_timeout", "3s")
	viper.SetDefault("redis.idle_timeout", "5m")
	viper.SetDefault("redis.max_retries", 3)
	
	// Docker defaults
	viper.SetDefault("docker.host", "unix:///var/run/docker.sock")
	viper.SetDefault("docker.api_version", "1.41")
	viper.SetDefault("docker.timeout", "30s")
	viper.SetDefault("docker.network_name", "kvirtualstage")
	viper.SetDefault("docker.volume_base_path", "/var/lib/kvirtualstage/volumes")
	viper.SetDefault("docker.resource_limits.default_cpu", 1.0)
	viper.SetDefault("docker.resource_limits.default_memory", 1073741824) // 1GB
	viper.SetDefault("docker.resource_limits.default_disk", 5368709120)   // 5GB
	viper.SetDefault("docker.resource_limits.max_cpu", 4.0)
	viper.SetDefault("docker.resource_limits.max_memory", 8589934592)     // 8GB
	viper.SetDefault("docker.resource_limits.max_disk", 21474836480)      // 20GB
	
	// Security defaults
	viper.SetDefault("security.jwt.token_duration", "24h")
	viper.SetDefault("security.jwt.refresh_duration", "168h") // 7 days
	viper.SetDefault("security.jwt.issuer", "kvirtualstage")
	viper.SetDefault("security.jwt.algorithm", "HS256")
	viper.SetDefault("security.encryption.algorithm", "AES-256-GCM")
	viper.SetDefault("security.session.default_duration", "8h")
	viper.SetDefault("security.session.max_duration", "24h")
	viper.SetDefault("security.session.max_sessions", 10)
	viper.SetDefault("security.session.cleanup_interval", "1h")
	viper.SetDefault("security.session.auto_destroy", true)
	viper.SetDefault("security.audit.enabled", true)
	viper.SetDefault("security.audit.log_level", "info")
	viper.SetDefault("security.audit.retention", "90d")
	
	// Recording defaults
	viper.SetDefault("recording.enabled", true)
	viper.SetDefault("recording.output_path", "./recordings")
	viper.SetDefault("recording.default_format", "mp4")
	viper.SetDefault("recording.default_quality", "high")
	viper.SetDefault("recording.max_duration", "2h")
	viper.SetDefault("recording.max_file_size", 2147483648) // 2GB
	viper.SetDefault("recording.cleanup.enabled", true)
	viper.SetDefault("recording.cleanup.retention", "30d")
	viper.SetDefault("recording.cleanup.interval", "24h")
	viper.SetDefault("recording.ffmpeg.binary_path", "ffmpeg")
	viper.SetDefault("recording.ffmpeg.hardware_accel", "auto")
	viper.SetDefault("recording.ffmpeg.preset", "medium")
	viper.SetDefault("recording.ffmpeg.crf", 23)
	viper.SetDefault("recording.ffmpeg.framerate", 30)
	viper.SetDefault("recording.ffmpeg.video_codec", "libx264")
	viper.SetDefault("recording.ffmpeg.audio_codec", "aac")
	
	// Logging defaults
	viper.SetDefault("logging.level", "info")
	viper.SetDefault("logging.format", "json")
	viper.SetDefault("logging.output", []string{"stdout"})
	viper.SetDefault("logging.structured", true)
	viper.SetDefault("logging.file.max_size", 100)
	viper.SetDefault("logging.file.max_backups", 3)
	viper.SetDefault("logging.file.max_age", 28)
	viper.SetDefault("logging.file.compress", true)
	
	// Metrics defaults
	viper.SetDefault("metrics.enabled", true)
	viper.SetDefault("metrics.path", "/metrics")
	viper.SetDefault("metrics.interval", "15s")
	viper.SetDefault("metrics.namespace", "kvirtualstage")
	viper.SetDefault("metrics.subsystem", "api")
}

// Validate validates the configuration
func (c *Config) Validate() error {
	if c.Server.Port <= 0 || c.Server.Port > 65535 {
		return fmt.Errorf("invalid server port: %d", c.Server.Port)
	}
	
	if c.Security.JWT.SecretKey == "" {
		return fmt.Errorf("JWT secret key is required")
	}
	
	if c.Database.Host == "" {
		return fmt.Errorf("database host is required")
	}
	
	if c.Docker.Host == "" {
		return fmt.Errorf("Docker host is required")
	}
	
	return nil
}

// GetDSN returns the database connection string
func (c *Config) GetDSN() string {
	switch c.Database.Type {
	case "postgres":
		return fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=%s",
			c.Database.Host,
			c.Database.Port,
			c.Database.Username,
			c.Database.Password,
			c.Database.Database,
			c.Database.SSLMode,
		)
	case "mysql":
		return fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=Local",
			c.Database.Username,
			c.Database.Password,
			c.Database.Host,
			c.Database.Port,
			c.Database.Database,
		)
	default:
		return ""
	}
}

// GetRedisAddr returns the Redis address
func (c *Config) GetRedisAddr() string {
	return fmt.Sprintf("%s:%d", c.Redis.Host, c.Redis.Port)
}

// GetServerAddr returns the server address
func (c *Config) GetServerAddr() string {
	return fmt.Sprintf("%s:%d", c.Server.Host, c.Server.Port)
}