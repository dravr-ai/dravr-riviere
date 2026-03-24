# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability in dravr-riviere, please report it responsibly:

1. **Do not** open a public GitHub issue
2. Email **security@dravr.ai** with a description of the vulnerability
3. Include steps to reproduce, if possible
4. You will receive an acknowledgment within 48 hours

We will work with you to understand the issue and coordinate a fix before any public disclosure.

## Security Model

Dravr-riviere is a time-series storage library. The security boundary is:

- **No secrets in core** -- the library stores no API keys or tokens
- **Input validation** -- all numeric inputs are validated against domain ranges
- **No SQL injection** -- queries use parameterized statements (when Postgres-backed)
