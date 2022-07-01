use std::io::ErrorKind;

use mio::{net::TcpListener, Poll, Token, Interest, event::Event};

use crate::{log::Log, head::{LineType, LogTag}};

use self::hub_head::Hub;

pub mod hub;
mod call_hub;
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
        match TcpListener::bind(addr) {
            Ok(mut listener) => {
                p.registry().register(&mut listener, LISTENER, Interest::READABLE).unwrap();
                let str = format!("gate({:?}) listening on {} waiting for connections...",front_type,addr);
                Log::add(str, front_type,&LogTag::Default);
                Gate{ listener,front_type,hub:Hub::new(LogTag::Default as u64) }
            }

            Err(err) => {
                Log::error(format!("{}|{:?}",addr,err));
                panic!();
            }
        }
    }

    pub fn process(&mut self, event:&Event,p:&Poll) {
        Log::add(format!("{:?}",event), LineType::Defalut, &LogTag::Event);
        match event.token() {
            LISTENER => { self.on_listener_event(event,p); }
            _ => { self.hub.process(event,p); }
        }
    }
}

impl Gate {
    fn on_listener_event(&mut self, event:&Event,p:&Poll) {
        if event.is_error() { panic!(); }

        loop {
            match self.listener.accept() {
                Ok((socket, _)) => {
                    self.hub.new_line(socket,p,self.front_type);
                }
                    
                Err(err) if err.kind() == ErrorKind::WouldBlock => { break; }
                
                _ => { panic!(); }
            }
        }
    }
}