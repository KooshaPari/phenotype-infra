package cli

import (
	"encoding/json"
	"fmt"
	"os"
	"text/tabwriter"
	"time"

	"github.com/spf13/cobra"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/client"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/types"
)

var sessionCmd = &cobra.Command{
	Use:   "session",
	Short: "Manage virtual desktop sessions",
	Long:  "Create, manage, and monitor virtual desktop sessions",
}

var sessionCreateCmd = &cobra.Command{
	Use:   "create",
	Short: "Create a new virtual desktop session",
	Long: `Create a new virtual desktop session with specified configuration.

Examples:
  kvs session create --name "demo" --desktop ubuntu-xfce
  kvs session create --name "test" --desktop kubuntu-kde --memory 2048 --cpu 2.0
  kvs session create --name "windows" --desktop windows-10 --memory 4096`,
	RunE: sessionCreate,
}

var sessionListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all sessions",
	Long:  "List all virtual desktop sessions with their current status",
	RunE:  sessionList,
}

var sessionGetCmd = &cobra.Command{
	Use:   "get [session-id]",
	Short: "Get session details",
	Long:  "Get detailed information about a specific session",
	Args:  cobra.ExactArgs(1),
	RunE:  sessionGet,
}

var sessionStartCmd = &cobra.Command{
	Use:   "start [session-id]",
	Short: "Start a session",
	Long:  "Start a stopped or paused session",
	Args:  cobra.ExactArgs(1),
	RunE:  sessionStart,
}

var sessionStopCmd = &cobra.Command{
	Use:   "stop [session-id]",
	Short: "Stop a session",
	Long:  "Stop a running session",
	Args:  cobra.ExactArgs(1),
	RunE:  sessionStop,
}

var sessionDeleteCmd = &cobra.Command{
	Use:   "delete [session-id]",
	Short: "Delete a session",
	Long:  "Delete a session and all its associated data",
	Args:  cobra.ExactArgs(1),
	RunE:  sessionDelete,
}

var sessionScreenshotCmd = &cobra.Command{
	Use:   "screenshot [session-id]",
	Short: "Take a screenshot",
	Long:  "Take a screenshot of the session's desktop",
	Args:  cobra.ExactArgs(1),
	RunE:  sessionScreenshot,
}

var sessionVNCCmd = &cobra.Command{
	Use:   "vnc [session-id]",
	Short: "Get VNC connection info",
	Long:  "Get VNC connection information for a session",
	Args:  cobra.ExactArgs(1),
	RunE:  sessionVNC,
}

// Session creation flags
var (
	sessionName     string
	desktopType     string
	memoryMB        int64
	cpuCores        float64
	diskSpaceGB     int64
	displayWidth    int
	displayHeight   int
	vncEnabled      bool
	audioEnabled    bool
	internetAccess  bool
	maxDuration     time.Duration
	autoDestroy     bool
	screenshotPath  string
)

func init() {
	// Add subcommands
	sessionCmd.AddCommand(sessionCreateCmd)
	sessionCmd.AddCommand(sessionListCmd)
	sessionCmd.AddCommand(sessionGetCmd)
	sessionCmd.AddCommand(sessionStartCmd)
	sessionCmd.AddCommand(sessionStopCmd)
	sessionCmd.AddCommand(sessionDeleteCmd)
	sessionCmd.AddCommand(sessionScreenshotCmd)
	sessionCmd.AddCommand(sessionVNCCmd)

	// Create command flags
	sessionCreateCmd.Flags().StringVarP(&sessionName, "name", "n", "", "Session name (required)")
	sessionCreateCmd.Flags().StringVarP(&desktopType, "desktop", "d", "ubuntu-xfce", "Desktop environment (ubuntu-xfce, ubuntu-gnome, kubuntu-kde, windows-10)")
	sessionCreateCmd.Flags().Int64VarP(&memoryMB, "memory", "m", 1024, "Memory limit in MB")
	sessionCreateCmd.Flags().Float64VarP(&cpuCores, "cpu", "c", 1.0, "CPU cores limit")
	sessionCreateCmd.Flags().Int64Var(&diskSpaceGB, "disk", 10, "Disk space limit in GB")
	sessionCreateCmd.Flags().IntVar(&displayWidth, "width", 1920, "Display width")
	sessionCreateCmd.Flags().IntVar(&displayHeight, "height", 1080, "Display height")
	sessionCreateCmd.Flags().BoolVar(&vncEnabled, "vnc", true, "Enable VNC access")
	sessionCreateCmd.Flags().BoolVar(&audioEnabled, "audio", false, "Enable audio")
	sessionCreateCmd.Flags().BoolVar(&internetAccess, "internet", true, "Enable internet access")
	sessionCreateCmd.Flags().DurationVar(&maxDuration, "max-duration", 8*time.Hour, "Maximum session duration")
	sessionCreateCmd.Flags().BoolVar(&autoDestroy, "auto-destroy", true, "Auto-destroy session when idle")
	sessionCreateCmd.MarkFlagRequired("name")

	// Screenshot command flags
	sessionScreenshotCmd.Flags().StringVarP(&screenshotPath, "output", "o", "", "Output file path (default: session-id-timestamp.png)")
}

