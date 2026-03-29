# Configuration Reference

Ferrox is configured via a single YAML file. The config path is resolved in this order:

1. `LLM_PROXY_CONFIG` environment variable
2. `./config/config.yaml`
3. `/etc/ferrox/config.yaml`

Environment variables can be embedded anywhere in string values using `${VAR}` or `${VAR:-default}` syntax. Substitution is done on the already-parsed YAML value tree, so injected values with special characters are always safe.

## Top-level structure

```yaml
server:     { ... }
telemetry:  { ... }
defaults:   { ... }
providers:  [ ... ]
models:     [ ... ]
virtual_keys: [ ... ]
```

---

## server

```yaml
server:
  host: "0.0.0.0"          # bind address
  port: 8080                # bind port
  graceful_shutdown_timeout_secs: 30
  max_request_body_bytes: 10485760   # 10 MB

  timeouts:
    connect_secs: 10   # TCP + TLS handshake to provider
    ttfb_secs: 60      # wait for first SSE chunk (reasoning models can be slow)
    idle_secs: 30      # max silence between consecutive SSE chunks
```

All fields have defaults and are optional.

---

## telemetry

```yaml
telemetry:
  log_level: "info"    # trace | debug | info | warn | error
  log_format: "json"   # json | text

  metrics:
    enabled: true
    path: "/metrics"

  tracing:
    enabled: false
    otlp_endpoint: "http://otel-collector:4317"
    service_name: "ferrox"
    service_version: "0.1.0"
    sample_rate: 1.0   # 0.0 = off, 1.0 = 100%
```

---

## defaults

Applied to every provider unless the provider block overrides them.

```yaml
defaults:
  timeouts:
    connect_secs: 10
    ttfb_secs: 60
    idle_secs: 30

  retry:
    max_attempts: 3
    initial_backoff_ms: 100
    max_backoff_ms: 2000
    jitter: true          # adds random jitter up to initial_backoff_ms

  circuit_breaker:
    failure_threshold: 5    # failures before opening
    success_threshold: 2    # successful probes needed to close
    recovery_timeout_secs: 30
```

---

## providers

A list of upstream provider connections. You can have multiple entries of the same type to use multiple API keys or base URLs.

```yaml
providers:
  - name: anthropic-primary        # unique name, referenced by models
    type: anthropic                # anthropic | openai | gemini | bedrock
    api_key: "${ANTHROPIC_API_KEY}"
    base_url: "https://api.anthropic.com"   # optional override

    # Optional per-provider overrides:
    timeouts:
      ttfb_secs: 90
    retry:
      max_attempts: 2
    circuit_breaker:
      failure_threshold: 3
```

For Bedrock, omit `api_key` and set `region`. Credentials come from the AWS credential chain (environment variables, instance role, IRSA).

```yaml
  - name: bedrock-us
    type: bedrock
    region: "${AWS_REGION:-us-east-1}"
```

| Field | Required | Description |
|---|---|---|
| `name` | yes | Unique identifier used in model routing |
| `type` | yes | Provider type: `anthropic`, `openai`, `gemini`, `bedrock` |
| `api_key` | yes* | API key (*not required for Bedrock) |
| `base_url` | no | Override the default endpoint |
| `region` | no | AWS region (Bedrock only) |
| `timeouts` | no | Per-provider timeout overrides |
| `retry` | no | Per-provider retry overrides |
| `circuit_breaker` | no | Per-provider circuit breaker overrides |

---

## models

Each model alias maps client requests to a provider pool with a routing strategy.

```yaml
models:
  - alias: "claude-sonnet"        # the model name clients send in requests
    routing:
      strategy: weighted          # round_robin | weighted | failover | random
      targets:
        - provider: anthropic-primary
          model_id: "claude-sonnet-4-20250514"
          weight: 70              # only required for weighted strategy
        - provider: anthropic-secondary
          model_id: "claude-sonnet-4-20250514"
          weight: 30
      fallback:                   # tried in order when all targets fail
        - provider: bedrock-us
          model_id: "anthropic.claude-3-5-sonnet-20241022-v2:0"
```

| Routing strategy | Behavior |
|---|---|
| `round_robin` | Cycles through available targets in order |
| `weighted` | Distributes traffic proportionally by weight |
| `failover` | Always uses the first available target |
| `random` | Picks a random available target per request |

See [Routing](routing.md) for details on circuit breakers and fallback behavior.

---

## virtual_keys

Virtual keys are the credentials clients use to authenticate with Ferrox. Each key can be scoped to specific models and rate-limited independently.

```yaml
virtual_keys:
  - key: "${PROXY_KEY_APP}"     # the Bearer token clients send
    name: "my-app"              # unique name used in metrics/logs
    description: "Production app"   # optional
    allowed_models:
      - "claude-sonnet"
      - "gpt-4o"
    rate_limit:
      requests_per_minute: 120
      burst: 20
```

Use `allowed_models: ["*"]` to allow access to all model aliases.

See [Virtual Keys](virtual-keys.md) for more detail.

---

## Environment variable interpolation

All string values support `${VAR}` and `${VAR:-default}` syntax:

| Syntax | Behavior |
|---|---|
| `${VAR}` | Substitutes the value of `VAR`; errors if unset |
| `${VAR:-default}` | Uses `default` when `VAR` is unset or empty |

Interpolation happens on the already-parsed YAML tree. Values containing YAML special characters (`:`, `{`, `#`, etc.) are safe to inject.
