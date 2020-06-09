use std::fmt;
use crate::message::Message;

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

pub struct Acceptor<'a>{
    max_known_id: u32,
    id: u32,
    val: u32,
    status: Status,
    messages: Vec<Message<'a>>,
}

impl<'a> Acceptor<'a>{

    pub fn max_known_id(&self) -> u32{
        self.max_known_id
    }

    pub fn id(&self) -> u32{
        self.id
    }

    pub fn val(&self) -> u32{
        self.val
    }

    pub fn status(&self) -> &Status{
        &self.status
    }

    pub fn set_max_known_id(&mut self, id: u32){
        self.max_known_id = id;
    }

    pub fn set_id(&mut self, id: u32){
        self.id = id;
    }

    pub fn set_val(&mut self, val: u32){
        self.val = val;
    }

    pub fn set_status(&mut self, status: Status){
        self.status = status;
    }

    pub fn publish_message(&mut self, message: Message<'a>) {
        self.messages.push(message);
    }

    pub fn check_messages(&mut self) {
        println!("Checking messages to see what data we have");
        while self.messages.len() > 0 {
            let msg = match self.messages.pop(){
                Some(t) => t,
                None => panic!("Vector Empty!")
            };
            println!("{}", msg);
            
            match msg {
                Message::Prepare(i,p) => {
                    if i > self.max_known_id {
                        self.max_known_id = i;
                        self.status = Status::Proposed;
                        // Reply Promise to the Proposer
                    }
                    // else reply Fail
                },
                Message::Propose(i,v,p) => {
                    if self.max_known_id == i {
                        self.val = v;
                        self.status = Status::Accepted;
                        // Reply Accepted to the proposer
                        // Broadcast Accpeted to all
                    }
                    // Else reply Fail
                },
                _ => ()
            }
        }
    }
}

impl<'a> Default for Acceptor<'a>{
    fn default() -> Acceptor<'a>{
        Acceptor{
            max_known_id: 0,
            id: 0,
            val: 0,
            status: Status::Active,
            messages: vec![]
        }
    }
}

impl<'a> fmt::Display for Acceptor<'a>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Acceptor => max_known_id: {}, id: {}, val: {}, status: {}", self.max_known_id, self.id, self.val, self.status)
    }

}