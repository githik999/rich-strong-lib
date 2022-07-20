use std::io::{Write, ErrorKind};

use mio::net::TcpStream;

use crate::{log::Log, time::Time, head::{LineType, LogTag}};
use Status::{Baby,Dead};

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd)]
pub enum Status {
    Baby,
    Working,
    Error,
    Dead,
}

#[derive(Debug)]
pub struct  Line {
    id:u64,
    partner_id:u64,
    stream:TcpStream,
    status:Status,
    kind:LineType,
    queue:Vec<u8>,
    stage:u8,
    traffic:usize,
    host:String,
    read_close:bool,
    write_close:bool,
    born:u128,
    last_send:u128
}

pub enum LineAge {
    Young,
    Fail,
    Old,
    Defalut,
}

//Get
impl Line {
    
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn partner_id(&self) -> u64 {
        self.partner_id
    }

    pub fn stream(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn kind(&self) -> LineType {
        self.kind
    }

    pub fn queue(&mut self) -> &mut Vec<u8> {
        &mut self.queue
    }

    pub fn stage(&self) -> u8 {
        self.stage
    }

    pub fn traffic(&self) -> usize {
        self.traffic
    }

    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn is_dead(&self) -> bool {
        if self.status == Dead { return true; }
        false
    }

    pub fn born_time(&self) -> u128 {
        self.born
    }

}

//Set
impl Line {
    pub fn new(id:u64,stream:TcpStream,kind:LineType) -> Line {
        Log::new(kind,&id);
        Log::add(format!("{:?}",stream), kind, &id);
        Line{ id,stream,kind,partner_id:0,status:Baby,queue:Vec::new(),stage:0,traffic:0,
            host:String::from(""),read_close:false,write_close:false,born:Time::now(),last_send:0 }
    }

    pub fn set_partner_id(&mut self,id:u64) {
        if self.partner_id == id { return; }
        self.partner_id = id;
        self.log(format!("p|{}",id));
    }

    pub fn set_status(&mut self,v:Status) {
        if v <= self.status { return; }
        self.log(format!("s|{:?}",v));
        self.status = v;
    }

    pub fn set_last_send(&mut self,t:u128) {
        self.last_send = t
    }

    pub fn read_closed(&mut self) {
        self.log(format!("rclose|{}",self.write_close));
        self.read_close = true;
    }

    pub fn write_closed(&mut self) {
        self.log(format!("wclose|{}",self.read_close));
        self.write_close = true;
    }

    pub fn next_stage(&mut self) {
        self.stage = self.stage + 1;
    }

    pub fn add_traffic(&mut self,n:usize) {
        self.traffic += n
    }

    pub fn set_host(&mut self,str:String,tag:u64) {
        Log::add(format!("{}|{}|{}",self.id,str,tag), self.kind, &LogTag::Establish);
        self.log(format!("h|{}",str));
        self.host = str;
    }
    
}

//[Queue]
impl Line {
    pub fn add_queue(&mut self,v:Vec<u8>) {
        if v.len() > 0 {
            self.queue.extend(v.iter());
            //self.log(format!("q|{}",v.len()));
        }
    }
    
    pub fn pour_queue(&mut self) {
        match  self.stream.write(&self.queue) {
            Ok(n) => { 
                self.log(format!("w|{}",n));
                self.add_traffic(n);
                self.set_last_send(Time::now());
                self.shrink_queue(n);
            }
            Err(err) => {
                self.log(format!("write error|{:?}",err));
                if err.kind() != ErrorKind::WouldBlock {
                    self.go_die();
                }
            }
        }
    }
    
    pub fn shrink_queue(&mut self,n:usize) {
        self.queue = self.queue[n..].to_vec();
    }
    
    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }
}