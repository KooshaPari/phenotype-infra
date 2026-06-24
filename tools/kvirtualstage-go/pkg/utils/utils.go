package utils

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// GenerateID generates a random hex ID
func GenerateID(length int) string {
	bytes := make([]byte, length/2)
	if _, err := rand.Read(bytes); err != nil {
		// Fallback to timestamp-based ID
		return fmt.Sprintf("%d", time.Now().UnixNano())
	}
	return hex.EncodeToString(bytes)
}

// EnsureDir ensures that a directory exists, creating it if necessary
func EnsureDir(path string) error {
	return os.MkdirAll(path, 0755)
}

// FileExists checks if a file exists
func FileExists(path string) bool {
	_, err := os.Stat(path)
	return !os.IsNotExist(err)
}

// CopyFile copies a file from src to dst
func CopyFile(src, dst string) error {
	sourceFile, err := os.Open(src)
	if err != nil {
		return err
	}
	defer sourceFile.Close()

	// Ensure destination directory exists
	if err := EnsureDir(filepath.Dir(dst)); err != nil {
		return err
	}

	destFile, err := os.Create(dst)
	if err != nil {
		return err
	}
	defer destFile.Close()

	_, err = io.Copy(destFile, sourceFile)
	return err
}

// SanitizeFilename removes dangerous characters from filenames
func SanitizeFilename(filename string) string {
	// Replace dangerous characters
	dangerous := []string{"/", "\\", ":", "*", "?", "\"", "<", ">", "|"}
	sanitized := filename
	for _, char := range dangerous {
		sanitized = strings.ReplaceAll(sanitized, char, "_")
	}
	
	// Trim spaces and dots
	sanitized = strings.Trim(sanitized, " .")
	
	// Ensure not empty
	if sanitized == "" {
		sanitized = "unnamed"
	}
	
	return sanitized
}

// FormatBytes formats bytes into human-readable format
func FormatBytes(bytes int64) string {
	const unit = 1024
	if bytes < unit {
		return fmt.Sprintf("%d B", bytes)
	}
	
	div, exp := int64(unit), 0
	for n := bytes / unit; n >= unit; n /= unit {
		div *= unit
		exp++
	}
	
	units := []string{"KB", "MB", "GB", "TB", "PB"}
	return fmt.Sprintf("%.1f %s", float64(bytes)/float64(div), units[exp])
}

// FormatDuration formats duration into human-readable format
func FormatDuration(duration time.Duration) string {
	if duration < time.Minute {
		return fmt.Sprintf("%.0fs", duration.Seconds())
	}
	if duration < time.Hour {
		return fmt.Sprintf("%.0fm %.0fs", duration.Minutes(), duration.Seconds()%60)
	}
	return fmt.Sprintf("%.0fh %.0fm", duration.Hours(), duration.Minutes()%60)
}

// TruncateString truncates a string to specified length with ellipsis
func TruncateString(str string, length int) string {
	if len(str) <= length {
		return str
	}
	if length <= 3 {
		return str[:length]
	}
	return str[:length-3] + "..."
}

// StringInSlice checks if string exists in slice
func StringInSlice(str string, slice []string) bool {
	for _, s := range slice {
		if s == str {
			return true
		}
	}
	return false
}

// UniqueStrings removes duplicates from string slice
func UniqueStrings(slice []string) []string {
	seen := make(map[string]bool)
	result := []string{}
	
	for _, str := range slice {
		if !seen[str] {
			seen[str] = true
			result = append(result, str)
		}
	}
	
	return result
}

// GetEnvOrDefault gets environment variable or returns default value
func GetEnvOrDefault(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

// IsValidPort checks if port number is valid
func IsValidPort(port int) bool {
	return port > 0 && port <= 65535
}

// ParseDurationOrDefault parses duration string or returns default
func ParseDurationOrDefault(durationStr string, defaultDuration time.Duration) time.Duration {
	if duration, err := time.ParseDuration(durationStr); err == nil {
		return duration
	}
	return defaultDuration
}

// RetryWithBackoff retries a function with exponential backoff
func RetryWithBackoff(fn func() error, maxRetries int, initialDelay time.Duration) error {
	var err error
	delay := initialDelay
	
	for i := 0; i < maxRetries; i++ {
		if err = fn(); err == nil {
			return nil
		}
		
		if i < maxRetries-1 {
			time.Sleep(delay)
			delay *= 2 // Exponential backoff
		}
	}
	
	return fmt.Errorf("failed after %d retries: %w", maxRetries, err)
}

// GetMimeType returns MIME type for file extension
func GetMimeType(filename string) string {
	ext := strings.ToLower(filepath.Ext(filename))
	
	mimeTypes := map[string]string{
		".mp4":  "video/mp4",
		".webm": "video/webm",
		".gif":  "image/gif",
		".png":  "image/png",
		".jpg":  "image/jpeg",
		".jpeg": "image/jpeg",
		".json": "application/json",
		".yaml": "application/yaml",
		".yml":  "application/yaml",
		".txt":  "text/plain",
		".log":  "text/plain",
	}
	
	if mimeType, exists := mimeTypes[ext]; exists {
		return mimeType
	}
	
	return "application/octet-stream"
}

// ValidateSessionName validates session name format
func ValidateSessionName(name string) error {
	if name == "" {
		return fmt.Errorf("session name cannot be empty")
	}
	
	if len(name) > 50 {
		return fmt.Errorf("session name too long (max 50 characters)")
	}
	
	// Check for valid characters (alphanumeric, spaces, hyphens, underscores)
	for _, char := range name {
		if !((char >= 'a' && char <= 'z') ||
			(char >= 'A' && char <= 'Z') ||
			(char >= '0' && char <= '9') ||
			char == ' ' || char == '-' || char == '_') {
			return fmt.Errorf("session name contains invalid characters")
		}
	}
	
	return nil
}