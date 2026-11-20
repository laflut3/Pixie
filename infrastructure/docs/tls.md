# TLS and mTLS

Pixie can serve HTTPS when TLS is configured. Without a `tls` section it keeps the existing plain HTTP listener.

```yaml
addr: 0.0.0.0:8443
workers: 4
tls:
  cert_file: /etc/pixie/tls/server-chain.pem
  key_file: /etc/pixie/tls/server-key.pem
  # Optional: requiring a client certificate enables mTLS.
  # client_ca_file: /etc/pixie/tls/client-ca.pem
```

The certificate file must contain the server certificate followed by any intermediate certificates. The private key must be an unencrypted PEM key supported by rustls. Keep private keys readable only by the Pixie service account. The client CA file must contain trusted PEM CA certificates; when configured, clients without a certificate trusted by that CA are rejected during the TLS handshake.

TLS requires integration at the connection accept path. The library API `load_server_config` builds the rustls configuration, and `accept_tls` performs a handshake on an accepted TCP stream. A deployment must expose the configured TCP port and mount the certificate, key, and optional CA files. Existing deployment templates do not enable TLS or mount these files automatically.
