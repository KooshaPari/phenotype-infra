package cli

import (
	"encoding/json"
	"fmt"

	"github.com/spf13/cobra"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/client"
)

var systemCmd = &cobra.Command{
	Use:   "system",
	Short: "System management commands",
	Long:  "Get system information, status, and perform maintenance tasks",
}

var systemInfoCmd = &cobra.Command{
	Use:   "info",
	Short: "Get system information",
	Long:  "Display system information and version details",
	RunE:  systemInfo,
}

var systemStatusCmd = &cobra.Command{
	Use:   "status",
	Short: "Get system status",
	Long:  "Display current system status and health",
	RunE:  systemStatus,
}

func init() {
	// Add subcommands
	systemCmd.AddCommand(systemInfoCmd)
	systemCmd.AddCommand(systemStatusCmd)
}

func systemInfo(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)

	info, err := client.GetSystemInfo()
	if err != nil {
		return fmt.Errorf("failed to get system info: %w", err)
	}

	switch outputFormat {
	case "json":
		data, _ := json.MarshalIndent(info, "", "  ")
		fmt.Println(string(data))
	case "yaml":
		// TODO: Implement YAML output
		fallthrough
	default:
		fmt.Printf("System Information:\n")
		for key, value := range info {
			fmt.Printf("%s: %v\n", key, value)
		}
	}

	return nil
}

func systemStatus(cmd *cobra.Command, args []string) error {
	// TODO: Implement system status retrieval
	fmt.Println("System status not yet implemented")
	return nil
}