//maybe just type BitSet = PrefixedArray<Long>??? (TODO)

use crate::out::{
    Transfer,
    types::{Long, array::PrefixedArray},
};

pub struct BitSet {
    inner: PrefixedArray<Long>,
}

impl BitSet {
    pub fn from_iter(iter: impl Iterator<Item = Long>) -> BitSet {
        BitSet {
            inner: PrefixedArray::from_iter(iter),
        }
    }

    pub fn new(data: Box<[Long]>) -> BitSet {
        BitSet {
            inner: PrefixedArray::new(data),
        }
    }
}

impl Transfer for BitSet {
    fn write_bytes(&self, buf: &mut crate::out::Buffer) {
        self.inner.write_bytes(buf);
    }
}

//sadly, const generic_const_exprs isn't stable... (FIXME)
/*pub struct FixedBitSet<const N: u32> {
    data: [u8; f64::ceil(N / 8) as usize],
}*/

pub struct FixedBitSet<const BIT_COUNT: usize> {
    data: Box<[u8]>,
}

impl<const BIT_COUNT: usize> FixedBitSet<BIT_COUNT> {
    pub fn zero() -> FixedBitSet<BIT_COUNT> {
        //TODO: truncation
        let byte_count = f64::ceil(BIT_COUNT as f64 / 8.) as usize;
        let data = (0..byte_count)
            .map(|_| 0u8)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        FixedBitSet { data }
    }

    //TODO: determine egornomic way for creation
}

impl<const BIT_COUNT: usize> Transfer for FixedBitSet<BIT_COUNT> {
    fn write_bytes(&self, buf: &mut crate::out::Buffer) {
        buf.write_all(&self.data);
    }
}
