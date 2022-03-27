use std::net::SocketAddr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod monitor;
pub mod address;
pub mod socks5;
pub mod http_local;
mod consts;
mod http_client;

pub async fn startHttpsProxy(listen_addr: SocketAddr, proxy_address: SocketAddr) -> std::io::Result<()> {
    http_local::run(listen_addr, proxy_address).await
}