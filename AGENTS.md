# Pixie

Pixie is a lightweight Rust HTTP server for serving static web pages. It includes configuration, routing, logging, and deployment files for Linux packages, Docker, and Kubernetes.

## Project rules

- Keep user-facing documentation in English. Review and update relevant docs with every behavior, configuration, or deployment change.
- Run only checks that provide evidence for the change. Avoid unrelated or repetitive verification.
- Keep the design and dependencies simple. Add an abstraction or dependency only when it solves a concrete need.
- Keep Pixie deployable through every supported path: local Cargo builds, Docker/Compose, Debian/APT, Arch, and Kubernetes/Helm. Update affected packaging and deployment files with application changes.
- Treat HTTP protocol support and TLS/mTLS as security-sensitive. Use established, maintained libraries and document required certificates, keys, and configuration.
- Do not commit, publish, or deploy unless the user explicitly asks.

## Useful skills

- `humanizer`: review English documentation for clear, natural wording.
- `fast-codex-workflow`: make focused changes and run only the checks needed to verify them.
- `security-review`: use when changing TLS/mTLS, request parsing, routing, or other security-sensitive server behavior.
- `workers-best-practices`: use only if Pixie adds Cloudflare Workers code.

