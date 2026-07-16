use std::fs;
use std::io;
use std::sync::Arc;

/// Raw PEM bytes — this is what tiny_http wants directly.
pub struct CertPaths {
    pub cert_path: String,
    pub key_path: String,
}

pub fn init(cert_path: &str, key_path: &str) -> ((Vec<u8>, Vec<u8>), Arc<rustls::ServerConfig>){
    let paths = CertPaths {
        cert_path: cert_path.into(),
        key_path: key_path.into(),
    };

    let (cert_pem, key_pem) = read_pem_bytes(&paths).expect("failed to read cert/key files");
    let rustls_config = build_rustls_config(&cert_pem, &key_pem);

    return ((cert_pem, key_pem), rustls_config)
}

fn read_pem_bytes(paths: &CertPaths) -> io::Result<(Vec<u8>, Vec<u8>)> {
    let cert = fs::read(&paths.cert_path)?;
    let key = fs::read(&paths.key_path)?;
    Ok((cert, key))
}

/// Build a rustls ServerConfig from the same PEM bytes — this is what
/// the websocket server wraps each TcpStream in.
fn build_rustls_config(cert_pem: &[u8], key_pem: &[u8]) -> Arc<rustls::ServerConfig> {
    let certs = rustls_pemfile::certs(&mut &*cert_pem)
        .collect::<Result<Vec<_>, _>>()
        .expect("invalid certificate PEM");

    let key = rustls_pemfile::private_key(&mut &*key_pem)
        .expect("invalid private key PEM")
        .expect("no private key found in file");

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("bad certificate/key combination");

    Arc::new(config)
}
