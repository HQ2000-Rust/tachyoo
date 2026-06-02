use tokio::io::{self, AsyncWriteExt};

pub(super) mod raw_packet;
pub mod types;

//types that are reprs for transmission
#[async_trait::async_trait] //TODO: tmp
pub trait Transfer: AsRef<[u8]> {
    async fn write_to_tcp_stream(&self, mut stream: tokio::net::TcpStream) -> Result<(), io::Error> {
        //for cancel safety (FIXME)
        stream.write_all_buf(&mut self.as_ref()).await?;
        Ok(())
    }
}


/*
macro_rules! impl_transfer {
($N:expr, $($T:ident),*) => {
        impl<$($T),*> Transfer for ($($T,)*)
       where T: Transfer,
        {
           // type Tup = ($(Foo<$T>,)*);

            fn write_to_tcp_stream(&self, mut stream: tokio::net::TcpStream) -> Result< {
                
                tokio::try_join!(
                    stream.write_all_buf(&mut self.$N.as_ref()),
            
                )
            }
        }
    };
}

variadics_please::all_tuples!(impl_transfer, 1, 15, T);
*/