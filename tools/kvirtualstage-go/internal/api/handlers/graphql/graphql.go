package graphql

import (
	"context"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/graphql-go/graphql"
	"github.com/kvirtualstage/kvirtualstage-go/internal/config"
	"github.com/kvirtualstage/kvirtualstage-go/pkg/types"
	"github.com/sirupsen/logrus"
)

// Handler handles GraphQL requests
type Handler struct {
	config *config.Config
	logger *logrus.Logger
	schema graphql.Schema
}

// NewHandler creates a new GraphQL handler
func NewHandler(cfg *config.Config, logger *logrus.Logger) *Handler {
	h := &Handler{
		config: cfg,
		logger: logger,
	}

	// Build GraphQL schema
	schema, err := h.buildSchema()
	if err != nil {
		logger.Fatalf("Failed to build GraphQL schema: %v", err)
	}

	h.schema = schema
	return h
}

// Handle handles GraphQL requests
func (h *Handler) Handle(c *gin.Context) {
	if !h.config.Server.GraphQL.Enabled {
		c.JSON(http.StatusNotFound, gin.H{"error": "GraphQL is disabled"})
		return
	}

	var request struct {
		Query         string                 `json:"query"`
		Variables     map[string]interface{} `json:"variables"`
		OperationName string                 `json:"operationName"`
	}

	if err := c.ShouldBindJSON(&request); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid JSON"})
		return
	}

	// Execute GraphQL query
	result := graphql.Do(graphql.Params{
		Schema:         h.schema,
		RequestString:  request.Query,
		VariableValues: request.Variables,
		OperationName:  request.OperationName,
		Context:        context.WithValue(c.Request.Context(), "gin_context", c),
	})

	c.JSON(http.StatusOK, result)
}

// Playground serves the GraphQL playground
func (h *Handler) Playground(c *gin.Context) {
	if !h.config.Server.GraphQL.Enabled || !h.config.Server.GraphQL.Playground {
		c.JSON(http.StatusNotFound, gin.H{"error": "GraphQL playground is disabled"})
		return
	}

	playgroundHTML := `
<!DOCTYPE html>
<html>
<head>
    <meta charset=utf-8/>
    <meta name="viewport" content="user-scalable=no, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, minimal-ui">
    <title>GraphQL Playground</title>
    <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
    <link rel="shortcut icon" href="//cdn.jsdelivr.net/npm/graphql-playground-react/build/favicon.png" />
    <script src="//cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
</head>
<body>
    <div id="root">
        <style>
            body {
                background-color: rgb(23, 42, 58);
                font-family: Open Sans, sans-serif;
                height: 90vh;
            }
            #root {
                height: 100%;
                width: 100%;
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .loading {
                font-size: 32px;
                font-weight: 200;
                color: rgba(255, 255, 255, .6);
                margin-left: 20px;
            }
            img {
                width: 78px;
                height: 78px;
            }
            .title {
                font-weight: 400;
            }
        </style>
        <img src='//cdn.jsdelivr.net/npm/graphql-playground-react/build/logo.png' alt=''>
        <div class="loading"> Loading
            <span class="title">GraphQL Playground</span>
        </div>
    </div>
    <script>window.addEventListener('load', function (event) {
        GraphQLPlayground.init(document.getElementById('root'), {
            endpoint: '/graphql'
        })
    })</script>
</body>
</html>
`

	c.Header("Content-Type", "text/html")
	c.String(http.StatusOK, playgroundHTML)
}

