use mio::{Poll, Events};

use crate::gate::Gate;

pub struct Server {
    p:Poll,
    events:Events,
    gate:Gate
}