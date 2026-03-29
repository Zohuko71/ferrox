# Virtual Keys

Virtual keys are the credentials clients use to authenticate with Ferrox. Each key can be scoped to specific models and rate-limited independently.

Ferrox itself does not call upstream providers with these keys. Each virtual key maps to one or more upstream provider API keys through the routing config.

## Configuration

```yaml
virtual_keys:
  - key: "${PROXY_KEY_APP}"     # Bearer token clients send in Authorization header
    name: "my-app"              # unique name; appears in logs and metrics
    description: "Production API"
    allowed_models:
      - "claude-sonnet"
      - "gpt-4o"
    rate_limit:
      requests_per_minute: 120
      burst: 20
```

## Authentication

Clients authenticate with a standard HTTP Bearer token:

```
Authorization: Bearer <virtual-key>
```

Requests without this header, or with an unrecognized key, receive a `401 Unauthorized` response.

## Model access control

`allowed_models` is a list of model aliases the key is permitted to use. Set to `["*"]` to allow all configured aliases.

```yaml
allowed_models: ["*"]                         # all aliases
allowed_models: ["claude-sonnet", "gpt-4o"]   # specific aliases only
```

Requests to a model alias not in the list receive a `403 Forbidden` response.

## Rate limiting

Ferrox uses a lock-free token bucket per key, per instance. It is approximate for multi-instance deployments (each instance maintains its own bucket independently).

```yaml
rate_limit:
  requests_per_minute: 120   # sustained throughput
  burst: 20                  # max instantaneous burst
```

When the bucket is empty, the request is rejected immediately with `429 Too Many Requests`.

The `burst` value sets the bucket capacity. A fully-charged bucket allows `burst` requests instantly before the sustained limit applies.

### Disabling rate limiting

Omit the `rate_limit` field to remove limits for a key:

```yaml
virtual_keys:
  - key: "${PROXY_KEY_INTERNAL}"
    name: "internal-batch-job"
    allowed_models: ["*"]
    # no rate_limit
```

## Example: multi-tenant setup

```yaml
virtual_keys:
  # Internal service: unrestricted access
  - key: "${KEY_INTERNAL}"
    name: "data-pipeline"
    allowed_models: ["*"]

  # Customer A: claude only, 60 rpm
  - key: "${KEY_CUSTOMER_A}"
    name: "customer-a"
    allowed_models: ["claude-sonnet", "claude-haiku"]
    rate_limit:
      requests_per_minute: 60
      burst: 10

  # Customer B: limited to cheap models, 30 rpm
  - key: "${KEY_CUSTOMER_B}"
    name: "customer-b"
    allowed_models: ["claude-haiku", "gemini-flash"]
    rate_limit:
      requests_per_minute: 30
      burst: 5
```

## Metrics

Ferrox records the following metrics per key:

- `ferrox_requests_total{key_name=...}` - requests dispatched
- `ferrox_rate_limited_total{key_name=...}` - requests rejected by rate limiter

See [Observability](observability.md) for the full metrics reference.
