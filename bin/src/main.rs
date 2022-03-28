// mod address;
// mod consts;
// mod http_client;
// mod http_local;
// mod socks5;
use log::info;
use h2s::{http_local};
use clap::{App, Arg};
use futures::{
    future::{self, Either},
    FutureExt,
};
use std::net::SocketAddr;
use tokio::runtime::Builder;
use chrono::Local;
use std::io::Write;

fn main() {

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");

    let local_listen_addr: SocketAddr = "127.0.0.1:4188".parse().unwrap();
    let proxy_address: SocketAddr = "127.0.0.1:5180".parse().unwrap();


    let mut builder = Builder::new_current_thread();
    // if cfg!(feature = "single-threaded") {
    //     builder.basic_scheduler();
    // } else {
    //     builder.threaded_scheduler();
    // }
    info!("This is socks2http");
    let mut runtime = builder
        .enable_all()
        .build()
        .expect("Unable to create Tokio Runtime");
    runtime.block_on(async move {
        let res = h2s::startHttpsProxy("127.0.0.1:4188".to_string(), "127.0.0.1:5180".to_string()).await;

    })

}
