//! TLS configuration and synchronous TLS stream setup.

use std::{
    fs::File,
    io::{self, BufReader},
    net::TcpStream,
    path::Path,
    sync::Arc,
};

use rustls::{
    RootCertStore, ServerConfig, ServerConnection, StreamOwned,
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
};

use crate::TlsConfig;

/// Loads the server certificate/key and builds a TLS config.
/// If `client_ca_file` is set, client certificates signed by that CA are required.
pub fn load_server_config(config: &TlsConfig) -> io::Result<Arc<ServerConfig>> {
    let certs = read_certificates(&config.cert_file)?;
    if certs.is_empty() {
        return Err(invalid_data(
            "TLS certificate file contains no certificates",
        ));
    }
    let key = read_private_key(&config.key_file)?;

    let builder = ServerConfig::builder();
    let server = if let Some(ca_file) = &config.client_ca_file {
        let mut roots = RootCertStore::empty();
        for cert in read_certificates(ca_file)? {
            roots
                .add(cert)
                .map_err(|e| invalid_data(format!("invalid client CA certificate: {e}")))?;
        }
        if roots.is_empty() {
            return Err(invalid_data("TLS client CA file contains no certificates"));
        }
        let verifier = WebPkiClientVerifier::builder(Arc::new(roots))
            .build()
            .map_err(|e| invalid_data(format!("invalid TLS client CA: {e}")))?;
        builder
            .with_client_cert_verifier(verifier)
            .with_single_cert(certs, key)
            .map_err(|e| invalid_data(format!("invalid TLS certificate/key: {e}")))?
    } else {
        builder
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| invalid_data(format!("invalid TLS certificate/key: {e}")))?
    };

    Ok(Arc::new(server))
}

/// Performs the TLS handshake and returns an encrypted TCP stream.
/// Call this in the connection worker before reading the HTTP request.
pub fn accept_tls(
    stream: TcpStream,
    config: Arc<ServerConfig>,
) -> io::Result<StreamOwned<ServerConnection, TcpStream>> {
    let mut connection = ServerConnection::new(config)
        .map_err(|e| invalid_data(format!("failed to create TLS connection: {e}")))?;
    while connection.is_handshaking() {
        connection.complete_io(&mut &stream)?;
    }
    Ok(StreamOwned::new(connection, stream))
}

fn read_certificates(path: impl AsRef<Path>) -> io::Result<Vec<CertificateDer<'static>>> {
    rustls_pemfile::certs(&mut BufReader::new(File::open(path)?))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| invalid_data(format!("invalid PEM certificate: {e}")))
}

fn read_private_key(path: impl AsRef<Path>) -> io::Result<PrivateKeyDer<'static>> {
    rustls_pemfile::private_key(&mut BufReader::new(File::open(path)?))?
        .ok_or_else(|| invalid_data("TLS key file contains no supported private key"))
}

fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}
