use std::fmt;
use crate::messages::Message;
use crate::messages::Status;
use crate::acceptors::Acceptor;

pub struct Proposer{
    id: u32,
    val: u32,
    num_acceptors: u32,
    status: Status,

}

impl Proposer{

    pub fn num_acceptors(&self) -> u32{
        self.num_acceptors
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

    pub fn set_num_acceptors(&mut self, num: u32){
        self.num_acceptors = num;
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

    pub fn run(&mut self, list: &Vec<Acceptor>){
        // Check status of proposer
            // If status is Active and value > 0 is set, send propose messages out
            // Otherwise, deal with messages in the message queue and perform actions based on them 
    }
}

impl Default for Proposer{
    fn default() -> Proposer{
        Proposer{
            id: 0,
            val: 0,
            num_acceptors: 0,
            status: Status::Active
        }
    }
}

impl fmt::Display for Proposer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Proposer => num_acceptors: {}, id: {}, val: {}, status: {}", self.num_acceptors, self.id, self.val, self.status)
    }

}