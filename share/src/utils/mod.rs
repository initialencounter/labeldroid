pub mod fs;

use std::net::SocketAddr;

pub use fs::*;


/// 异步版本：查找可用端口并返回已绑定的 TcpListener
/// 这样可以避免在查找端口和绑定端口之间出现竞态条件
pub async fn bind_available_port(
    start_port: u16,
) -> std::io::Result<(tokio::net::TcpListener, u16)> {
    const MAX_ATTEMPTS: u16 = 1000;

    for port in start_port..start_port.saturating_add(MAX_ATTEMPTS) {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                return Ok((listener, port));
            }
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::AddrInUse,
        format!("No available port found starting from {}", start_port),
    ))
}
