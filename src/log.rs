use core::fmt::Debug;
use std::{fs::{File, self}, io::Write};

use enum_iterator::all;

use crate::{config::Config, time::Time, head::{LineType, LogTag}};
pub struct Log;

impl Log {
    pub fn create_log_dir() {
        let path = "log";
        match fs::remove_dir_all(path) {
            _ => { fs::create_dir(path).unwrap(); }
        }
        File::create(Log::event_file()).unwrap();
        File::create(Log::heart_beat_file()).unwrap();
    }

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

    pub fn heart_beat(str:String) {
        let s = format!("{}|{}\n",Time::now(),str);
        let mut f = File::options().append(true).open(Log::heart_beat_file()).unwrap();
        f.write(s.as_bytes()).unwrap();
    }

    pub fn event(str:String) {
        let s = format!("{}|{}\n",Time::now(),str);
        let mut f = File::options().append(true).open(Log::event_file()).unwrap();
        f.write(s.as_bytes()).unwrap();
    }
    
    
}

impl Log {
    fn get_path<T: Debug>(kind:LineType,name:&T) -> String {
        format!("log/{:?}/{:?}.log",kind,name)
    }

    fn heart_beat_file() -> String {
        String::from("log/heart_beat.log")
    }

    fn event_file() -> String {
        String::from("log/event.log")
    }
}
