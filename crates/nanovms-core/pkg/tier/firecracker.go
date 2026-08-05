// SPDX-License-Identifier: MIT OR Apache-2.0
// Package tier provides public tier adapters for NVMS isolation levels.
package tier

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"sync"
	"time"

	"github.com/kooshapari/nanovms/internal/domain"
)

// FirecrackerAdapter is the Tier3 Firecracker microVM adapter for untrusted workloads.
// Startup: ~125ms, Memory: ~128MB, CPU overhead: ~10%
type FirecrackerAdapter struct {
	path      string
	apiSocket string
	mu        sync.Mutex
	processes map[string]*firecrackerProcess
}

type firecrackerProcess struct {
	process *os.Process
	done    <-chan error
}

// NewFirecrackerAdapter creates a new Tier3 Firecracker adapter.
func NewFirecrackerAdapter() *FirecrackerAdapter {
	return &FirecrackerAdapter{
		apiSocket: "/tmp/firecracker-api.sock",
		processes: make(map[string]*firecrackerProcess),
	}
}

// Deploy deploys a Firecracker microVM workload.
func (a *FirecrackerAdapter) Deploy(ctx context.Context, config domain.SandboxConfig) (*domain.Sandbox, error) {
	path, err := exec.LookPath("firecracker")
	if err != nil {
		return nil, fmt.Errorf("firecracker binary not found: %w", err)
	}
	a.path = path

	id := fmt.Sprintf("fc-%s", domain.GenerateID())
	sandbox := &domain.Sandbox{
		ID:        id,
		Name:      config.Name,
		Status:    domain.SandboxStatusRunning,
		Type:      domain.SandboxTypeVM,
		VMFlavor:  domain.VMFlavorMicroVM,
		Config:    &config,
		CreatedAt: time.Now(),
	}
	return sandbox, nil
}

// Start starts the Firecracker microVM.
func (a *FirecrackerAdapter) Start(ctx context.Context, id string) error {
	if a.path == "" {
		return fmt.Errorf("firecracker path not set")
	}
	a.mu.Lock()
	if a.processes == nil {
		a.processes = make(map[string]*firecrackerProcess)
	}
	if _, exists := a.processes[id]; exists {
		a.mu.Unlock()
		return fmt.Errorf("firecracker process already tracked for id %s", id)
	}
	// Firecracker starts via API socket after the binary is launched
	cmd := exec.CommandContext(ctx, a.path, "--api-sock", a.apiSocket, "--id", id)
	if err := cmd.Start(); err != nil {
		a.mu.Unlock()
		return err
	}
	done := make(chan error, 1)
	go func() {
		done <- cmd.Wait()
		close(done)
	}()
	a.processes[id] = &firecrackerProcess{process: cmd.Process, done: done}
	a.mu.Unlock()
	return nil
}

// Stop stops the Firecracker microVM.
func (a *FirecrackerAdapter) Stop(ctx context.Context, id string) error {
	a.mu.Lock()
	managed, ok := a.processes[id]
	a.mu.Unlock()
	if !ok {
		return fmt.Errorf("firecracker process handle unavailable for id %s; refusing unscoped termination", id)
	}

	// The process handle was created by this adapter; never search the host by
	// executable or command-line pattern.  A process that already exited is
	// considered stopped and is removed from the registry.
	select {
	case <-managed.done:
		a.forgetProcess(id, managed)
		return nil
	default:
	}
	if err := managed.process.Signal(os.Interrupt); err != nil {
		select {
		case <-managed.done:
			a.forgetProcess(id, managed)
			return nil
		default:
		}
		return fmt.Errorf("failed to signal tracked firecracker process %s: %w", id, err)
	}

	select {
	case <-managed.done:
		a.forgetProcess(id, managed)
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}

func (a *FirecrackerAdapter) forgetProcess(id string, managed *firecrackerProcess) {
	a.mu.Lock()
	if a.processes[id] == managed {
		delete(a.processes, id)
	}
	a.mu.Unlock()
}

// Delete deletes the Firecracker microVM.
func (a *FirecrackerAdapter) Delete(ctx context.Context, id string) error {
	// Clean up socket and VM state
	if err := a.Stop(ctx, id); err != nil {
		return fmt.Errorf("failed to stop vm %s: %w", id, err)
	}
	return nil
}

// GetStartupTime returns the typical startup time for a Firecracker microVM.
func (a *FirecrackerAdapter) GetStartupTime() time.Duration {
	return 125 * time.Millisecond
}