// buildSchema builds the GraphQL schema
func (h *Handler) buildSchema() (graphql.Schema, error) {
	// Define custom types
	sessionType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Session",
		Fields: graphql.Fields{
			"id": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"name": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"status": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"config": &graphql.Field{
				Type: sessionConfigType,
			},
			"container_id": &graphql.Field{
				Type: graphql.String,
			},
			"created_at": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"updated_at": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"expires_at": &graphql.Field{
				Type: graphql.String,
			},
			"user_id": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
		},
	})

	automationScriptType := graphql.NewObject(graphql.ObjectConfig{
		Name: "AutomationScript",
		Fields: graphql.Fields{
			"id": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"name": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"description": &graphql.Field{
				Type: graphql.String,
			},
			"steps": &graphql.Field{
				Type: graphql.NewList(automationStepType),
			},
			"created_at": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"updated_at": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"user_id": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
		},
	})

	recordingType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Recording",
		Fields: graphql.Fields{
			"id": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"session_id": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"name": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"status": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"format": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"file_path": &graphql.Field{
				Type: graphql.String,
			},
			"file_size": &graphql.Field{
				Type: graphql.Int,
			},
			"duration": &graphql.Field{
				Type: graphql.String,
			},
			"started_at": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
			"ended_at": &graphql.Field{
				Type: graphql.String,
			},
			"user_id": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
			},
		},
	})

	// Define query type
	queryType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Query",
		Fields: graphql.Fields{
			"sessions": &graphql.Field{
				Type: graphql.NewList(sessionType),
				Args: graphql.FieldConfigArgument{
					"limit": &graphql.ArgumentConfig{
						Type:         graphql.Int,
						DefaultValue: 10,
					},
					"offset": &graphql.ArgumentConfig{
						Type:         graphql.Int,
						DefaultValue: 0,
					},
					"status": &graphql.ArgumentConfig{
						Type: graphql.String,
					},
				},
				Resolve: h.resolveSessions,
			},
			"session": &graphql.Field{
				Type: sessionType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{
						Type: graphql.NewNonNull(graphql.String),
					},
				},
				Resolve: h.resolveSession,
			},
			"automationScripts": &graphql.Field{
				Type: graphql.NewList(automationScriptType),
				Args: graphql.FieldConfigArgument{
					"limit": &graphql.ArgumentConfig{
						Type:         graphql.Int,
						DefaultValue: 10,
					},
					"offset": &graphql.ArgumentConfig{
						Type:         graphql.Int,
						DefaultValue: 0,
					},
				},
				Resolve: h.resolveAutomationScripts,
			},
			"recordings": &graphql.Field{
				Type: graphql.NewList(recordingType),
				Args: graphql.FieldConfigArgument{
					"limit": &graphql.ArgumentConfig{
						Type:         graphql.Int,
						DefaultValue: 10,
					},
					"offset": &graphql.ArgumentConfig{
						Type:         graphql.Int,
						DefaultValue: 0,
					},
					"session_id": &graphql.ArgumentConfig{
						Type: graphql.String,
					},
				},
				Resolve: h.resolveRecordings,
			},
		},
	})

	// Define mutation type
	mutationType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Mutation",
		Fields: graphql.Fields{
			"createSession": &graphql.Field{
				Type: sessionType,
				Args: graphql.FieldConfigArgument{
					"name": &graphql.ArgumentConfig{
						Type: graphql.NewNonNull(graphql.String),
					},
					"desktop": &graphql.ArgumentConfig{
						Type:         graphql.String,
						DefaultValue: "ubuntu-xfce",
					},
					"memory": &graphql.ArgumentConfig{
						Type:         graphql.Int,
						DefaultValue: 1024,
					},
					"cpu": &graphql.ArgumentConfig{
						Type:         graphql.Float,
						DefaultValue: 1.0,
					},
				},
				Resolve: h.resolveCreateSession,
			},
			"startSession": &graphql.Field{
				Type: sessionType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{
						Type: graphql.NewNonNull(graphql.String),
					},
				},
				Resolve: h.resolveStartSession,
			},
			"stopSession": &graphql.Field{
				Type: sessionType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{
						Type: graphql.NewNonNull(graphql.String),
					},
				},
				Resolve: h.resolveStopSession,
			},
			"deleteSession": &graphql.Field{
				Type: graphql.Boolean,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{
						Type: graphql.NewNonNull(graphql.String),
					},
				},
				Resolve: h.resolveDeleteSession,
			},
		},
	})

	// Build schema
	return graphql.NewSchema(graphql.SchemaConfig{
		Query:    queryType,
		Mutation: mutationType,
	})
}

// Resolver functions

func (h *Handler) resolveSessions(p graphql.ResolveParams) (interface{}, error) {
	// TODO: Implement actual session retrieval
	return []types.Session{}, nil
}

func (h *Handler) resolveSession(p graphql.ResolveParams) (interface{}, error) {
	sessionID := p.Args["id"].(string)
	// TODO: Implement actual session retrieval
	_ = sessionID
	return types.Session{}, nil
}

func (h *Handler) resolveAutomationScripts(p graphql.ResolveParams) (interface{}, error) {
	// TODO: Implement actual automation script retrieval
	return []types.AutomationScript{}, nil
}

func (h *Handler) resolveRecordings(p graphql.ResolveParams) (interface{}, error) {
	// TODO: Implement actual recording retrieval
	return []types.Recording{}, nil
}

func (h *Handler) resolveCreateSession(p graphql.ResolveParams) (interface{}, error) {
	name := p.Args["name"].(string)
	// TODO: Implement actual session creation
	_ = name
	return types.Session{}, nil
}

func (h *Handler) resolveStartSession(p graphql.ResolveParams) (interface{}, error) {
	sessionID := p.Args["id"].(string)
	// TODO: Implement actual session start
	_ = sessionID
	return types.Session{}, nil
}

func (h *Handler) resolveStopSession(p graphql.ResolveParams) (interface{}, error) {
	sessionID := p.Args["id"].(string)
	// TODO: Implement actual session stop
	_ = sessionID
	return types.Session{}, nil
}

func (h *Handler) resolveDeleteSession(p graphql.ResolveParams) (interface{}, error) {
	sessionID := p.Args["id"].(string)
	// TODO: Implement actual session deletion
	_ = sessionID
	return true, nil
}

