pub mod unprefixed {
    use tokio::io;

    use crate::out::{Transfer, Writable};

    #[derive()]
    pub struct Optional<T>(Option<T>);

    impl<T> Optional<T>
    where
        T: Transfer,
    {
        pub fn some(transferable: T) -> Optional<T> {
            Optional(Some(transferable))
        }

        pub fn none() -> Optional<T> {
            Optional(Option::None)
        }

        pub fn new(transferable_opt: Option<T>) -> Optional<T> {
            Optional(transferable_opt)
        }
    }

    /*
    impl<T> IntoTransferable for Option<T>
    where
        T: IntoTransferable,
        <T as IntoTransferable>::Transferable: Send + Sync,
    {
        type Transferable = Optional<<T as IntoTransferable>::Transferable>;
        type Error = <T as IntoTransferable>::Error;

        fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error> {
            Ok(Optional(
                self.map(|t| t.try_into_transferable()).transpose()?,
            ))
        }
    }*/

    #[async_trait::async_trait]
    impl<T> Transfer for Optional<T>
    where
        T: Transfer + Send + Sync,
    {
        async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
            match self.0 {
                Some(ref t) => t.write_data(writeable).await,
                None => Ok(()),
            }
        }
    }
}

pub mod prefixed {
    use tokio::io;

    use crate::out::{Transfer, Writable};

    #[derive()]
    pub struct PrefixedOptional<T>(Option<T>);

    /*impl<T> IntoTransferable for PrefixedOptional<T>
    where
        T: IntoTransferable,
        <T as IntoTransferable>::Transferable: Send + Sync,
    {
        type Transferable = PrefixedOptional<<T as IntoTransferable>::Transferable>;
        type Error = <T as IntoTransferable>::Error;

        fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error> {
            Ok(PrefixedOptional(
                self.map(|t| t.try_into_transferable()).transpose()?,

            ))
        }
    }*/

    impl<T> PrefixedOptional<T>
    where
        T: Transfer,
    {
        pub fn some(transferable: T) -> PrefixedOptional<T> {
            PrefixedOptional(Some(transferable))
        }

        pub fn none() -> PrefixedOptional<T> {
            PrefixedOptional(Option::None)
        }

        pub fn new(transferable_opt: Option<T>) -> PrefixedOptional<T> {
            PrefixedOptional(transferable_opt)
        }
    }

    #[async_trait::async_trait]
    impl<T> Transfer for PrefixedOptional<T>
    where
        T: Transfer + Send + Sync,
    {
        async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
            writeable.write_all(&[self.0.is_some() as u8]).await?;

            match self.0 {
                Some(ref t) => t.write_data(writeable).await,
                None => Ok(()),
            }
        }
    }
}
