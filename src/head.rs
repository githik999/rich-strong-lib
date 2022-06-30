use enum_iterator::Sequence;


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