// Helper types for GraphQL schema

var sessionConfigType = graphql.NewObject(graphql.ObjectConfig{
	Name: "SessionConfig",
	Fields: graphql.Fields{
		"desktop_environment": &graphql.Field{
			Type: desktopEnvironmentType,
		},
		"resources": &graphql.Field{
			Type: resourceLimitsType,
		},
		"display": &graphql.Field{
			Type: displayConfigType,
		},
		"audio": &graphql.Field{
			Type: audioConfigType,
		},
		"network": &graphql.Field{
			Type: networkConfigType,
		},
		"security": &graphql.Field{
			Type: securityConfigType,
		},
		"auto_destroy": &graphql.Field{
			Type: graphql.Boolean,
		},
		"max_duration": &graphql.Field{
			Type: graphql.String,
		},
	},
})

var desktopEnvironmentType = graphql.NewObject(graphql.ObjectConfig{
	Name: "DesktopEnvironment",
	Fields: graphql.Fields{
		"type": &graphql.Field{
			Type: graphql.NewNonNull(graphql.String),
		},
		"version": &graphql.Field{
			Type: graphql.String,
		},
		"image": &graphql.Field{
			Type: graphql.String,
		},
	},
})

var resourceLimitsType = graphql.NewObject(graphql.ObjectConfig{
	Name: "ResourceLimits",
	Fields: graphql.Fields{
		"cpu_cores": &graphql.Field{
			Type: graphql.Float,
		},
		"memory_mb": &graphql.Field{
			Type: graphql.Int,
		},
		"disk_space_gb": &graphql.Field{
			Type: graphql.Int,
		},
		"network_mbps": &graphql.Field{
			Type: graphql.Int,
		},
	},
})

var displayConfigType = graphql.NewObject(graphql.ObjectConfig{
	Name: "DisplayConfig",
	Fields: graphql.Fields{
		"width": &graphql.Field{
			Type: graphql.Int,
		},
		"height": &graphql.Field{
			Type: graphql.Int,
		},
		"depth": &graphql.Field{
			Type: graphql.Int,
		},
		"dpi": &graphql.Field{
			Type: graphql.Int,
		},
		"vnc_enabled": &graphql.Field{
			Type: graphql.Boolean,
		},
		"vnc_port": &graphql.Field{
			Type: graphql.Int,
		},
		"webrtc": &graphql.Field{
			Type: graphql.Boolean,
		},
	},
})

var audioConfigType = graphql.NewObject(graphql.ObjectConfig{
	Name: "AudioConfig",
	Fields: graphql.Fields{
		"enabled": &graphql.Field{
			Type: graphql.Boolean,
		},
		"driver": &graphql.Field{
			Type: graphql.String,
		},
		"sample_rate": &graphql.Field{
			Type: graphql.Int,
		},
		"channels": &graphql.Field{
			Type: graphql.Int,
		},
		"buffer_size": &graphql.Field{
			Type: graphql.Int,
		},
		"pulse_audio": &graphql.Field{
			Type: graphql.Boolean,
		},
	},
})

var networkConfigType = graphql.NewObject(graphql.ObjectConfig{
	Name: "NetworkConfig",
	Fields: graphql.Fields{
		"internet_access": &graphql.Field{
			Type: graphql.Boolean,
		},
		"allowed_domains": &graphql.Field{
			Type: graphql.NewList(graphql.String),
		},
		"blocked_domains": &graphql.Field{
			Type: graphql.NewList(graphql.String),
		},
		"dns_servers": &graphql.Field{
			Type: graphql.NewList(graphql.String),
		},
	},
})

var securityConfigType = graphql.NewObject(graphql.ObjectConfig{
	Name: "SecurityConfig",
	Fields: graphql.Fields{
		"isolation": &graphql.Field{
			Type: graphql.Boolean,
		},
		"readonly_rootfs": &graphql.Field{
			Type: graphql.Boolean,
		},
		"no_new_privileges": &graphql.Field{
			Type: graphql.Boolean,
		},
		"apparmor_profile": &graphql.Field{
			Type: graphql.String,
		},
		"seccomp_profile": &graphql.Field{
			Type: graphql.String,
		},
		"capabilities": &graphql.Field{
			Type: graphql.NewList(graphql.String),
		},
	},
})

var automationStepType = graphql.NewObject(graphql.ObjectConfig{
	Name: "AutomationStep",
	Fields: graphql.Fields{
		"id": &graphql.Field{
			Type: graphql.NewNonNull(graphql.Int),
		},
		"type": &graphql.Field{
			Type: graphql.NewNonNull(graphql.String),
		},
		"selector": &graphql.Field{
			Type: graphql.String,
		},
		"value": &graphql.Field{
			Type: graphql.String,
		},
		"wait_time": &graphql.Field{
			Type: graphql.String,
		},
	},
})