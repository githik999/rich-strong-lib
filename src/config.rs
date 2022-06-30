use std::{fs::File, io::Write};

use backtrace::Backtrace;
use configparser::ini::Ini;

use crate::log::Log;

pub struct Config;

static mut WRITE_LOG: bool = false;
static mut MININUM_WORKER: u8 = 0;
static mut WORKING_CALLER: u8 = 0;
static mut PROXY_SERVER_ADDR : String = String::new();

impl Config {
    pub fn init() {
        Config::set_panic_hook();
        let r = Config::load();
        let n:u8 = r.getuint("other","minimum_worker").unwrap().unwrap() as u8;
        let addr = r.get("server","addr").unwrap();
        let write_log = r.getbool("other", "write_log").unwrap().unwrap();
        
        if write_log {
            Config::turn_on()
        }
        Config::set_minimum_worker(n);
        Config::set_proxy_server_addr(addr);
    }

    pub fn get_listen_addr() -> (String,String) {
        let r = Config::load();
        let app = r.get("listen","app").unwrap();
        let http = r.get("listen","http").unwrap();
        (app,http)
    }

    
}

//[Private]
impl Config {
    fn load() -> Ini {
        let mut config = Ini::new();
        config.load("theshy.ini".to_string()).unwrap();
        config
    }

    fn set_panic_hook() {
        std::panic::set_hook(Box::new(|_| {
            let bt = Backtrace::new();
            let mut f = File::options().append(true).open(Log::panic_file()).unwrap();
            f.write(format!("{:?}",bt).as_bytes()).unwrap();
        }));
    }
}

impl Config {
    pub fn turn_on() {
        unsafe{ WRITE_LOG = true }
    }

    pub fn log_off() -> bool {
        unsafe{ if !WRITE_LOG { return true; } }
        false
    }

    pub fn working_caller_count() -> u8 {
        unsafe{ WORKING_CALLER }
    }

    pub fn set_working_caller_count(n:u8) {
        unsafe{ WORKING_CALLER = n; }
    }

    pub fn minimum_worker() -> u8 {
        unsafe{ MININUM_WORKER }
    }

    pub fn set_minimum_worker(n:u8) {
        assert!(n < u8::MAX/4);
        unsafe{ MININUM_WORKER = n; }
    }

    pub fn proxy_server_addr() -> String {
        unsafe{ PROXY_SERVER_ADDR.clone() }
    }

    pub fn set_proxy_server_addr(str:String) {
        unsafe{ PROXY_SERVER_ADDR = str }
    }

}