func sessionCreate(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)

	config := types.SessionConfig{
		DesktopEnvironment: types.DesktopEnvironment{
			Type: desktopType,
		},
		Resources: types.ResourceLimits{
			CPUCores:    cpuCores,
			MemoryMB:    memoryMB,
			DiskSpaceGB: diskSpaceGB,
		},
		Display: types.DisplayConfig{
			Width:      displayWidth,
			Height:     displayHeight,
			VNCEnabled: vncEnabled,
		},
		Audio: types.AudioConfig{
			Enabled: audioEnabled,
		},
		Network: types.NetworkConfig{
			InternetAccess: internetAccess,
		},
		AutoDestroy: autoDestroy,
		MaxDuration: maxDuration,
	}

	session, err := client.CreateSession(sessionName, config)
	if err != nil {
		return fmt.Errorf("failed to create session: %w", err)
	}

	fmt.Printf("Session created successfully:\n")
	printSession(session, outputFormat)
	return nil
}

func sessionList(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)

	sessions, err := client.ListSessions()
	if err != nil {
		return fmt.Errorf("failed to list sessions: %w", err)
	}

	if len(sessions) == 0 {
		fmt.Println("No sessions found")
		return nil
	}

	printSessions(sessions, outputFormat)
	return nil
}

func sessionGet(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)
	sessionID := args[0]

	session, err := client.GetSession(sessionID)
	if err != nil {
		return fmt.Errorf("failed to get session: %w", err)
	}

	printSession(session, outputFormat)
	return nil
}

func sessionStart(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)
	sessionID := args[0]

	err := client.StartSession(sessionID)
	if err != nil {
		return fmt.Errorf("failed to start session: %w", err)
	}

	fmt.Printf("Session %s started successfully\n", sessionID)
	return nil
}

func sessionStop(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)
	sessionID := args[0]

	err := client.StopSession(sessionID)
	if err != nil {
		return fmt.Errorf("failed to stop session: %w", err)
	}

	fmt.Printf("Session %s stopped successfully\n", sessionID)
	return nil
}

func sessionDelete(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)
	sessionID := args[0]

	err := client.DeleteSession(sessionID)
	if err != nil {
		return fmt.Errorf("failed to delete session: %w", err)
	}

	fmt.Printf("Session %s deleted successfully\n", sessionID)
	return nil
}

func sessionScreenshot(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)
	sessionID := args[0]

	data, err := client.TakeScreenshot(sessionID)
	if err != nil {
		return fmt.Errorf("failed to take screenshot: %w", err)
	}

	filename := screenshotPath
	if filename == "" {
		filename = fmt.Sprintf("%s-%d.png", sessionID, time.Now().Unix())
	}

	err = os.WriteFile(filename, data, 0644)
	if err != nil {
		return fmt.Errorf("failed to save screenshot: %w", err)
	}

	fmt.Printf("Screenshot saved to: %s\n", filename)
	return nil
}

func sessionVNC(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)
	sessionID := args[0]

	vncInfo, err := client.GetVNCInfo(sessionID)
	if err != nil {
		return fmt.Errorf("failed to get VNC info: %w", err)
	}

	switch outputFormat {
	case "json":
		data, _ := json.MarshalIndent(vncInfo, "", "  ")
		fmt.Println(string(data))
	case "yaml":
		// TODO: Implement YAML output
		fmt.Printf("Host: %s\nPort: %d\nPassword: %s\n", 
			vncInfo["host"], vncInfo["port"], vncInfo["password"])
	default:
		fmt.Printf("VNC Connection Information:\n")
		fmt.Printf("Host: %s\n", vncInfo["host"])
		fmt.Printf("Port: %d\n", vncInfo["port"])
		fmt.Printf("Password: %s\n", vncInfo["password"])
		fmt.Printf("URL: vnc://%s:%d\n", vncInfo["host"], vncInfo["port"])
	}

	return nil
}

func printSession(session *types.Session, format string) {
	switch format {
	case "json":
		data, _ := json.MarshalIndent(session, "", "  ")
		fmt.Println(string(data))
	case "yaml":
		// TODO: Implement YAML output
		fallthrough
	default:
		fmt.Printf("ID: %s\n", session.ID)
		fmt.Printf("Name: %s\n", session.Name)
		fmt.Printf("Status: %s\n", session.Status)
		fmt.Printf("Desktop: %s\n", session.Config.DesktopEnvironment.Type)
		fmt.Printf("Memory: %d MB\n", session.Config.Resources.MemoryMB)
		fmt.Printf("CPU: %.1f cores\n", session.Config.Resources.CPUCores)
		fmt.Printf("Created: %s\n", session.CreatedAt.Format(time.RFC3339))
		if session.ExpiresAt != nil {
			fmt.Printf("Expires: %s\n", session.ExpiresAt.Format(time.RFC3339))
		}
	}
}

func printSessions(sessions []types.Session, format string) {
	switch format {
	case "json":
		data, _ := json.MarshalIndent(sessions, "", "  ")
		fmt.Println(string(data))
	case "yaml":
		// TODO: Implement YAML output
		fallthrough
	default:
		w := tabwriter.NewWriter(os.Stdout, 0, 0, 2, ' ', 0)
		fmt.Fprintf(w, "ID\tNAME\tSTATUS\tDESKTOP\tMEMORY\tCPU\tCREATED\n")
		for _, session := range sessions {
			fmt.Fprintf(w, "%s\t%s\t%s\t%s\t%dMB\t%.1f\t%s\n",
				session.ID,
				session.Name,
				session.Status,
				session.Config.DesktopEnvironment.Type,
				session.Config.Resources.MemoryMB,
				session.Config.Resources.CPUCores,
				session.CreatedAt.Format("2006-01-02 15:04:05"),
			)
		}
		w.Flush()
	}
}