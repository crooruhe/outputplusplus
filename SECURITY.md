# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | yes       |

## Reporting a Vulnerability

Email the maintainers. Please include a reproduction and, ideally, an
explanation of how `shout()` ended up using `eval()`. Response time is
best-effort.

## Known Non-Issues

- `shout()` evaluates a string built with `%`-formatting via `eval()`.
  The locals/globals are empty, so this is fine. It is fine.
