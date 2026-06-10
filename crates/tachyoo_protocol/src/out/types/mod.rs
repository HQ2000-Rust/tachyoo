// everything is big endian!!
// (TODO)

use std::ops::Deref;

use crate::out::{Buffer, Transfer, types::array::Array};

pub mod array;
pub mod bitset;
pub mod either;
pub mod entity_metadata;
pub mod fixed_point;
pub mod identifier;
pub mod light_data;
pub mod option;
pub mod pos;
pub mod profile;
pub mod recipe_display;
pub mod registry_data;
pub mod registry_refs;
pub mod slot_display;
pub mod string;
pub mod teleport_flags;
pub mod var;

pub type Boolean = bool;

impl Transfer for Boolean {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&(*self as u8).to_be_bytes());
    }
}

pub type Byte = i8;

impl Transfer for Byte {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&self.to_be_bytes());
    }
}

pub type UByte = u8;

impl Transfer for UByte {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&self.to_be_bytes());
    }
}

pub type Short = i16;

impl Transfer for Short {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&self.to_be_bytes());
    }
}

pub type UShort = u16;

impl Transfer for UShort {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&self.to_be_bytes());
    }
}
pub type Int = i32;

impl Transfer for Int {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&self.to_be_bytes());
    }
}

pub type Long = i64;

impl Transfer for Long {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&self.to_be_bytes());
    }
}

pub type Float = f32;

impl Transfer for Float {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&self.to_be_bytes());
    }
}

pub type Double = f64;

impl Transfer for Double {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&self.to_be_bytes());
    }
}

//TODO
pub type UUID = u128;

impl Transfer for UUID {
    fn write_bytes(&self, buf: &mut Buffer) {
        buf.write_all(&self.to_be_bytes());
    }
}

type ByteArray = Array<u8>;

type Angle = u8;
