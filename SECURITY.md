# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | yes       |

## Reporting a Vulnerability

Email the maintainers. Please include a reproduction. Response time is
best-effort.

## Known Non-Issues

- `shout()` evaluates an expression built with `%`-formatting. The
  expression embeds a `repr()`'d literal of the (already uppercased)
  input and is evaluated with empty globals and locals, so only literal
  string concatenation can execute.
