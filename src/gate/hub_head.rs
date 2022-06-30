use std::collections::{HashMap, VecDeque};

use mio::{Token, net::TcpStream, Poll, Interest};

use crate::{config::Config, log::Log, head::{LineType, LogTag}};

use super::hub::line_head::Line;

pub struct Hub {
    key:u64,
    m:HashMap<Token,Line>,
    idle_caller:VecDeque<u64>,
    dead:Vec<u64>
}

//[Line]
impl Hub {
    pub fn get_line_by_id(&self,id:u64) -> &Line {
        assert!(id > 0);
        self.get_line(&self.token(id))
    }

    pub fn get_line(&self,token:&Token) -> &Line {
        self.m.get(token).unwrap()
    }

    pub fn get_mut_line_by_id(&mut self,id:u64) -> &mut Line {
        assert!(id > 0);
        self.get_mut_line(&self.token(id))
    }

    pub fn get_mut_line(&mut self,token:&Token) -> &mut Line {
        self.m.get_mut(token).unwrap()
    }

    pub fn kill_line_by_id(&mut self,id:u64) {
        if id > 0 {
            self.kill_line(&self.token(id));
        }
    }
    
    pub fn kill_line(&mut self,k:&Token) {
        self.get_mut_line(k).go_die();
        self.add_dead(k);
    }

}

//Set

impl Hub {
    
    pub fn new(key:u64) -> Hub {
        Hub { key, m:HashMap::new(), idle_caller:VecDeque::new(), dead:Vec::new() }
    }

    pub fn new_line(&mut self,mut stream:TcpStream,p:&Poll,kind:LineType) -> u64 {
        let id = self.next_key();
        let token = Token(id.try_into().unwrap());
        p.registry().register(&mut stream, token, Interest::READABLE | Interest::WRITABLE).unwrap();
        let line = Line::new(id,stream,kind);
        self.m.insert(token, line);
        id
    }

    pub fn kill_both(&mut self,k:&Token) {
        let pid = self.get_line(k).partner_id();
        self.kill_line_by_id(pid);
        self.kill_line(k);
    }
}

//[Dead Manager]
impl Hub {
    pub fn dead_count(&self) -> u8 {
        self.dead.len() as u8
    }

    pub fn dead_check(&mut self) {
        if self.dead_count() > Config::minimum_worker() {
            self.remove_dead();
        }
    }

    pub fn remove_dead(&mut self) {
        loop {
            match self.dead.pop() {
                Some(id) => {
                    let kind = self.get_line_by_id(id).kind();
                    self.m.remove(&self.token(id));
                    Log::add(format!("rm|{}",id), kind, &LogTag::Default);
                }
                None => {break;}
            }
        }
    }
}

//[Caller]
impl Hub {
    pub fn idle_caller_list(&self) -> &VecDeque<u64> {
        &self.idle_caller
    }

    pub fn idle_caller_list_mut(&mut self) -> &mut VecDeque<u64> {
        &mut self.idle_caller
    }

    pub fn idle_caller_count(&self) -> u8 {
        self.idle_caller.len() as u8
    }

    pub fn idle_caller(&mut self) -> u64 {
        self.idle_caller.pop_front().unwrap()
    }

    pub fn add_idle_caller(&mut self,id:u64) {
        self.idle_caller.push_back(id);
    }

}

//[Private]
impl Hub {
    fn next_key(&mut self) -> u64 {
        self.key = self.key + 1;
        self.key
    }

    fn token(&self,id:u64) -> Token {
        Token(id.try_into().unwrap())
    }

    fn add_dead(&mut self,k:&Token) {
        self.dead.push(k.0 as u64);
    }
}