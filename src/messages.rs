use std::fmt;
use crate::acceptors::Acceptor;
use crate::proposers::Proposer;
pub enum Message{
    Prepare(u32,u32),
    Promise(u32,u32),
    Propose(u32,u32,u32),
    Accepted(u32,u32,u32),
    Fail(u32,u32)
}

impl fmt::Display for Message{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print = match &self{
            Message::Prepare(id, sid) => format!("Prepare => ID: {}, Sender ID: {}", &id, &sid),
            Message::Promise(id, sid) => format!("Promise => ID: {}, Sender ID: {}", &id, &sid),
            Message::Fail(id, sid) => format!("Fail => ID: {}, Sender ID: {}", &id, &sid),
            Message::Propose(id, val, sid) => format!("Propose => ID: {}, Val: {}, Sender ID: {}", &id, &val, &sid),
            Message::Accepted(id, val, sid) => format!("Accepted => ID: {}, Val: {}, Sender ID: {}", &id, &val, &sid),
        };
        write!(f, "{}", print)
    }

}

pub enum Status{
    Active,
    Proposed,
    Accepted,
    Failed
}

impl fmt::Display for Status{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match &self {
            Status::Active => "Active",
            Status::Proposed => "Proposed",
            Status::Accepted => "Accepted",
            Status::Failed => "Failed"
        };
        write!(f, "{}", printable)
    }
}
