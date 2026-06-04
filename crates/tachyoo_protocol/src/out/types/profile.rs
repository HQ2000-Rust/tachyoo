use tokio::io;

use crate::out::{Transfer, Writable, types::{UUID, array::PrefixedArray, option::prefixed::PrefixedOptional, string::String_, var::int::VarInt}};



pub struct ResolvableProfile {
    profile_kind: VarInt,
    unpack: Unpack,
}

pub enum Unpack {
    Partial {
        username: PrefixedOptional<String_<16>>,
        uuid: PrefixedOptional<UUID>,
        //array: max 16??
        properties: PrefixedArray<(String_<64>, String_<32767>, PrefixedOptional<String_<1024>>)>
    },
    Complete(GameProfile)
}

#[async_trait::async_trait]
impl Transfer for Unpack {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        match self {
            Unpack::Complete(profile) => {profile.write_data(writeable).await?; },
            Unpack::Partial { username, uuid, properties } => {
                username.write_data(writeable).await?;
                uuid.write_data(writeable).await?;
                properties.write_data(writeable).await?;
            }
        }
    }
}