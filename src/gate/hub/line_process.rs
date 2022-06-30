use crate::log::Log;

use super::line_head::{Line, LogTag, Status};
use Status::Working;

impl Line {
    pub fn on_error(&mut self) {
        let err = self.stream().take_error().unwrap().unwrap();
        Log::add(format!("{}|{}",self.id(),err), self.kind(), &LogTag::Unexpected);
        self.log(format!("{err}"));
    }


    pub fn on_writable(&mut self) {
        self.set_status(Working);
        self.send();
    }

    

}