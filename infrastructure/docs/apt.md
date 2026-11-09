# Debian / Ubuntu (APT)

Pixie uses its own signed APT repository. Install and start it with:

```sh
curl -fsSL https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/apt/install.sh | bash
sudo systemctl enable --now pixie
curl http://localhost:8080/hello
```

Supported architectures: `amd64` and `arm64`.

First-time repository setup: [APT publishing](apt-publish.md).
