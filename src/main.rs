use hickory_client::client::{Client, SyncClient};
use hickory_client::h2::HttpsClientConnection; // Add this import
use hickory_client::rr::{DNSClass, Name, RecordType};
use hickory_proto::iocompat::AsyncIoTokioAsStd;
use rustls::{ClientConfig, RootCertStore};
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

fn main() {

    let name_server = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 443);
    let root_store = rustls::RootCertStore::empty();
    /*root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject.into(),
            ta.subject_public_key_info.into(),
            ta.name_constraints.into(),
        )
    }));*/
    //let root_store = RootCertStore {
    //    roots: webpki_roots::TLS_SERVER_ROOTS.iter().cloned().collect(),
    //};

    let _client_config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let client_config = rustls_platform_verifier::tls_config();

    let shared_client_config = Arc::new(client_config);
    let conn: HttpsClientConnection<AsyncIoTokioAsStd<tokio::net::TcpStream>> =
        HttpsClientConnection::new(name_server, "dns.google".to_string(), shared_client_config);

    let client = SyncClient::new(conn);
    // Specify the name, note the final '.' which specifies it's an FQDN
    let host_to_lookup = "example.com".to_string();
    let name = Name::from_ascii(host_to_lookup).unwrap();

    let dns_class = DNSClass::IN;
    let record_type = RecordType::A;
    let response = client.query(&name, dns_class, record_type);
    match response {
        Ok(answer) => {
            println!("ok={:?}", answer);
        }
        Err(e) => {
            println!("got err Resp={:?}", e);
        }
    }
}
