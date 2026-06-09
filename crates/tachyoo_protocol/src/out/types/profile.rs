use crate::out::{
    Buffer, Transfer,
    types::{
        UUID,
        array::PrefixedArray,
        identifier::Identifier,
        option::{prefixed::PrefixedOptional, unprefixed::Optional},
        string::McString,
        var::int::VarInt,
    },
};

pub struct ResolvableProfile {
    profile_kind: VarInt,
    unpack: Unpack,
    body: Optional<Identifier>,
    cape: Optional<Identifier>,
    elytra: Optional<Identifier>,
    //wide=0, slim=1
    model: Optional<VarInt>,
}

enum Unpack {
    Partial {
        username: PrefixedOptional<McString<16>>,
        uuid: PrefixedOptional<UUID>,
        //max 16 (TODO)
        props: PrefixedArray<GameProfileProp>,
    },
    Complete(GameProfile),
}

impl Transfer for Unpack {
    fn write_bytes(&self, buf: &mut Buffer) {
        match self {
            Unpack::Complete(profile) => {
                profile.write_bytes(buf);
            }
            Unpack::Partial {
                username,
                uuid,
                props,
            } => {
                username.write_bytes(buf);
                uuid.write_bytes(buf);
                props.write_bytes(buf);
            }
        }
    }
}

impl ResolvableProfile {
    //TODO: as needed
    //pub fn new
}

impl Transfer for ResolvableProfile {
    fn write_bytes(&self, buf: &mut Buffer) {
        self.profile_kind.write_bytes(buf);
        self.unpack.write_bytes(buf);
        self.body.write_bytes(buf);
        self.cape.write_bytes(buf);
        self.elytra.write_bytes(buf);
        self.model.write_bytes(buf);
    }
}

pub struct GameProfile {
    uuid: UUID,
    username: McString<16>,
    //max 16 (TODO again)
    props: PrefixedArray<GameProfileProp>,
}

struct GameProfileProp {
    name: McString<64>,
    val: McString<32767>,
    signature: PrefixedOptional<McString<1024>>,
}

impl Transfer for GameProfileProp {
    fn write_bytes(&self, buf: &mut Buffer) {
        self.name.write_bytes(buf);
        self.val.write_bytes(buf);
        self.signature.write_bytes(buf);
    }
}

impl Transfer for GameProfile {
    fn write_bytes(&self, buf: &mut Buffer) {
        self.uuid.write_bytes(buf);
        self.username.write_bytes(buf);
        self.props.write_bytes(buf);
    }
}
