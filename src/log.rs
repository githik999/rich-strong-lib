use core::fmt::Debug;
use std::{fs::{File, self}, io::Write};

use enum_iterator::all;

use crate::{config::Config, time::Time, head::{LineType, LogTag}};
pub struct Log;

impl Log {
    pub fn create_dir(kind:LineType) {
        if Config::log_off() { return; }
        let path = format!("log/{:?}",kind);
        fs::create_dir(path).unwrap();
        for x in all::<LogTag>() {
            Log::new(kind, &x);
        }
    }

    pub fn create_file(path:String) {
        if Config::log_off() { return; }
        File::create(path).unwrap();
    }

    pub fn new<T:Debug>(kind:LineType,name:&T) {
        let path = Log::get_path(kind,name);
        Log::create_file(path);
    }
    
    pub fn add<T:Debug>(str:String,kind:LineType,name:&T) {
        if Config::log_off() { return; }
        let path = Log::get_path(kind,name);
        let s = format!("{}|{}\n",Time::now(),str);
        let mut f = File::options().append(true).open(path).unwrap();
        f.write(s.as_bytes()).unwrap();
    }
}

impl Log {
    fn get_path<T: Debug>(kind:LineType,name:&T) -> String {
        format!("log/{:?}/{:?}.log",kind,name)
    }
}