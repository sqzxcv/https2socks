use std::net::SocketAddr;
use log::error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub mod address;
pub mod socks5;
pub mod http_local;
mod consts;
mod http_client;

pub async fn startHttpsProxy(listen_addr: String, proxy_address: String) -> std::io::Result<()> {
    let local_listen_addr: SocketAddr = listen_addr.parse().expect(format!("{:?} 地址不合法,解析失败", listen_addr).as_str());
    let proxy_address_addr: SocketAddr = proxy_address.parse().expect(format!("{:?} 地址不合法,解析失败", proxy_address).as_str());
    http_local::run(local_listen_addr, proxy_address_addr).await
}