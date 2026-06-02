
use std::convert::Infallible;

use crate::out::{IntoTransferable, Transfer};

use tokio::io::{self, AsyncWriteExt};

impl<T, I> IntoTransferable for T where T: Iterator<Item=I>, I: Transfer {
    type Transferable = Array<T>;
    type Error = Infallible;

    fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error> {
        Ok(Array(self))
    }
}

pub struct Array<T>(T);

#[async_trait::async_trait]
impl<'a, T> Transfer for Array<'a, T> where T: Transfer {
    async fn write_to_tcp_stream(&self, stream: tokio::net::TcpStream) -> io::Result<()> {
        stream.write_all(self.0.)
    }
}