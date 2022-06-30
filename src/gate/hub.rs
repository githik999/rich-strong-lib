use mio::{event::Event, Poll, Token};

use crate::{log::Log, head::{LogTag, LineType}};

use super::hub_head::Hub;

pub mod line_head;
mod fox;
mod line;
mod line_process;

impl Hub {
    pub fn process(&mut self,event:&Event,p:&Poll) {
        let k = &event.token();
        Log::add(format!("{:?}",event), self.get_line(k).kind(), &LogTag::Event);        
        if self.get_line(k).is_dead() {
            self.get_line(k).event_after_die(event);
            return;
        }

        if event.is_error() {
            self.get_mut_line(k).on_error();
            self.dead_pair(k);
            return;
        }
        
        if event.is_writable() {
            self.get_mut_line(k).on_writable();
        }

        if event.is_readable() {
            self.process_read(k,p);
        }

        if event.is_write_closed() {
            self.get_mut_line(k).write_closed();
        }

        if event.is_read_closed() {
            self.get_mut_line(k).read_closed();
            self.dead_pair(k);
        }

    }
}

//[Private]

impl Hub {
    fn process_read(&mut self,k:&Token,p:&Poll) {
        let line = self.get_mut_line(k);
        let pid = line.partner_id();
        let buf =  line.recv();
        
        if buf.len() == 0 {
            return;
        }

        match line.kind() {
            LineType::Fox => {self.process_fox(k,buf,p);}
            //LineType::Http => {self.process_http(k,buf);}
            //LineType::Operator => {self.process_operator(k,buf,p);}
            _ => { self.tunnel(pid, buf); }
        }
    }

    fn process_fox(&mut self,k:&Token,buf:Vec<u8>,p:&Poll) {
        let line = self.get_mut_line(k);
        let fox_id = line.id();
        let mut caller_id = line.partner_id();
        
        match line.fox_data(buf) {
            Some(data) => {
                if caller_id == 0 {
                    self.check(p);
                    caller_id = self.idle_caller();
                    self.get_mut_line_by_id(caller_id).set_partner_id(fox_id);
                    self.get_mut_line(k).set_partner_id(caller_id);
                }
                self.tunnel(caller_id, data);
            }
            _ => { }
        }
    }

    fn tunnel(&mut self,pid:u64,data:Vec<u8>) {
        assert!(pid > 0);
        let line = self.get_mut_line_by_id(pid);
        line.add_queue(data);
        line.send();
    }

    fn check(&mut self,p:&Poll) {
        self.old_check();
        self.dead_check();
        self.health_check(p);
    }
}