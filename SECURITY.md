# Security Policy

## Reporting a Vulnerability

We take security seriously and appreciate your help in keeping Ferrox safe for everyone.

### GitHub Issues

For most security concerns, open a GitHub issue describing the vulnerability, its potential impact, and steps to reproduce.

### Private Disclosure for Critical Vulnerabilities

If you discover a zero-day vulnerability or a critical issue that could be actively exploited, report it privately by email:

**hello@shaharialab.com**

Use email when:

- The vulnerability is a zero-day or has no known fix.
- Public disclosure could put users at immediate risk.
- The issue involves sensitive data exposure or remote code execution.
- You believe the vulnerability is being actively exploited.

Please include:

- A description of the vulnerability.
- Steps to reproduce or a proof of concept.
- The potential impact and affected components.
- Any suggested fixes, if you have them.

We will acknowledge your report within 48 hours and work with you to coordinate a fix before any public disclosure.

## Supported Versions

Security fixes are applied to the latest release. Always run the most recent version of Ferrox.

## API Key and Credential Safety

Ferrox acts as a gateway that holds upstream provider API keys and issues virtual keys to clients. Keep the following in mind:

- **Config files** contain API keys via environment variable references (`${VAR}`). Never commit a config file with literal API keys to version control.
- **Virtual keys** are Bearer tokens. Treat them like passwords. Rotate them if you suspect compromise.
- **Upstream provider keys** (Anthropic, OpenAI, etc.) should have the minimum required permissions. Use separate keys per environment.
- **AWS Bedrock** credentials follow the standard AWS credential chain. Use IAM roles with least-privilege policies. Prefer instance roles or IRSA over long-lived access keys.
- **Kubernetes secrets** storing API keys should have RBAC restricted to only the Ferrox service account.
- **Docker Compose** reads keys from environment variables or a `.env` file. Never commit `.env` to version control. Add it to `.gitignore`.

## Known Security Considerations

- **Rate limiting is per-instance.** In a multi-replica deployment, each pod enforces its own token bucket independently. This is approximate. For strict cross-replica enforcement, place an external rate limiter in front of Ferrox.
- **The `/metrics` endpoint is unauthenticated.** It can leak request volume, model usage, key names, and error rates. Restrict access to this endpoint at the network level (ingress, security group, or NetworkPolicy) in production.
- **Request bodies are forwarded verbatim to upstream providers.** Ferrox does not sanitize or inspect message content. Ensure your clients send only intended content.
