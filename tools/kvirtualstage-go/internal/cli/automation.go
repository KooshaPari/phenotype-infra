package cli

import (
	"encoding/json"
	"fmt"
	"os"
	"text/tabwriter"

	"github.com/spf13/cobra"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/client"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/types"
)

var automationCmd = &cobra.Command{
	Use:   "automation",
	Short: "Manage automation workflows",
	Long:  "Create, manage, and execute automation workflows and scripts",
}

var automationScriptsCmd = &cobra.Command{
	Use:   "scripts",
	Short: "Manage automation scripts",
	Long:  "Create, list, and manage automation scripts",
}

var automationExecutionsCmd = &cobra.Command{
	Use:   "executions",
	Short: "Manage automation executions",
	Long:  "Execute scripts and monitor execution results",
}

var automationScriptsListCmd = &cobra.Command{
	Use:   "list",
	Short: "List automation scripts",
	Long:  "List all available automation scripts",
	RunE:  automationScriptsList,
}

var automationExecuteCmd = &cobra.Command{
	Use:   "execute",
	Short: "Execute an automation script",
	Long:  "Execute an automation script on a session",
	RunE:  automationExecute,
}

var automationExecutionsListCmd = &cobra.Command{
	Use:   "list",
	Short: "List automation executions",
	Long:  "List all automation executions and their status",
	RunE:  automationExecutionsList,
}

var (
	scriptID    string
	sessionID   string
	executionID string
)

func init() {
	// Add subcommands
	automationCmd.AddCommand(automationScriptsCmd)
	automationCmd.AddCommand(automationExecutionsCmd)
	
	automationScriptsCmd.AddCommand(automationScriptsListCmd)
	
	automationExecutionsCmd.AddCommand(automationExecutionsListCmd)
	automationExecutionsCmd.AddCommand(automationExecuteCmd)

	// Execute command flags
	automationExecuteCmd.Flags().StringVarP(&scriptID, "script", "s", "", "Script ID to execute (required)")
	automationExecuteCmd.Flags().StringVar(&sessionID, "session", "", "Session ID to execute on (required)")
	automationExecuteCmd.MarkFlagRequired("script")
	automationExecuteCmd.MarkFlagRequired("session")
}

func automationScriptsList(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)

	scripts, err := client.ListAutomationScripts()
	if err != nil {
		return fmt.Errorf("failed to list scripts: %w", err)
	}

	if len(scripts) == 0 {
		fmt.Println("No automation scripts found")
		return nil
	}

	printAutomationScripts(scripts, outputFormat)
	return nil
}

func automationExecute(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)

	result, err := client.ExecuteAutomation(sessionID, scriptID)
	if err != nil {
		return fmt.Errorf("failed to execute automation: %w", err)
	}

	fmt.Printf("Automation execution started:\n")
	printAutomationResult(result, outputFormat)
	return nil
}

func automationExecutionsList(cmd *cobra.Command, args []string) error {
	// TODO: Implement executions listing
	fmt.Println("Automation executions listing not yet implemented")
	return nil
}

func printAutomationScripts(scripts []types.AutomationScript, format string) {
	switch format {
	case "json":
		data, _ := json.MarshalIndent(scripts, "", "  ")
		fmt.Println(string(data))
	case "yaml":
		// TODO: Implement YAML output
		fallthrough
	default:
		w := tabwriter.NewWriter(os.Stdout, 0, 0, 2, ' ', 0)
		fmt.Fprintf(w, "ID\tNAME\tDESCRIPTION\tSTEPS\tCREATED\n")
		for _, script := range scripts {
			fmt.Fprintf(w, "%s\t%s\t%s\t%d\t%s\n",
				script.ID,
				script.Name,
				script.Description,
				len(script.Steps),
				script.CreatedAt.Format("2006-01-02 15:04:05"),
			)
		}
		w.Flush()
	}
}

func printAutomationResult(result *types.AutomationResult, format string) {
	switch format {
	case "json":
		data, _ := json.MarshalIndent(result, "", "  ")
		fmt.Println(string(data))
	case "yaml":
		// TODO: Implement YAML output
		fallthrough
	default:
		fmt.Printf("ID: %s\n", result.ID)
		fmt.Printf("Session ID: %s\n", result.SessionID)
		fmt.Printf("Script ID: %s\n", result.ScriptID)
		fmt.Printf("Status: %s\n", result.Status)
		fmt.Printf("Started: %s\n", result.StartedAt.Format("2006-01-02 15:04:05"))
		if result.CompletedAt != nil {
			fmt.Printf("Completed: %s\n", result.CompletedAt.Format("2006-01-02 15:04:05"))
		}
		if result.Error != "" {
			fmt.Printf("Error: %s\n", result.Error)
		}
	}
}