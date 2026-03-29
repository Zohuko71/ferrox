# Quick Start

This guide gets Ferrox running locally in under 5 minutes.

## Prerequisites

- Rust 1.74+ (`rustup update stable`)
- At least one LLM provider API key

## 1. Clone and build

```bash
git clone https://github.com/shaharia-lab/ferrox
cd ferrox
cargo build --release
```

## 2. Configure

Copy the example config:

```bash
cp config/config.yaml config/local.yaml
```

Edit `config/local.yaml`. At minimum, set one provider and one model:

```yaml
providers:
  - name: anthropic
    type: anthropic
    api_key: "${ANTHROPIC_API_KEY}"

models:
  - alias: claude-sonnet
    routing:
      strategy: failover
      targets:
        - provider: anthropic
          model_id: claude-sonnet-4-20250514

virtual_keys:
  - key: "${PROXY_KEY:-sk-local-dev}"
    name: local-dev
    allowed_models: ["*"]
    rate_limit:
      requests_per_minute: 120
      burst: 20
```

## 3. Set environment variables

```bash
export ANTHROPIC_API_KEY=sk-ant-...
export PROXY_KEY=sk-local-dev
```

## 4. Run

```bash
LLM_PROXY_CONFIG=config/local.yaml ./target/release/ferrox
```

## 5. Send a request

```bash
curl http://localhost:8080/v1/chat/completions \
  -H "Authorization: Bearer sk-local-dev" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "claude-sonnet",
    "messages": [{"role": "user", "content": "Say hello in one sentence."}]
  }'
```

## 6. Verify health

```bash
curl http://localhost:8080/healthz   # -> "ok"
curl http://localhost:8080/readyz    # -> "ready" when startup is complete
curl http://localhost:8080/metrics   # -> Prometheus text format
```

## Using Docker Compose

The fastest way to get the full observability stack running:

```bash
cp config/config.yaml config/local.yaml
# Edit config/local.yaml with your keys
docker compose up
```

This starts Ferrox, Prometheus (`:9090`), Grafana (`:3000`), Jaeger (`:16686`), and the OTEL Collector.

## Next steps

- [Configuration reference](configuration.md) - all config options
- [Providers](providers.md) - add OpenAI, Gemini, or Bedrock
- [Routing](routing.md) - set up failover and weighted routing
- [Virtual keys](virtual-keys.md) - issue scoped keys for each service
