use mio::{net::TcpListener, Poll, Token, Interest};

use crate::log::Log;

use self::{hub::line_head::{LineType, LogTag}, hub_head::Hub};

pub mod hub;
pub mod hub_head;

const LISTENER: Token = Token(0);

pub struct Gate {
    listener:TcpListener,
    front_type:LineType,
    pub hub:Hub
}

impl Gate {
    pub fn new(addr:String,front_type:LineType,p:&Poll) -> Gate {
        let addr = addr.parse().unwrap();
        let mut listener = TcpListener::bind(addr).unwrap();
        p.registry().register(&mut listener, LISTENER, Interest::READABLE).unwrap();
        let str = format!("gate({:?}) listening on {} waiting for connections...",front_type,addr);
        Log::add(str, front_type,&LogTag::Default);
        Gate{ listener,front_type,hub:Hub::new(LogTag::Default as u64) }
    }
}