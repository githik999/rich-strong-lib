use std::time::Duration;

use mio::{Poll, Events};

use crate::{gate::Gate, log::Log, head::LineType};

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
            self.p.poll(&mut self.events, Some(Duration::from_millis(1000))).unwrap();
            for event in self.events.iter() {
                self.gate.process(event,&self.p);
            }
            self.heart_beat();
        }
    }

    pub fn init(&mut self) {
        self.gate.hub.init_callers(&self.p);
    }

}

impl Server {
    fn heart_beat(&mut self) {
        self.gate.hub.check(&self.p);
    }
}