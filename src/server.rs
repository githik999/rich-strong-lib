use mio::{Poll, Events};

use crate::{gate::{Gate, hub::line_head::LineType}, log::Log};

pub struct Server {
    p:Poll,
    events:Events,
    gate:Gate
}

impl Server {
    pub fn new(addr:String,kind:LineType) -> Server {
        let p = Poll::new().unwrap();
        let events = Events::with_capacity(u8::MAX.into());
        Log::create_dir(kind);
        Log::create_dir(enum_iterator::next(&kind).unwrap());
        let gate = Gate::new(addr,kind,&p);
        Server { p, events , gate }
    }

    pub fn start(&mut self) {
        loop {
            self.p.poll(&mut self.events, None).unwrap();
            for event in self.events.iter() {
                self.gate.process(event,&self.p);
            }
            //self.gate.hub.update_working_count();
        }
    }
}