package tui

import (
	"fmt"
	"time"

	"github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/client"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/types"
)

// App represents the TUI application
type App struct {
	client   *client.Client
	program  *tea.Program
	width    int
	height   int
	state    AppState
	sessions []types.Session
	focused  FocusedComponent
	err      error
}

// AppState represents the current application state
type AppState int

const (
	StateLoading AppState = iota
	StateSessionList
	StateSessionDetail
	StateAutomation
	StateRecordings
	StateSettings
	StateError
)

// FocusedComponent represents which component has focus
type FocusedComponent int

const (
	FocusMain FocusedComponent = iota
	FocusSidebar
	FocusModal
)

// NewApp creates a new TUI application
func NewApp(apiURL, authToken string) (*App, error) {
	client := client.NewClient(apiURL, authToken)
	
	app := &App{
		client:  client,
		state:   StateLoading,
		focused: FocusMain,
	}

	return app, nil
}

// Run starts the TUI application
func (a *App) Run() error {
	p := tea.NewProgram(a, tea.WithAltScreen())
	a.program = p
	
	_, err := p.Run()
	return err
}

// Model interface implementation for Bubble Tea

// Init initializes the application
func (a *App) Init() tea.Cmd {
	return tea.Batch(
		a.loadSessions(),
		tea.Every(5*time.Second, func(t time.Time) tea.Msg {
			return tickMsg(t)
		}),
	)
}

// Update handles messages and updates the application state
func (a *App) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		a.width = msg.Width
		a.height = msg.Height
		return a, nil

	case tea.KeyMsg:
		return a.handleKeyMsg(msg)

	case sessionsLoadedMsg:
		a.sessions = msg.sessions
		a.state = StateSessionList
		a.err = nil
		return a, nil

	case errorMsg:
		a.err = msg.err
		a.state = StateError
		return a, nil

	case tickMsg:
		// Refresh data periodically
		return a, a.loadSessions()

	default:
		return a, nil
	}
}

// View renders the application UI
func (a *App) View() string {
	if a.width == 0 || a.height == 0 {
		return "Loading..."
	}

	switch a.state {
	case StateLoading:
		return a.renderLoading()
	case StateSessionList:
		return a.renderSessionList()
	case StateSessionDetail:
		return a.renderSessionDetail()
	case StateAutomation:
		return a.renderAutomation()
	case StateRecordings:
		return a.renderRecordings()
	case StateSettings:
		return a.renderSettings()
	case StateError:
		return a.renderError()
	default:
		return "Unknown state"
	}
}

// handleKeyMsg handles keyboard input
func (a *App) handleKeyMsg(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "ctrl+c", "q":
		return a, tea.Quit

	case "tab":
		// Switch focus between components
		if a.focused == FocusMain {
			a.focused = FocusSidebar
		} else {
			a.focused = FocusMain
		}
		return a, nil

	case "1":
		a.state = StateSessionList
		return a, a.loadSessions()

	case "2":
		a.state = StateAutomation
		return a, nil

	case "3":
		a.state = StateRecordings
		return a, nil

	case "4":
		a.state = StateSettings
		return a, nil

	case "r":
		// Refresh current view
		switch a.state {
		case StateSessionList:
			return a, a.loadSessions()
		default:
			return a, nil
		}

	case "enter":
		if a.state == StateSessionList && len(a.sessions) > 0 {
			a.state = StateSessionDetail
			return a, nil
		}
		return a, nil

	case "esc":
		if a.state == StateSessionDetail {
			a.state = StateSessionList
			return a, nil
		}
		return a, nil

	default:
		return a, nil
	}
}

// Styles
var (
	titleStyle = lipgloss.NewStyle().
			Bold(true).
			Foreground(lipgloss.Color("#FAFAFA")).
			Background(lipgloss.Color("#7D56F4")).
			Padding(0, 1)

	sidebarStyle = lipgloss.NewStyle().
			Border(lipgloss.RoundedBorder()).
			BorderForeground(lipgloss.Color("#874BFD")).
			Padding(1, 2).
			Width(25)

	mainStyle = lipgloss.NewStyle().
			Border(lipgloss.RoundedBorder()).
			BorderForeground(lipgloss.Color("#874BFD")).
			Padding(1, 2)

	activeStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#7D56F4")).
			Bold(true)

	inactiveStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#666666"))

	errorStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#FF0000")).
			Bold(true)

	statusStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#04B575")).
			Bold(true)
)

// renderLoading renders the loading screen
func (a *App) renderLoading() string {
	loading := "Loading KVirtualStage..."
	
	return lipgloss.Place(a.width, a.height,
		lipgloss.Center, lipgloss.Center,
		loading)
}

