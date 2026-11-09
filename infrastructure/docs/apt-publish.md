# Publish the APT repository

The workflow builds and publishes `amd64` and `arm64` packages when a version tag is pushed.

## One-time setup

1. Set GitHub Pages to deploy from **GitHub Actions**.
2. Add the Actions secret `APT_GPG_PRIVATE_KEY`.
3. Add the Actions variable `APT_GPG_KEY_ID`.
4. If the key has a passphrase, add the Actions secret `APT_GPG_PASSPHRASE`.

## Publish a release

```sh
git tag v1.1.1
git push origin v1.1.1
```

The tag must match the version in `Cargo.toml`.
