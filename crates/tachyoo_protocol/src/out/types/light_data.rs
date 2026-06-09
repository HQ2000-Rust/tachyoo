use crate::out::{
    Buffer, Transfer,
    types::{Byte, array::PrefixedArray, bitset::BitSet},
};

//see the wiki
// TODO: more efficient repr later
pub struct LightData {
    sky_light_mask: BitSet,
    block_light_mask: BitSet,
    empty_sky_light_mask: BitSet,
    empty_block_light_mask: BitSet,
    sky_light_arrays: PrefixedArray<PrefixedArray<Byte>>,
    block_light_arrays: PrefixedArray<PrefixedArray<Byte>>,
}

impl LightData {
    pub fn new(
        sky_light_mask: BitSet,
        block_light_mask: BitSet,
        empty_sky_light_mask: BitSet,
        empty_block_light_mask: BitSet,
        sky_light_arrays: PrefixedArray<PrefixedArray<Byte>>,
        block_light_arrays: PrefixedArray<PrefixedArray<Byte>>,
    ) -> LightData {
        LightData {
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_arrays,
            block_light_arrays,
        }
    }
}

impl Transfer for LightData {
    fn write_bytes(&self, buf: &mut Buffer) {
        self.sky_light_mask.write_bytes(buf);
        self.block_light_mask.write_bytes(buf);
        self.empty_sky_light_mask.write_bytes(buf);
        self.empty_block_light_mask.write_bytes(buf);
        self.sky_light_arrays.write_bytes(buf);
        self.block_light_arrays.write_bytes(buf);
    }
}
