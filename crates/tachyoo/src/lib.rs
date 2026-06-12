pub mod options;
pub mod player_data;
pub mod error;

use std::{
    collections::HashMap,
    net::{IpAddr, Ipv6Addr},
};

use console_subscriber::Server;
use tokio::{net::TcpListener, sync::mpsc};

use crate::options::StartOptions;

use snafu::prelude::*;

//TODO: determine HashDoS risk/exposure
type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;



pub fn run(options: StartOptions) -> Result<(), ServerError> {
    // hopefully sufficient?
    #[cfg(feature = "tokio_console")]
    console_subscriber::Builder::default()
        .with_default_env()
        .init();

    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()?
        .block_on(run_inner(options))
}

async fn run_inner(options: StartOptions) -> Result<(), ServerError> {
    //TODO: determine buffer size
    let (conn_tx, conn_rx) = mpsc::channel(100);

    let listen_task =
        tokio::spawn(async { 
            let listener=TcpListener::bind((Ipv6Addr::LOCALHOST, options.port)).await;
            loop {
                let conn=listener.
            }
        });
    todo!()
}