# API Reference

Ferrox exposes an OpenAI-compatible HTTP API. Existing OpenAI SDK clients work without modification by pointing the base URL at Ferrox.

## Base URL

```
http://your-ferrox-host:8080
```

## Authentication

All `/v1/*` endpoints require a Bearer token in the `Authorization` header:

```
Authorization: Bearer <virtual-key>
```

Health and metrics endpoints are public.

---

## POST /v1/chat/completions

Send a chat completion request. Ferrox routes it to the configured provider based on the `model` field.

### Request

```json
{
  "model": "claude-sonnet",
  "messages": [
    {"role": "system", "content": "You are a helpful assistant."},
    {"role": "user", "content": "Hello"}
  ],
  "stream": false,
  "temperature": 0.7,
  "max_tokens": 1024,
  "top_p": 1.0,
  "stop": ["END"],
  "tools": [ ... ],
  "tool_choice": "auto"
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `model` | string | yes | Model alias from your config |
| `messages` | array | yes | Conversation history |
| `stream` | boolean | no | Enable SSE streaming (default: false) |
| `temperature` | float | no | Sampling temperature |
| `max_tokens` | integer | no | Max tokens to generate |
| `top_p` | float | no | Nucleus sampling |
| `stop` | string or array | no | Stop sequences |
| `tools` | array | no | Tool definitions |
| `tool_choice` | string or object | no | Tool selection mode |

Unknown fields are forwarded to the provider as-is.

### Non-streaming response

```json
{
  "id": "chatcmpl-abc123",
  "object": "chat.completion",
  "created": 1735000000,
  "model": "claude-sonnet",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hello! How can I help you today?"
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 15,
    "completion_tokens": 12,
    "total_tokens": 27
  }
}
```

### Streaming response

When `stream: true`, responses are sent as Server-Sent Events:

```
data: {"id":"chatcmpl-abc","object":"chat.completion.chunk","created":1735000000,"model":"claude-sonnet","choices":[{"index":0,"delta":{"role":"assistant","content":"Hello"},"finish_reason":null}]}

data: {"id":"chatcmpl-abc","object":"chat.completion.chunk","created":1735000000,"model":"claude-sonnet","choices":[{"index":0,"delta":{"content":"!"},"finish_reason":null}]}

data: {"id":"chatcmpl-abc","object":"chat.completion.chunk","created":1735000000,"model":"claude-sonnet","choices":[{"index":0,"delta":{},"finish_reason":"stop"}],"usage":{"prompt_tokens":15,"completion_tokens":2,"total_tokens":17}}

data: [DONE]
```

### Error responses

All errors use OpenAI error format:

```json
{
  "error": {
    "message": "Key 'my-app' is not authorized to use model 'gpt-4o'",
    "type": "forbidden",
    "code": 403
  }
}
```

| Status | Type | Cause |
|---|---|---|
| 401 | `unauthorized` | Missing or invalid API key |
| 403 | `forbidden` | Key not allowed to use this model |
| 404 | `model_not_found` | Model alias not in config |
| 429 | `rate_limited` | Per-key rate limit exceeded |
| 500 | `stream_error` | Upstream streaming failure |
| 502 | `circuit_open` | Circuit breaker open; all targets unavailable |
| 502 | `provider_error` | Provider returned an error |
| 504 | `upstream_timeout` | Provider did not respond in time |

---

## GET /v1/models

List all configured model aliases.

### Response

```json
{
  "object": "list",
  "data": [
    {
      "id": "claude-sonnet",
      "object": "model",
      "created": 1735000000,
      "owned_by": "ferrox"
    },
    {
      "id": "gpt-4o",
      "object": "model",
      "created": 1735000000,
      "owned_by": "ferrox"
    }
  ]
}
```

---

## GET /healthz

Liveness check. Always returns `200 OK` with body `ok` if the process is running.

---

## GET /readyz

Readiness check. Returns `200 OK` with body `ready` when the server has finished startup. Returns `503 Service Unavailable` during startup or graceful shutdown drain.

Use `/readyz` for readiness probes and load balancer health checks.

---

## GET /metrics

Prometheus metrics in text exposition format (content type `text/plain; version=0.0.4`).

No authentication required. See [Observability](observability.md) for the full metric list.

---

## Using OpenAI SDKs

Point the base URL at Ferrox and use your virtual key:

**Python:**

```python
from openai import OpenAI

client = OpenAI(
    api_key="sk-proxy-key",
    base_url="http://localhost:8080/v1"
)

response = client.chat.completions.create(
    model="claude-sonnet",
    messages=[{"role": "user", "content": "Hello"}]
)
```

**Node.js:**

```javascript
import OpenAI from "openai";

const client = new OpenAI({
  apiKey: "sk-proxy-key",
  baseURL: "http://localhost:8080/v1",
});

const response = await client.chat.completions.create({
  model: "claude-sonnet",
  messages: [{ role: "user", content: "Hello" }],
});
```
