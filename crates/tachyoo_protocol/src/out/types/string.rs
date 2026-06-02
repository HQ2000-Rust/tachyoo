use crate::out::types::var::int::{VarInt, var_int};

//better name!
pub struct String_ {
    //max: 32767
    len: VarInt,
    data: Box<str>,
}

impl String_ {
    const MAX_LENGTH: usize = 32767;
}

//TODO: maybe impl Into<Box<str>>??
pub fn string(string: String) -> Option<String_> {
    if string.len() > String_::MAX_LENGTH {
        None
    } else {
        Some(String_ {
            len: var_int(string.len() as i32),
            data: string.into_boxed_str(),
        })
    }
}
