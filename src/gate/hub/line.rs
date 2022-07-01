use std::{net::Shutdown, io::{Read, ErrorKind}};

use mio::event::Event;

use crate::{log::Log, time::Time, head::LogTag};

use super::line_head::{Line,Status};
use Status::{Working,Dead};

impl Line {
    pub fn go_die(&mut self) {
        self.clear_queue();
        self.shutdown_stream();
        self.set_partner_id(0);
        self.set_status(Dead);
        let t = Time::now() - self.born_time();
        self.log(format!("die|{}ms|{}bytes",t,self.traffic()));
        Log::add(format!("{}|{}|{}ms|{}bytes",self.id(),self.host(),t,self.traffic()), self.kind(), &LogTag::GoodBye);
    }
    
    pub fn event_after_die(&self,e:&Event) {
        self.log(format!("e|{:?}",e));
    }

    pub fn log(&self,str:String) {
        Log::add(str,self.kind(),&self.id());
    }

    pub fn recv(&mut self) -> Vec<u8> {
        let mut data:Vec<u8> = Vec::new();
        let mut buf = [0;8192];
        loop {
            match self.stream().read(&mut buf) {
                Ok(n) => {
                    if n > 0 {
                        data.extend_from_slice(&buf[..n]);
                    } else {
                        break;
                    }
                }
                
                Err(err) => {
                    if err.kind() != ErrorKind::WouldBlock { 
                        let str = format!("read_error {:?}",err);
                        self.log(str);
                    }
                    break;
                }
            }
        }

        let str = format!("r|{}",data.len());
        self.log(str);
        data
    }

    pub fn send(&mut self) {
        if self.status() != Working { return ; }

        loop {
            if self.queue().len() > 0 {
                self.pour_queue();
            } else {
                break;
            }
        }
    }
}


//[Private]
impl Line {
    fn shutdown_stream(&mut self) {
        if self.status() != Working { return; }
        match self.stream().shutdown(Shutdown::Both) {
            Err(err) => {
                self.log(format!("shutdown_stream fail|{}",err));
            }
            _ => {}
        }
    }
}