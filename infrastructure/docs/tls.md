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

When the `tls` section is present, Pixie loads the credentials at startup and performs a TLS handshake for each accepted connection. Without it, the listener serves plain HTTP. A deployment must expose the configured TCP port and mount the certificate, key, and optional CA files. Existing deployment templates do not mount these files automatically; configure those mounts in the deployment environment.
