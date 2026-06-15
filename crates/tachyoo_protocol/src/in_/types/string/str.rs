use crate::{in_::types::string::McStringError, util::string::ABSOLUTE_MAX_LEN};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct McStr<'a, const MAX_LENGTH: u16> {
    data: &'a str,
}

impl<'a, const MAX_LEN: u16> McStr<'a, MAX_LEN> {
    //TODO: fix
    const __ASSERTION: () = assert!(MAX_LEN <= ABSOLUTE_MAX_LEN);

    fn new(bytes: &[u8]) -> Result<McStr<'_, MAX_LEN>, McStringError> {
        let str = str::from_utf8(bytes).map_err(McStringError::InvalidUtf8)?;

        let (valid, len) = crate::util::string::is_valid_and_len::<MAX_LEN>(str);

        if valid {
            Ok(McStr { data: str })
        } else {
            Err(McStringError::TooLong { len })
        }
    }

    pub fn as_str(&self) -> &str {
        self.data
    }

    pub fn len(&self) -> u16 {
        self.data.len() as u16
    }

    //needs to be a valid McStr
    pub fn from_str_unchecked(str: &str) -> McStr<'_, MAX_LEN> {
        McStr { data: str }
    }
}

impl<const MAX_LEN: u16> AsRef<str> for McStr<'_, MAX_LEN> {
    fn as_ref(&self) -> &str {
        self.data
    }
}