use crate::{log::Log, head::LogTag};

use super::line_head::{Line,Status};
use Status::{Working,Error};

impl Line {
    pub fn on_error(&mut self) {
        self.set_status(Error);
        let err = self.stream().take_error().unwrap().unwrap();
        let err = format!("stream_error|{}",err);
        Log::add(format!("{}|{}",self.id(),err), self.kind(), &LogTag::Unexpected);
        self.log(format!("{err}"));
    }


    pub fn on_writable(&mut self) {
        self.set_status(Working);
        self.send();
    }

    

}