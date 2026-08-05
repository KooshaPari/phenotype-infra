// SPDX-License-Identifier: MIT OR Apache-2.0
package tier

import (
	"context"
	"strings"
	"testing"
)

func TestStopFailsClosedWithoutOwnedProcessHandle(t *testing.T) {
	adapter := NewFirecrackerAdapter()

	err := adapter.Stop(context.Background(), "fc-untracked")
	if err == nil || !strings.Contains(err.Error(), "handle unavailable") {
		t.Fatalf("expected an untracked-handle refusal, got %v", err)
	}
}
