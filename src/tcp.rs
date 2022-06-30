use std::net::{ToSocketAddrs, SocketAddr};

use mio::net::TcpStream;

use crate::log::Log;

pub struct Tcp;

impl Tcp {
    
    pub fn dns(host:&str) -> Option<SocketAddr> {
        match host.to_socket_addrs() {
            Ok(mut it) => {
                return Some(it.next().unwrap());
            }
            Err(err) => {
                Log::error(format!("dns fail|{}|{}",host,err));
                None
            }
        }
    }

    pub fn connect(host:&str) -> Option<TcpStream> {
        match Tcp::dns(host) {
            Some(addr) => {
                match TcpStream::connect(addr) {
                    Ok(socket) => {
                        return Some(socket)
                    }
                    Err(err) => {
                        Log::error(format!("connect fail|{}|{}",host,err));
                    }
                }
            }
            None => {}
        }
        None
    }
}