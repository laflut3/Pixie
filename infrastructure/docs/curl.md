# Install with curl

```sh
curl -fsSL https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/apt/install.sh | bash
sudo systemctl enable --now pixie
curl http://localhost:8080/hello
```

Debian/Ubuntu only. Supports `amd64` and `arm64`.
