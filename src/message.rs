use std::fmt;
use crate::acceptors::Acceptor;
use crate::proposers::Proposer;
pub enum Message<'a>{
    Prepare(u32,&'a mut Proposer),
    Promise(u32,&'a mut Acceptor<'a>),
    Propose(u32,u32,&'a mut Proposer),
    Accepted(u32,u32, &'a mut Acceptor<'a>),
    Fail(u32, &'a mut Acceptor<'a>)
}

impl<'a> fmt::Display for Message<'a>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print = match &self{
            Message::Prepare(id, prop) => format!("Prepare => ID: {}, Sender ID: {}", &id, prop.id()),
            Message::Promise(id, acc) => format!("Promise => ID: {}, Sender ID: {}", &id, acc.id()),
            Message::Fail(id, acc) => format!("Fail => ID: {}, Sender ID: {}", &id, acc.id()),
            Message::Propose(id, val, prop) => format!("Propose => ID: {}, Val: {}, Sender ID: {}", &id, &val, prop.id()),
            Message::Accepted(id, val, acc) => format!("Accepted => ID: {}, Val: {}, Sender ID: {}", &id, &val, acc.id()),
        };
        write!(f, "{}", print)
    }

}
