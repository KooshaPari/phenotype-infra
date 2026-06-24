package cli

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var (
	cfgFile    string
	apiURL     string
	authToken  string
	outputFormat string
	verbose    bool
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "kvs",
	Short: "KVirtualStage CLI - Manage virtual desktop sessions and automation",
	Long: `KVirtualStage CLI is a command-line interface for managing virtual desktop
sessions, automation workflows, and recordings.

Examples:
  kvs session create --name "demo" --desktop ubuntu-xfce
  kvs automation run --script demo.json --session demo-session
  kvs recording start --session demo-session --format mp4`,
	Version: "1.0.0",
}

// Execute adds all child commands to the root command and sets flags appropriately.
func Execute() error {
	return rootCmd.Execute()
}

func init() {
	cobra.OnInitialize(initConfig)

	// Global flags
	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.kvs.yaml)")
	rootCmd.PersistentFlags().StringVar(&apiURL, "api-url", "http://localhost:8080", "KVirtualStage API URL")
	rootCmd.PersistentFlags().StringVar(&authToken, "token", "", "Authentication token")
	rootCmd.PersistentFlags().StringVarP(&outputFormat, "output", "o", "table", "Output format (table, json, yaml)")
	rootCmd.PersistentFlags().BoolVarP(&verbose, "verbose", "v", false, "Enable verbose output")

	// Bind flags to viper
	viper.BindPFlag("api_url", rootCmd.PersistentFlags().Lookup("api-url"))
	viper.BindPFlag("auth_token", rootCmd.PersistentFlags().Lookup("token"))
	viper.BindPFlag("output_format", rootCmd.PersistentFlags().Lookup("output"))
	viper.BindPFlag("verbose", rootCmd.PersistentFlags().Lookup("verbose"))

	// Add subcommands
	rootCmd.AddCommand(sessionCmd)
	rootCmd.AddCommand(automationCmd)
	rootCmd.AddCommand(recordingCmd)
	rootCmd.AddCommand(authCmd)
	rootCmd.AddCommand(systemCmd)
	rootCmd.AddCommand(tuiCmd)
	rootCmd.AddCommand(completionCmd)
}

// initConfig reads in config file and ENV variables if set
func initConfig() {
	if cfgFile != "" {
		// Use config file from the flag
		viper.SetConfigFile(cfgFile)
	} else {
		// Find home directory
		home, err := os.UserHomeDir()
		cobra.CheckErr(err)

		// Search config in home directory with name ".kvs" (without extension)
		viper.AddConfigPath(home)
		viper.SetConfigType("yaml")
		viper.SetConfigName(".kvs")
	}

	// Environment variables
	viper.SetEnvPrefix("KVS")
	viper.AutomaticEnv()

	// Read config file
	if err := viper.ReadInConfig(); err == nil && verbose {
		fmt.Fprintln(os.Stderr, "Using config file:", viper.ConfigFileUsed())
	}
}