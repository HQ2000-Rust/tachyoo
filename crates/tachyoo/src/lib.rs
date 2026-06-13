pub mod error;
pub mod options;
pub mod player_data;

use crate::error::*;
use snafu::prelude::*;

use std::net::Ipv6Addr;

use tokio::{net::TcpListener, sync::mpsc, task::JoinHandle};

use crate::{error::ServerError, options::StartOptions};

//TODO: determine HashDoS risk/exposure
type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

//TODO maybe use the want crate

pub fn run(options: StartOptions) -> Result<(), ServerError> {
    // hopefully sufficient?
    #[cfg(feature = "tokio_console")]
    console_subscriber::Builder::default()
        .with_default_env()
        .init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .context(RuntimeCreationSnafu {})?
        .block_on(run_inner(options))
}

async fn run_inner(options: StartOptions) -> Result<(), ServerError> {
    //TODO: determine buffer size
    let (conn_tx, conn_rx) = mpsc::channel(100);

    let listen_task: JoinHandle<Result<(), ServerError>> = tokio::spawn(async {
        let listener = TcpListener::bind((Ipv6Addr::LOCALHOST, options.port))
            .await
            .context(TcpBindSnafu {})?;
        loop {
            let conn = listener.accept().await.context(TcpConnectSnafu {})?;
        }
        Ok(())
    });
    todo!()
}
