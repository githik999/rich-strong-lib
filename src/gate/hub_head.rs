use std::collections::HashMap;

use mio::Token;

use super::hub::line_head::Line;

pub struct Hub {
    key:u64,
    m:HashMap<Token,Line>
}



//Set

impl Hub {
    
    pub fn new(key:u64) -> Hub {
        Hub { key, m:HashMap::new() }
    }
}