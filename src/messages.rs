use std::fmt;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum Message{
    Prepare(u32,u32),
    Promise(u32,u32),
    Propose(u32,u32,u32),
    Accepted(u32,u32,u32),
    Fail(u32,u32),
    Error
}

impl fmt::Display for Message{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print = match &self{
            Message::Prepare(id, sid) => format!("Prepare => ID: {}, Sender ID: {}", &id, &sid),
            Message::Promise(id, sid) => format!("Promise => ID: {}, Sender ID: {}", &id, &sid),
            Message::Fail(id, sid) => format!("Fail => ID: {}, Sender ID: {}", &id, &sid),
            Message::Propose(id, val, sid) => format!("Propose => ID: {}, Val: {}, Sender ID: {}", &id, &val, &sid),
            Message::Accepted(id, val, sid) => format!("Accepted => ID: {}, Val: {}, Sender ID: {}", &id, &val, &sid),
            Message::Error => format!("Error")
        };
        write!(f, "{}", print)
    }

}

#[derive(PartialEq, Eq)]
pub enum Status{
    Active,
    Prepared,
    Promised,
    Proposed,
    Accepted,
    Failed
}

impl fmt::Display for Status{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match &self {
            Status::Active => "Active",
            Status::Prepared => "Prepared",
            Status::Promised => "Promised",
            Status::Proposed => "Proposed",
            Status::Accepted => "Accepted",
            Status::Failed => "Failed"
        };
        write!(f, "{}", printable)
    }
}
