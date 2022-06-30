use mio::net::TcpListener;

use self::{hub::line_head::LineType, hub_head::Hub};

pub mod hub;
pub mod hub_head;

pub struct Gate {
    listener:TcpListener,
    front_type:LineType,
    pub hub:Hub
}