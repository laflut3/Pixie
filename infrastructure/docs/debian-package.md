# Build and install a Debian package

```sh
infrastructure/scripts/build-deb.sh
sudo apt install ./pixie_*.deb
sudo systemctl enable --now pixie
curl http://localhost:8080/hello
```
