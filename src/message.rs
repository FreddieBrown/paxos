use std::fmt;
pub enum Message{
    Prepare(u32),
    Promise(u32),
    Propose(u32,u32),
    Accepted(u32,u32),
    Fail(u32)
}

impl fmt::Display for Message{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print = match &self{
            Message::Prepare(id) => format!("Prepare => ID: {}", &id),
            Message::Promise(id) => format!("Promise => ID: {}", &id),
            Message::Fail(id) => format!("Fail => ID: {}", &id),
            Message::Propose(id, val) => format!("Propose => ID: {}, Val: {}", &id, &val),
            Message::Accepted(id, val) => format!("Accepted => ID: {}, Val: {}", &id, &val),
        };
        write!(f, "{}", print)
    }

}
