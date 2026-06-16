pub mod int {
    pub mod signed {
        use std::io;

        use leb128::read::Error;
        use tokio::io::AsyncReadExt;
        use tokio_util::io::SyncIoBridge;

        use crate::in_::types::Int;

        //TODO: own implementaion!!!
        // (this is very inefficient, I think)
        pub async fn parse_var_int<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Int, leb128::read::Error> {
            let mut bridge = SyncIoBridge::new(reader);
            leb128::read::signed(&mut bridge).map(|val| val as Int)

        }
    }
    pub mod unsigned {
        use std::io;

        use leb128::read::Error;
        use tokio::io::AsyncReadExt;
        use tokio_util::io::SyncIoBridge;

        use crate::in_::types::UInt;

        //TODO: own implementaion!!!
        // (this is very inefficient, I think)
        pub async fn parse<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<UInt, leb128::read::Error> {
            let mut bridge = SyncIoBridge::new(reader);
            leb128::read::unsigned(&mut bridge).map(|val| val as UInt)

        }
    }
}

pub mod long {
    use std::io;

    use leb128::read::Error;
    use tokio::io::AsyncReadExt;
    use tokio_util::io::SyncIoBridge;

    use crate::in_::types::{Long};

    //TODO: own implementaion!!!
    // (this is very inefficient, I think)
    pub async fn parse<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Long, leb128::read::Error> {
        let mut bridge = SyncIoBridge::new(reader);
        leb128::read::signed(&mut bridge) 
        
    }
    pub mod unsigned {
        use tokio::io::AsyncReadExt;
        use tokio_util::io::SyncIoBridge;

        use crate::in_::types::ULong;

        //TODO: own implementaion!!!
        // (this is very inefficient, I think)
        pub async fn parse<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<ULong, leb128::read::Error> {
            let mut bridge = SyncIoBridge::new(reader);
            leb128::read::unsigned(&mut bridge) 
        }
    }
}