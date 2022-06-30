pub struct Config;

static mut WRITE_LOG: bool = false;
static mut MININUM_WORKER: u8 = 0;
static mut WORKING_CALLER: u8 = 0;
static mut PROXY_SERVER_ADDR : String = String::new();

impl Config {

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

    pub fn proxy_server_addr() -> String {
        unsafe{ PROXY_SERVER_ADDR.clone() }
    }

}