use enum_iterator::Sequence;
use mio::net::TcpStream;

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd)]
pub enum Status {
    Baby,
    Working,
    Dead,
}

#[derive(Debug,Clone,Copy,PartialEq,Sequence)]
pub enum LineType {
    Fox,
    Caller,
    Operator,
    Spider,
    Http,
    Defalut,
}

#[derive(Debug,Sequence)]
pub enum LogTag {
    Unique,
    Event,
    Establish,
    GoodBye,
    Unexpected,
    Default,
}

#[derive(Debug)]
pub struct  Line {
    id:u64,
    partner_id:u64,
    stream:TcpStream,
    status:Status,
    kind:LineType,
    queue:Vec<u8>,
    stage:u8,
    host:String,
    read_close:bool,
    write_close:bool,
    born:u128,
}