use std::collections::HashMap;

use mio::Token;

use super::hub::line_head::Line;

pub struct Hub {
    m:HashMap<Token,Line>
}