// SPDX-License-Identifier: MIT OR Apache-2.0
// Package phenointegration wires request-scoped identifiers and logging
// into nanovms HTTP handlers.
//
// The ctxkit dependency (github.com/kooshapari/pheno-go-ctxkit) has been
// inlined here to remove the external dependency. The extracted middleware
// injects an X-Request-ID header into every response and propagates it
// through the request context.
package phenointegration

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"log"
	"net/http"
)

type contextKey string

const requestIDKey contextKey = "request-id"

// RequestIDFromContext extracts the request ID from a context.
func RequestIDFromContext(ctx context.Context) string {
	if id, ok := ctx.Value(requestIDKey).(string); ok {
		return id
	}
	return ""
}

// Middleware injects an X-Request-ID header into responses and propagates
// it through the request context.
func Middleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		id := r.Header.Get("X-Request-ID")
		if id == "" {
			id = generateRequestID()
		}
		w.Header().Set("X-Request-ID", id)
		ctx := context.WithValue(r.Context(), requestIDKey, id)
		next.ServeHTTP(w, ctx)
	})
}

func generateRequestID() string {
	b := make([]byte, 16)
	if _, err := rand.Read(b); err != nil {
		log.Printf("warning: failed to generate request ID: %v", err)
		return "unknown"
	}
	return hex.EncodeToString(b)
}

// InitServer returns an http.Handler with the inlined request-id middleware
// applied. The returned handler registers /healthz and can be passed
// directly to http.Server or wrapped by additional middleware.
func InitServer(ctx context.Context) http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("/healthz", HandleHealthz)
	return Middleware(mux)
}

// HandleHealthz is a simple liveness probe that returns HTTP 200.
func HandleHealthz(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
}
