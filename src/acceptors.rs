use std::fmt;
use crate::messages::Message;
use crate::messages::Status;
use std::collections::HashMap;

pub struct Acceptor{
    max_known_id: u32,
    id: u32,
    val: u32,
    status: Status,
    to_send: HashMap<u32, Message>
}

impl Acceptor{

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

    fn check_message(&mut self, message: Message) -> Message{
        match message {
            Message::Prepare(i, _) => {
                if i > self.max_known_id {
                    self.max_known_id = i;
                    self.status = Status::Promised;
                    Message::Promise(i, self.id)
                }
                else{
                    Message::Fail(i, self.id)
                }
            },
            Message::Propose(i,v, _) => {
                if self.max_known_id == i {
                    self.val = v;
                    self.status = Status::Accepted;
                    // Reply Accepted to the proposer
                    // Broadcast Accepted to all
                    Message::Accepted(i, v, self.id)
                }
                else{
                    Message::Fail(i, self.id)
                }
            },
            _ => Message::Error
        }
    }

    pub fn check_buffer(&mut self, buffer: &mut HashMap<u32, Vec<Message>>) {
        println!("Checking Buffer");
        if buffer.contains_key(&self.id) && self.status != Status::Failed {
            let bucket = buffer.get_mut(&self.id).unwrap();
            while bucket.len() > 0 {
                let message = self.check_message(bucket.pop().unwrap());
                match message {
                    Message::Promise(id, _) => self.to_send.insert(id, message),
                    Message::Fail(id, _) => self.to_send.insert(id, message),
                    Message::Accepted(id, _, _) => self.to_send.insert(id, message),
                    _ => panic!("Wrong message created"),
                };
            }
        }
    }

    pub fn send_buffer(&mut self, buffer: &mut HashMap<u32, Vec<Message>>){
        println!("Sending to Buffer");
        if self.status != Status::Failed {
            for (k,v) in self.to_send.drain(){
                // If the messaged is Accepted then broadcast it
                if buffer.contains_key(&k){
                    let bucket = buffer.get_mut(&k).unwrap();
                    bucket.push(v);
                }
            }
        }
    }
}

impl Default for Acceptor{
    fn default() -> Acceptor{
        Acceptor{
            max_known_id: 0,
            id: 0,
            val: 0,
            status: Status::Active,
            to_send: HashMap::new(),
        }
    }
}

impl fmt::Display for Acceptor{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Acceptor => max_known_id: {}, id: {}, val: {}, status: {}", self.max_known_id, self.id, self.val, self.status)
    }

}