use backtrace::Backtrace;
use configparser::ini::Ini;

use crate::log::Log;

pub struct Config;

static mut WRITE_LOG: bool = false;
static mut MININUM_WORKER: u8 = 0;
static mut WORKING_CALLER: u8 = 0;
static mut PROXY_SERVER_ADDR : String = String::new();

impl Config {
    pub fn init() -> String {
        Config::set_panic_hook();
        let r = Config::load();
        let app = r.get("common","app").unwrap();
        let write_log = r.getbool("common", "write_log").unwrap().unwrap();
        if write_log {
            Config::turn_on()
        }
        app
    }

    pub fn init_client_side_setting() {
        let r = Config::load();
        let n:u8 = r.getuint("client","minimum_worker").unwrap().unwrap() as u8;
        let addr = r.get("client","proxy_server_addr").unwrap();
        Config::set_minimum_worker(n);
        Config::set_proxy_server_addr(addr);
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
            Log::heart_beat(format!("{:?}",Backtrace::new()));
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