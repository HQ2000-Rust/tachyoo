use either::Either;

use crate::out::{Buffer, Transfer};

impl<L, R> Transfer for Either<L, R>
where
    L: Transfer + Send + Sync,
    R: Transfer + Send + Sync,
{
    fn write_bytes(&self, buf: &mut Buffer) {
        match self {
            Either::Left(left) => left.write_bytes(buf),
            Either::Right(right) => right.write_bytes(buf),
        };
    }
}
