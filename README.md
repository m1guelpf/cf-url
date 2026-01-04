# cf-url

Quick access to Cloudflare dashboard pages from your terminal.

## Installation

```bash
cargo install cfurl
```

## Usage

```bash
# Go to the DNS dashboard for miguel.build
cfurl dns miguel.build

# Workers dashboard
cfurl workers
cfurl workers my-worker  # specific worker

# R2 storage
cfurl r2
cfurl r2 my-bucket  # specific bucket

# D1 databases
cfurl d1
cfurl d1 my-database

# KV namespaces
cfurl kv

# Security settings
cfurl security miguel.build
cfurl security miguel.build -s waf    # WAF rules
cfurl security miguel.build -s events # Security events

# SSL/TLS
cfurl ssl miguel.build

# Caching
cfurl caching miguel.build

# Rules (redirects, transforms)
cfurl rules miguel.build

# Speed/optimization
cfurl speed miguel.build

# Email routing
cfurl email miguel.build

# Zero Trust / Access
cfurl zero-trust
cfurl access
cfurl tunnels

# Developer platform
cfurl pages
cfurl stream
cfurl images
cfurl queues
cfurl ai
cfurl vectorize
cfurl hyperdrive
cfurl durable-objects

# Account management
cfurl account
cfurl billing
cfurl audit-log
cfurl api-tokens
cfurl registrar

# Other
cfurl turnstile
cfurl zaraz miguel.build
cfurl analytics
cfurl logs
cfurl zone miguel.build  # zone overview
cfurl dash               # main dashboard
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
