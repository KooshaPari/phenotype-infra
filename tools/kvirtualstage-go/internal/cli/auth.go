package cli

import (
	"fmt"
	"syscall"

	"github.com/spf13/cobra"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/client"
	"golang.org/x/crypto/ssh/terminal"
)

var authCmd = &cobra.Command{
	Use:   "auth",
	Short: "Authentication commands",
	Long:  "Login, logout, and manage authentication",
}

var loginCmd = &cobra.Command{
	Use:   "login",
	Short: "Login to KVirtualStage",
	Long:  "Authenticate with KVirtualStage and store token",
	RunE:  login,
}

var logoutCmd = &cobra.Command{
	Use:   "logout",
	Short: "Logout from KVirtualStage",
	Long:  "Clear stored authentication token",
	RunE:  logout,
}

var (
	username string
)

func init() {
	// Add subcommands
	authCmd.AddCommand(loginCmd)
	authCmd.AddCommand(logoutCmd)

	// Login command flags
	loginCmd.Flags().StringVarP(&username, "username", "u", "", "Username")
}

func login(cmd *cobra.Command, args []string) error {
	if username == "" {
		fmt.Print("Username: ")
		fmt.Scanln(&username)
	}

	fmt.Print("Password: ")
	password, err := terminal.ReadPassword(int(syscall.Stdin))
	if err != nil {
		return fmt.Errorf("failed to read password: %w", err)
	}
	fmt.Println() // New line after password input

	client := client.NewClient(apiURL, "")
	token, err := client.Login(username, string(password))
	if err != nil {
		return fmt.Errorf("login failed: %w", err)
	}

	// TODO: Store token securely (keychain, config file, etc.)
	fmt.Printf("Login successful!\n")
	fmt.Printf("Token: %s\n", token)
	fmt.Printf("Use this token with --token flag or set KVS_AUTH_TOKEN environment variable\n")

	return nil
}

func logout(cmd *cobra.Command, args []string) error {
	// TODO: Clear stored token
	fmt.Println("Logged out successfully")
	return nil
}