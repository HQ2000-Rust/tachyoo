pub mod unprefixed {

    use crate::out::{Buffer, Transfer};

    #[derive()]
    pub struct Optional<T>(Option<T>);

    impl<T> Optional<T> {
        pub fn some(val: T) -> Optional<T> {
            Optional(Some(val))
        }

        pub fn none() -> Optional<T> {
            Optional(Option::None)
        }

        pub fn new(val_opt: Option<T>) -> Optional<T> {
            Optional(val_opt)
        }
    }

    impl<T> Transfer for Optional<T>
    where
        T: Transfer + Send + Sync,
    {
        fn write_bytes(&self, buf: &mut Buffer) {
            if let Some(ref t) = self.0 {
                t.write_bytes(buf);
            }
        }
    }
}

pub mod prefixed {

    use crate::out::{Buffer, Transfer};

    #[derive()]
    pub struct PrefixedOptional<T>(Option<T>);

    impl<T> PrefixedOptional<T> {
        pub fn some(val: T) -> PrefixedOptional<T> {
            PrefixedOptional(Some(val))
        }

        pub fn none() -> PrefixedOptional<T> {
            PrefixedOptional(Option::None)
        }

        pub fn new(val_opt: Option<T>) -> PrefixedOptional<T> {
            PrefixedOptional(val_opt)
        }
    }

    impl<T> Transfer for PrefixedOptional<T>
    where
        T: Transfer + Send + Sync,
    {
        fn write_bytes(&self, buf: &mut Buffer) {
            buf.write_all(&[self.0.is_some() as u8]);

            if let Some(ref t) = self.0 {
                t.write_bytes(buf);
            }
        }
    }
}
