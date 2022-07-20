use std::collections::VecDeque;

use mio::{Poll, net::TcpStream};

use crate::{time::Time, log::Log, config::Config, head::{LineType, LogTag}};

use super::{hub_head::Hub, hub::line_head::{LineAge,Status::{Baby,Error,Dead}}};

///Caller Hub

impl Hub {
    pub fn init_callers(&mut self,p:&Poll) {
        let n = Config::minimum_worker();
        assert!(n > 0);
        self.add_caller(n,p);
    }

    pub fn old_check(&mut self) {
        let limit = (Config::minimum_worker()/2).into();
        let mut error = Vec::new();
        let mut fail = Vec::new();
        let mut old = Vec::new();
        let mut young = VecDeque::new();

        let t = Time::now();
        
        for id in self.idle_caller_list() {
            let id = *id;
            match self.age(id, t) {
                LineAge::Young => { young.push_back(id); }
                LineAge::Fail => { fail.push(id); }
                LineAge::Error => { error.push(id); }
                LineAge::Old => { old.push(id); }
                _ => {}
            }
        }

        Log::add(format!("young:{}|fail:{}|error:{}|old:{}|dead:{}",young.len(),error.len(),fail.len(),old.len(),self.dead_count()), LineType::Caller, &LogTag::Default);
        
        if error.len() > limit {
            Log::heart_beat("tcp errors with proxy server".to_string());
        }

        if fail.len() > limit {
            Log::heart_beat("connection to proxy server is bad".to_string());
        }

        for id in error {
            self.kill_line_by_id(id);
        }

        for id in fail {
            self.kill_line_by_id(id);
        }

        for id in old {
            self.kill_line_by_id(id);
        }

        self.idle_caller_list_mut().clone_from(&young);
    }

    pub fn health_check(&mut self,p:&Poll) {
        let need = Config::minimum_worker();
        let have:u8 = self.idle_caller_count();
        Log::add(format!("have:{}|need:{}",have,need), LineType::Caller, &LogTag::Default);
        if have < need {
            self.add_caller(need, p);
        }
    }


}

//[Private]
impl Hub {
    fn age(&self,id:u64,now:u128) -> LineAge {
        let v = self.get_line_by_id(id);
        if v.kind() != LineType::Caller { return LineAge::Defalut; }
        if v.status() == Error { return LineAge::Error; }
        if v.status() == Dead { return LineAge::Defalut; }
        let age = now - v.born_time();
        
        if v.status() == Baby && age > 5*1000 { 
            return LineAge::Fail;
        }

        if age > 3*60*1000 { 
            return LineAge::Old;
        }
        LineAge::Young
    }

    fn add_caller(&mut self,n:u8,p:&Poll) {
        for _ in 0..n {
            self.add_one_caller(p);
        }
    }

    fn add_one_caller(&mut self,p:&Poll) {
        let addr = Config::proxy_server_addr().parse().unwrap();
        let stream = TcpStream::connect(addr).unwrap();
        let id = self.new_line(stream,p,LineType::Caller);
        self.add_idle_caller(id);
    }

}