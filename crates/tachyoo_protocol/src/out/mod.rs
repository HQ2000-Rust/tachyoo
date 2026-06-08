
pub(super) mod packet;
pub mod types;

//types that can be infallibly transmitted
pub trait Transfer {
    fn write_bytes(&self, buffer: &mut [u8]);
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
