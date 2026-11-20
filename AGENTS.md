# Pixie

Pixie is a lightweight Rust HTTP server for serving static web pages. It includes configuration, routing, logging, and deployment files for Linux packages, Docker, and Kubernetes.

## Project rules

- Keep documentation in English as a responsive static site in `docs/` (plain HTML/CSS/JS, no build required). Split it into user guides (deploy, use, configure) and feature explanations. Review and update relevant pages with every behavior, configuration, or deployment change.
- Keep the README focused on deployment methods and the documentation link. Use shared site styles and navigation; keep examples accurate and planned features clearly labeled.
- Run only checks that provide evidence for the change. Avoid unrelated or repetitive verification.
- Choose the least expensive model that can reliably complete each task; reserve stronger models for work that needs them.
- Keep the design and dependencies simple. Add an abstraction or dependency only when it solves a concrete need.
- Keep Pixie deployable through every supported path: local Cargo builds, Docker/Compose, Debian/APT, Arch, and Kubernetes/Helm. Update affected packaging and deployment files with application changes.
- Treat HTTP protocol support and TLS/mTLS as security-sensitive. Use established, maintained libraries and document required certificates, keys, and configuration.
- Do not commit, publish, or deploy unless the user explicitly asks.

## Useful skills

- `humanizer`: review English documentation for clear, natural wording.
- `fast-codex-workflow`: make focused changes and run only the checks needed to verify them.
- `security-review`: use when changing TLS/mTLS, request parsing, routing, or other security-sensitive server behavior.
- `workers-best-practices`: use only if Pixie adds Cloudflare Workers code.

