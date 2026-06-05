use std::marker::PhantomData;

use flate2::{Compress, write::ZlibEncoder};
use tokio::io::AsyncWriteExt;

use crate::out::{Transfer, types::var::int::VarInt};

//maybe still a non-raw version for compresion handling? or just bytes?
//raw presentation of the data
pub enum Packet<T> {
    Uncompressed {
        len: VarInt,
        id: VarInt,
        data: T,
    },
    Compressed {

    },
}

pub enum Compression {
    Uncompressed,
    Compressed {
        //non-negative!!
        // (negative would mean uncompressed)
        threshold: i32,
        level: flate2::Compression,
    }
}

/*
pub struct Compressed<T> {
    data: Box<[u8]>,
    phantom_data: PhantomData<T>,
}

impl<T> Compressed<T> where T: Transfer {
    pub fn new(data: T, level: flate2::Compression) -> Compressed {

    }
}*/

impl<T: Transfer> Packet<T> {
    pub async fn new(id: i32, data: T, compression: Compression) -> Packet<T>
    where T: Transfer {
        let buf=Vec::new();

        pollster::block_on(data.write)

        if let Compression::Compressed { threshold, level } = compression {
            Packet::Compressed {

            }
        } else {
            Packet::Uncompressed { len: , id, data }
        }
    }

    pub fn actual_len(&self) -> usize {
        //self.bytes.len() + 1 // self.packet_id
        todo!()
    }

    pub fn send(&self, stream: tokio::net::TcpStream) {}
}
