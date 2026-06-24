package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/kvirtualstage/kvirtualstage-go/internal/tui"
)

var (
	apiURL    = flag.String("api-url", "http://localhost:8080", "KVirtualStage API URL")
	authToken = flag.String("token", "", "Authentication token")
)

func main() {
	flag.Parse()

	app, err := tui.NewApp(*apiURL, *authToken)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error creating TUI app: %v\n", err)
		os.Exit(1)
	}

	if err := app.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "Error running TUI app: %v\n", err)
		os.Exit(1)
	}
}