package cli

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/kvirtualstage/kvirtualstage-go/internal/tui"
)

var tuiCmd = &cobra.Command{
	Use:   "tui",
	Short: "Launch interactive terminal UI",
	Long: `Launch the interactive terminal user interface for managing
KVirtualStage sessions, automation, and recordings.

The TUI provides a visual interface for:
- Managing virtual desktop sessions
- Running automation workflows  
- Monitoring recordings
- Viewing system metrics
- Real-time session monitoring`,
	RunE: launchTUI,
}

func launchTUI(cmd *cobra.Command, args []string) error {
	app, err := tui.NewApp(apiURL, authToken)
	if err != nil {
		return fmt.Errorf("failed to create TUI app: %w", err)
	}

	return app.Run()
}