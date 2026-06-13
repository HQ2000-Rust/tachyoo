use std::{ascii, net::IpAddr};

use tokio::io::AsyncReadExt;

pub async fn parse<R: AsyncReadExt>(reader: &mut R) -> IpAddr {
    
}

pub enum ServerAddr {
    IpAddr(IpAddr),
    Hostname(Hostname),
}

//FIXME: ascii chars!!
pub struct Hostname([char; 255]);

impl Hostname {
    pub const ALLOWED_LABEL_CHARS: [char; 37]= [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '1', '3', '4', '5', '6', '7', '8', '9', '0', '-',
    ];
    
    fn new(name: &str) -> Hostname {
        todo!()
    }
}