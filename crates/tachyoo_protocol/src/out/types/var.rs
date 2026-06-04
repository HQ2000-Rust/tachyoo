////// INT
pub mod int {

    use tokio::io;

    use crate::out::{Transfer, Writable};

    // #[repr(transparent)]
    pub struct VarInt {
        data: Box<[u8]>,
    }

    impl VarInt {
        pub fn new(num: i32) -> VarInt {
            let capacity_approx = 0;
            let mut bytes = Vec::with_capacity(capacity_approx);

            match leb128::write::signed(&mut bytes, num as i64) {
                Ok(_bytes_written) => VarInt {
                    data: bytes.into_boxed_slice(),
                },
                Err(_) => unreachable!("Vec's Write::write() impl never returns an error"),
            }
        }
    }

    #[async_trait::async_trait]
    impl Transfer for VarInt {
        async fn write_data(&self, writable: &mut Writable) -> io::Result<()> {
            writable.write_all(&self.data).await
        }
    }
}

pub mod long {
    use tokio::io;

    use crate::out::{Transfer, Writable};

    #[repr(transparent)]
    pub struct VarLong {
        data: Box<[u8]>,
    }

    impl VarLong {
        pub fn new(num: i64) -> VarLong {
            let capacity_approx = 0;
            let mut bytes = Vec::with_capacity(capacity_approx);

            match leb128::write::signed(&mut bytes, num) {
                Ok(_bytes_written) => VarLong {
                    data: bytes.into_boxed_slice(),
                },
                Err(_) => unreachable!("Vec's Write::write() impl never returns an error"),
            }
        }
    }

    #[async_trait::async_trait]
    impl Transfer for VarLong {
        async fn write_data(&self, writable: &mut Writable) -> Result<(), io::Error> {
            writable.write_all(&self.data).await
        }
    }
}
