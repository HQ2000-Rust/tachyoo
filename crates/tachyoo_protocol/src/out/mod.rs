use tokio::io::{self, AsyncWrite, AsyncWriteExt};

pub(super) mod raw_packet;
pub mod types;

//maybe still just methods????
//trait for types that implement (fallible) conversion into transferable types
/*pub trait IntoTransferable {
    type Transferable: Transfer;
    type Error: std::error::Error;
    /* async */
    fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error>;
}*/

//TODO: define cancel safety requirements
//types that can be infallibly transmitted (written to a tcp stream)
// maybe just something writable, so less mistakes could happen?
#[async_trait::async_trait] //TODO: tmp
pub trait Transfer {
    //async fn write_to_tcp_stream(&self, stream: tokio::net::TcpStream) -> Result<(), io::Error>;

    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()>;
}

pub struct Writable(tokio::net::TcpStream);

impl Writable {
    //TODO: more efficient: not only slices
    async fn write_all(&mut self, src: &[u8]) -> io::Result<()> {
        self.0.write_all(src).await
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
