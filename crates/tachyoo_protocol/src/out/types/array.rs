use crate::out::{Transfer, Writable, types::var::int::VarInt};

use tokio::io::{self, AsyncWriteExt};

/*
impl<T, I> IntoTransferable for T where T: Iterator<Item=I>, I: Transfer {
    type Transferable = Array<T>;
    type Error = Infallible;

    fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error> {
        Ok(Array(self))
    }
}
*/

//TODO: opt: dense array write (if Transfer gets removed)
pub struct Array<T> {
    iter: T,
}

impl<T> Array<T>
where
    T: IntoIterator,
    <T as IntoIterator>::Item: Transfer,
{
    pub fn new(iter: T) -> Array<T> {
        Array { iter }
    }
}

#[async_trait::async_trait]
impl<T> Transfer for Array<T>
where
    T: Iterator + Send + Sync + Clone,
    <T as Iterator>::Item: Transfer + Send + Sync,
{
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        for item in self.iter.clone() {
            item.write_data(writeable).await?;
        }
        Ok(())
    }
}

pub struct PrefixedArray<T> {
    iter: T,
}

impl<T> PrefixedArray<T>
where
    T: ExactSizeIterator,
    <T as Iterator>::Item: Transfer,
{
    pub fn new(iter: T) -> PrefixedArray<T> {
        PrefixedArray { iter }
    }

    pub fn len(&self) -> usize {
        self.iter.len()
    }
}

#[async_trait::async_trait]
impl<T> Transfer for PrefixedArray<T>
where
    T: ExactSizeIterator + Send + Sync + Clone,
    <T as Iterator>::Item: Transfer + Send + Sync,
{
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        VarInt::new(self.iter.len() as i32)
            .write_data(writeable)
            .await?;

        for item in self.iter.clone() {
            item.write_data(writeable).await?;
        }
        
        //TODO: debug_assert!(count == expected)
        Ok(())
    }
}
