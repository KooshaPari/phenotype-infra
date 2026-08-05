use std::io::{Read, Write};
use std::net::{TcpListener, SocketAddr};
use std::process::Command;
use std::thread::{self, JoinHandle};

fn response_server(status: &str, body: &str) -> (SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind test server");
    let address = listener.local_addr().expect("server address");
    let status = status.to_owned();
    let body = body.to_owned();
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept client");
        let mut request = [0_u8; 4096];
        let _ = stream.read(&mut request);
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream
            .write_all(response.as_bytes())
            .expect("write test response");
    });
    (address, handle)
}

fn run_keygen(address: SocketAddr) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tailscale-keygen"))
        .args([
            "--api-key",
            "tskey-api-test",
            "--tailnet",
            "test.example",
            "--tag",
            "tag:oci",
            "--ttl",
            "60",
            "--api-base",
            &format!("http://{address}"),
            "--format",
            "text",
        ])
        .env("NO_COLOR", "1")
        .output()
        .expect("run tailscale-keygen")
}

#[test]
fn successful_api_response_prints_only_key() {
    let (address, server) = response_server("200 OK", r#"{"key":"tskey-test"}"#);
    let output = run_keygen(address);
    server.join().expect("server thread");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "tskey-test\n");
}

#[test]
fn api_error_is_reported_without_key_output() {
    let (address, server) = response_server("400 Bad Request", r#"{"message":"denied"}"#);
    let output = run_keygen(address);
    server.join().expect("server thread");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("tailscale API returned"));
    assert!(output.stdout.is_empty());
}

#[test]
fn empty_api_key_is_rejected_before_request() {
    let output = Command::new(env!("CARGO_BIN_EXE_tailscale-keygen"))
        .args([
            "--api-key",
            "",
            "--tailnet",
            "test.example",
            "--api-base",
            "http://127.0.0.1:1",
        ])
        .output()
        .expect("run tailscale-keygen");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("TS_API_KEY is empty"));
}
