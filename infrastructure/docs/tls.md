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

## Experimental HTTP/3

HTTP/3 is an opt-in experimental feature using QUIC over UDP. It requires TLS and supports the same optional client CA for mTLS. Add this to the config:

```yaml
tls:
  cert_file: /etc/pixie/tls/server-chain.pem
  key_file: /etc/pixie/tls/server-key.pem
  # client_ca_file: /etc/pixie/tls/client-ca.pem
http3:
  addr: 0.0.0.0:8443
```

Expose UDP port 8443 in the firewall and deployment. Docker Compose maps it, and Helm exposes it when `http3.enabled: true`; TLS certificate mounts still need to be configured by the operator. The HTTP/3 implementation depends on the experimental Rust `h3` crate and should be evaluated before production use.
