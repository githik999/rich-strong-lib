use enum_iterator::Sequence;


#[derive(Debug,Clone,Copy,PartialEq,Sequence)]
pub enum LineType {
    Fox,
    Caller,
    Operator,
    Spider,
}

#[derive(Debug,Sequence)]
pub enum LogTag {
    Default,
    Event,
    Establish,
    GoodBye,
    Unexpected,
}
