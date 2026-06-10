pub mod state;
pub mod types;
pub mod var;

use bytes::{BufMut, BytesMut};
use tokio::io::{self, AsyncReadExt};

pub struct ProtocolParser {
    buffer: BytesMut,
}

impl ProtocolParser {
    //TODO: hand written future? (also, optimize in general!)
    pub async fn read<R: AsyncReadExt + Unpin>(&mut self, w: &mut R) -> io::Result<()> {
        let mut buf=Vec::new();
        w.read_to_end(&mut buf).await?;
        self.buffer.extend_from_slice(buf.as_slice());
        Ok(())
        
    }
}