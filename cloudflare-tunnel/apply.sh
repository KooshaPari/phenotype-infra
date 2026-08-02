#!/usr/bin/env bash
# apply.sh — apply pinned ingress.json to Cloudflare Tunnel via API.
# Source of truth: ./ingress.json (committed). Daemon auto-reloads ~15s.
# Usage: CF_API_TOKEN=... ./apply.sh
set -euo pipefail

ACCOUNT_ID="${CF_ACCOUNT_ID:-49dce512822987e0522f0faeffbcc0c8}"
TUNNEL_ID="${CF_TUNNEL_ID:-6daf5b3d-127e-43a7-8af1-67559de3aa60}"
CONFIG_FILE="${1:-$(dirname "$0")/ingress.json}"

: "${CF_API_TOKEN:?CF_API_TOKEN required (load from ~/.env)}"

# Wrap ingress.json in the {config: ...} envelope the API expects
TMP=$(mktemp)
trap 'rm -f "$TMP"' EXIT
python -c "
import json, sys
cfg = json.load(open('$CONFIG_FILE'))
print(json.dumps({'config': cfg}))
" > "$TMP"

curl -sS --fail-with-body -X PUT \
  -H "Authorization: Bearer $CF_API_TOKEN" \
  -H "Content-Type: application/json" \
  --data @"$TMP" \
  "https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/cfd_tunnel/${TUNNEL_ID}/configurations" \
  | python -c "
import json, sys
r = json.load(sys.stdin)
if r.get('success'):
    cfg = r['result']['config']
    print(f\"OK: applied v{cfg.get('version', '?')} ({len(cfg['ingress'])-1} hostnames + 404 fallback)\")
    for rule in cfg['ingress']:
        if 'hostname' in rule:
            print(f\"  - {rule['hostname']:40s} -> {rule['service']}\")
else:
    print('ERROR:', json.dumps(r, indent=2))
    sys.exit(1)
"
