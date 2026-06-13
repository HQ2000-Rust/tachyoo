use std::{io, net::SocketAddr};

use snafu::prelude::*;

//fatal server error
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ServerError {
    #[snafu(display("Failed to create a tokio runtime"))]
    RuntimeCreation {
        source: io::Error,
    },
    #[snafu(display("Failed to bind to the tcp socket {}", socket_addr))]
    TcpBind {
        source: io::Error,
        //socket_addr: SocketAddr,
    },
    #[snafu(display("Failed to establish a new tcp connection"))]
    TcpConnect {
        source: io::Error,
    },
    InvalidSocketAddress {
        source: io::Error,
        port: u16,
    },
}
