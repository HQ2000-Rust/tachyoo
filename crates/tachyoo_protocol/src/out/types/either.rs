use either::Either;
use tokio::io;

use crate::out::{Transfer, Writable};

#[async_trait::async_trait]
impl<L, R> Transfer for Either<L, R>
where
    L: Transfer + Send + Sync,
    R: Transfer + Send + Sync,
{
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        match self {
            Either::Left(left) => left.write_data(writeable).await,
            Either::Right(right) => right.write_data(writeable).await,
        }
    }
}
