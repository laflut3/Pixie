# Dependabot

`.github/dependabot.yml` checks Cargo, GitHub Actions, and Docker base images weekly. Rust minor and patch updates share one pull request. Review updates before merging; CI runs without release secrets.

In repository Settings, enable the dependency graph, Dependabot alerts, and Dependabot security updates under Code security. Keep GitHub Actions enabled. Public dependencies need no token or Dependabot secret.

Use standard GitHub-hosted runners. Dependabot update jobs are free; this public repository's normal CI is also free on standard runners. Larger runners are billed. See [GitHub's Dependabot runner documentation](https://docs.github.com/en/code-security/concepts/supply-chain-security/dependabot-on-actions) and [Actions billing](https://docs.github.com/en/billing/concepts/product-billing/github-actions).
