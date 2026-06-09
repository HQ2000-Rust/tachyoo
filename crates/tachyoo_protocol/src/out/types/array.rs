use crate::out::{Buffer, Transfer, types::var::int::VarInt};

//TODO: opt: dense array write (if Transfer gets removed)
pub struct Array<T> {
    data: Box<[T]>,
}

impl<T> Array<T>
where
    T: Transfer,
{
    pub fn new(data: Box<[T]>) -> Array<T> {
        Array { data }
    }

    pub fn from_iter(iter: impl Iterator) -> Array<T> {
        //TODO: adjust cap estimate
        let est_capacity = iter.size_hint().0;
        let data = Vec::with_capacity(est_capacity).into_boxed_slice();

        Array { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Transfer for Array<T>
where
    T: Transfer + Send + Sync,
{
    fn write_bytes(&self, buf: &mut Buffer) {
        for item in &self.data {
            item.write_bytes(buf);
        }
    }
}

pub struct PrefixedArray<T> {
    len: VarInt,
    data: Box<[T]>,
}

impl<T> PrefixedArray<T>
where
    T: Transfer,
{
    pub fn new(data: Box<[T]>) -> PrefixedArray<T> {
        PrefixedArray {
            len: VarInt::new(data.len() as i32),
            data,
        }
    }

    pub fn from_iter(iter: impl Iterator<Item = T>) -> PrefixedArray<T> {
        let data = iter.collect::<Vec<_>>();

        PrefixedArray::new(data.into_boxed_slice())
    }
}

impl<T> Transfer for PrefixedArray<T>
where
    T: Transfer + Send + Sync,
{
    fn write_bytes(&self, buf: &mut Buffer) {
        self.len.write_bytes(buf);

        for item in &self.data {
            item.write_bytes(buf);
        }

        //TODO: debug_assert!(count == expected)
    }
}