// renderSessionList renders the session list view
func (a *App) renderSessionList() string {
	title := titleStyle.Render("KVirtualStage TUI")
	
	sidebar := a.renderSidebar()
	main := a.renderSessionListMain()
	
	content := lipgloss.JoinHorizontal(
		lipgloss.Top,
		sidebar,
		main,
	)
	
	help := "Press 'q' to quit, 'r' to refresh, 'enter' to view details, '1-4' to switch views"
	
	return lipgloss.JoinVertical(
		lipgloss.Left,
		title,
		content,
		help,
	)
}

// renderSidebar renders the navigation sidebar
func (a *App) renderSidebar() string {
	var items []string
	
	menuItems := []struct {
		key   string
		label string
		state AppState
	}{
		{"1", "Sessions", StateSessionList},
		{"2", "Automation", StateAutomation},
		{"3", "Recordings", StateRecordings},
		{"4", "Settings", StateSettings},
	}
	
	for _, item := range menuItems {
		style := inactiveStyle
		if item.state == a.state {
			style = activeStyle
		}
		items = append(items, fmt.Sprintf("[%s] %s", item.key, style.Render(item.label)))
	}
	
	content := lipgloss.JoinVertical(lipgloss.Left, items...)
	
	return sidebarStyle.Copy().
		Height(a.height - 10).
		Render(content)
}

// renderSessionListMain renders the main session list content
func (a *App) renderSessionListMain() string {
	if len(a.sessions) == 0 {
		return mainStyle.Copy().
			Width(a.width - 30).
			Height(a.height - 10).
			Render("No sessions found")
	}
	
	var rows []string
	rows = append(rows, "ID\t\tName\t\tStatus\t\tDesktop")
	rows = append(rows, "----\t\t----\t\t------\t\t-------")
	
	for _, session := range a.sessions {
		statusText := string(session.Status)
		switch session.Status {
		case types.SessionStatusRunning:
			statusText = statusStyle.Render(statusText)
		case types.SessionStatusError:
			statusText = errorStyle.Render(statusText)
		}
		
		row := fmt.Sprintf("%s\t%s\t%s\t%s",
			session.ID[:8]+"...",
			session.Name,
			statusText,
			session.Config.DesktopEnvironment.Type,
		)
		rows = append(rows, row)
	}
	
	content := lipgloss.JoinVertical(lipgloss.Left, rows...)
	
	return mainStyle.Copy().
		Width(a.width - 30).
		Height(a.height - 10).
		Render(content)
}

// renderSessionDetail renders the session detail view
func (a *App) renderSessionDetail() string {
	title := titleStyle.Render("Session Details")
	
	if len(a.sessions) == 0 {
		return "No session selected"
	}
	
	session := a.sessions[0] // For now, show first session
	
	details := []string{
		fmt.Sprintf("ID: %s", session.ID),
		fmt.Sprintf("Name: %s", session.Name),
		fmt.Sprintf("Status: %s", session.Status),
		fmt.Sprintf("Desktop: %s", session.Config.DesktopEnvironment.Type),
		fmt.Sprintf("Memory: %d MB", session.Config.Resources.MemoryMB),
		fmt.Sprintf("CPU: %.1f cores", session.Config.Resources.CPUCores),
		fmt.Sprintf("Created: %s", session.CreatedAt.Format(time.RFC3339)),
	}
	
	content := lipgloss.JoinVertical(lipgloss.Left, details...)
	
	help := "Press 'esc' to go back"
	
	return lipgloss.JoinVertical(
		lipgloss.Left,
		title,
		content,
		help,
	)
}

// renderAutomation renders the automation view
func (a *App) renderAutomation() string {
	title := titleStyle.Render("Automation")
	content := "Automation management coming soon..."
	
	return lipgloss.JoinVertical(lipgloss.Left, title, content)
}

// renderRecordings renders the recordings view
func (a *App) renderRecordings() string {
	title := titleStyle.Render("Recordings")
	content := "Recording management coming soon..."
	
	return lipgloss.JoinVertical(lipgloss.Left, title, content)
}

// renderSettings renders the settings view
func (a *App) renderSettings() string {
	title := titleStyle.Render("Settings")
	content := "Settings management coming soon..."
	
	return lipgloss.JoinVertical(lipgloss.Left, title, content)
}

// renderError renders the error view
func (a *App) renderError() string {
	title := titleStyle.Render("Error")
	errorText := errorStyle.Render(fmt.Sprintf("Error: %v", a.err))
	help := "Press 'r' to retry or 'q' to quit"
	
	return lipgloss.JoinVertical(lipgloss.Left, title, errorText, help)
}

// Messages for Bubble Tea

type sessionsLoadedMsg struct {
	sessions []types.Session
}

type errorMsg struct {
	err error
}

type tickMsg time.Time

// Commands

func (a *App) loadSessions() tea.Cmd {
	return func() tea.Msg {
		sessions, err := a.client.ListSessions()
		if err != nil {
			return errorMsg{err}
		}
		return sessionsLoadedMsg{sessions}
	}
}