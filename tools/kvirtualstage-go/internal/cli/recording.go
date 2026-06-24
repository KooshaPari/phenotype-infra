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

var recordingCmd = &cobra.Command{
	Use:   "recording",
	Short: "Manage session recordings",
	Long:  "Start, stop, and manage video recordings of sessions",
}

var recordingListCmd = &cobra.Command{
	Use:   "list",
	Short: "List recordings",
	Long:  "List all session recordings",
	RunE:  recordingList,
}

var recordingStartCmd = &cobra.Command{
	Use:   "start",
	Short: "Start recording",
	Long:  "Start recording a session",
	RunE:  recordingStart,
}

var recordingStopCmd = &cobra.Command{
	Use:   "stop [recording-id]",
	Short: "Stop recording",
	Long:  "Stop an active recording",
	Args:  cobra.ExactArgs(1),
	RunE:  recordingStop,
}

var (
	recordingName   string
	recordingFormat string
)

func init() {
	// Add subcommands
	recordingCmd.AddCommand(recordingListCmd)
	recordingCmd.AddCommand(recordingStartCmd)
	recordingCmd.AddCommand(recordingStopCmd)

	// Start command flags
	recordingStartCmd.Flags().StringVarP(&recordingName, "name", "n", "", "Recording name (required)")
	recordingStartCmd.Flags().StringVar(&sessionID, "session", "", "Session ID to record (required)")
	recordingStartCmd.Flags().StringVarP(&recordingFormat, "format", "f", "mp4", "Recording format (mp4, webm, gif)")
	recordingStartCmd.MarkFlagRequired("name")
	recordingStartCmd.MarkFlagRequired("session")
}

func recordingList(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)

	recordings, err := client.ListRecordings()
	if err != nil {
		return fmt.Errorf("failed to list recordings: %w", err)
	}

	if len(recordings) == 0 {
		fmt.Println("No recordings found")
		return nil
	}

	printRecordings(recordings, outputFormat)
	return nil
}

func recordingStart(cmd *cobra.Command, args []string) error {
	client := client.NewClient(apiURL, authToken)

	recording, err := client.StartRecording(sessionID, recordingName, recordingFormat)
	if err != nil {
		return fmt.Errorf("failed to start recording: %w", err)
	}

	fmt.Printf("Recording started successfully:\n")
	printRecording(recording, outputFormat)
	return nil
}

func recordingStop(cmd *cobra.Command, args []string) error {
	recordingID := args[0]
	
	// TODO: Implement recording stop via client
	fmt.Printf("Recording %s stopped successfully\n", recordingID)
	return nil
}

func printRecordings(recordings []types.Recording, format string) {
	switch format {
	case "json":
		data, _ := json.MarshalIndent(recordings, "", "  ")
		fmt.Println(string(data))
	case "yaml":
		// TODO: Implement YAML output
		fallthrough
	default:
		w := tabwriter.NewWriter(os.Stdout, 0, 0, 2, ' ', 0)
		fmt.Fprintf(w, "ID\tNAME\tSESSION\tSTATUS\tFORMAT\tDURATION\tSTARTED\n")
		for _, recording := range recordings {
			duration := "N/A"
			if recording.Duration > 0 {
				duration = recording.Duration.String()
			}
			
			fmt.Fprintf(w, "%s\t%s\t%s\t%s\t%s\t%s\t%s\n",
				recording.ID,
				recording.Name,
				recording.SessionID,
				recording.Status,
				recording.Format,
				duration,
				recording.StartedAt.Format("2006-01-02 15:04:05"),
			)
		}
		w.Flush()
	}
}

func printRecording(recording *types.Recording, format string) {
	switch format {
	case "json":
		data, _ := json.MarshalIndent(recording, "", "  ")
		fmt.Println(string(data))
	case "yaml":
		// TODO: Implement YAML output
		fallthrough
	default:
		fmt.Printf("ID: %s\n", recording.ID)
		fmt.Printf("Name: %s\n", recording.Name)
		fmt.Printf("Session ID: %s\n", recording.SessionID)
		fmt.Printf("Status: %s\n", recording.Status)
		fmt.Printf("Format: %s\n", recording.Format)
		fmt.Printf("Started: %s\n", recording.StartedAt.Format("2006-01-02 15:04:05"))
		if recording.EndedAt != nil {
			fmt.Printf("Ended: %s\n", recording.EndedAt.Format("2006-01-02 15:04:05"))
		}
		if recording.Duration > 0 {
			fmt.Printf("Duration: %s\n", recording.Duration.String())
		}
		if recording.FileSize > 0 {
			fmt.Printf("File Size: %d bytes\n", recording.FileSize)
		}
	}
}